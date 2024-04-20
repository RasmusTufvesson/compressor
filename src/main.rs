use std::{env::args, fs::File};
use image::{codecs::jpeg::JpegEncoder, io::Reader};

fn main() {
    let mut args = args().skip(1);
    let path = args.next().expect("Supply image path as first command line argument");
    let quality: u8 = args.next().expect("Supply target quality as second command line argument").parse().expect("Could not parse target quality");
    if quality < 1 || quality > 100 {
        panic!("Quality needs to be between 1 and 100");
    }
    let img = Reader::open(path).expect("Could not open file").decode().expect("Could not decode file").to_rgb8();
    let mut out_file = File::create("out.jpg").expect("Could not create out file");
    let encoder = JpegEncoder::new_with_quality(&mut out_file, quality);
    img.write_with_encoder(encoder).expect("Could not encode");
}
