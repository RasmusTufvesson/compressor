use std::{env::args, fs::File, io::{BufReader, Cursor, Write}, sync::{Arc, Mutex}};
use image::{codecs::{gif::{GifDecoder, GifEncoder}, jpeg::JpegEncoder}, io::Reader, AnimationDecoder, DynamicImage, Frame, ImageBuffer, Rgb};
use indicatif::{ParallelProgressIterator, ProgressBar, ProgressStyle};
use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};

fn main() {
    let mut args = args().skip(1);
    let path = args.next().expect("Supply image path as first command line argument");
    let quality: u8 = args.next().expect("Supply target quality as second command line argument").parse().expect("Could not parse target quality");
    if quality < 1 || quality > 100 {
        panic!("Quality needs to be between 1 and 100");
    }
    if path.ends_with(".gif") {
        let gif_quality: i32 = args.next().expect("Supply target gif quality as third command line argument").parse().expect("Could not parse target gif quality");
        if gif_quality < 1 || gif_quality > 30 {
            panic!("Gif quality needs to be between 1 and 30");
        }
        let file_in = BufReader::new(File::open(path).unwrap());
        let decoder = GifDecoder::new(file_in).unwrap();
        let frames = decoder.into_frames();
        let frames = frames.collect_frames().expect("Error decoding gif");
        let mut out_file = File::create("out.gif").expect("Could not create out file");
        encode_gif(frames, &mut out_file, quality, gif_quality);
    } else {
        let img = Reader::open(path).expect("Could not open file").decode().expect("Could not decode file").to_rgb8();
        let mut out_file = File::create("out.jpg").expect("Could not create out file");
        encode(img, &mut out_file, quality);
    }
}

fn encode(img: ImageBuffer<Rgb<u8>, Vec<u8>>, buffer: &mut impl Write, quality: u8) {
    let encoder = JpegEncoder::new_with_quality(buffer, quality);
    img.write_with_encoder(encoder).expect("Could not encode");
}

fn encode_gif(frames: Vec<Frame>, buffer: &mut impl Write, jpg_quality: u8, gif_quality: i32) {
    let progress_style = ProgressStyle::with_template("{msg} {wide_bar}").unwrap();
    let out_frames = if jpg_quality != 100 {
        let out_frames: Vec<Option<Frame>> = vec![None; frames.len()];
        let out_frames = Arc::new(Mutex::new(out_frames));
        frames.par_iter().enumerate().progress_with(ProgressBar::new(frames.len() as u64).with_style(progress_style.clone()).with_message("Compressing")).for_each(|(i, frame)| {
            let original_buffer: ImageBuffer<Rgb<u8>, Vec<u8>> = DynamicImage::from(frame.clone().into_buffer()).into();
            let mut frame_buffer = vec![];
            encode(original_buffer, &mut frame_buffer, jpg_quality);
            out_frames.lock().unwrap()[i] = Some(Frame::from_parts(Reader::with_format(Cursor::new(frame_buffer), image::ImageFormat::Jpeg).decode().unwrap().into(), frame.left(), frame.top(), frame.delay()));
        });
        let out_frames = out_frames.lock().unwrap().clone();
        out_frames
    } else {
        frames.iter().map(|x| Some(x.clone())).collect()
    };
    let mut encoder = GifEncoder::new_with_speed(buffer, gif_quality);
    encoder.set_repeat(image::codecs::gif::Repeat::Infinite).unwrap();
    let progress = ProgressBar::new(frames.len() as u64).with_style(progress_style).with_message("Encoding gif");
    for frame in out_frames {
        progress.inc(1);
        encoder.encode_frame(frame.unwrap()).expect("Could not encode");
    }
}