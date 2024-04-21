# Compressor
This is a very simple program that takes an image and converts it to a jpg with a specified quality.

## How to use
This is a command line program so call it from there. There are two arguments, the path to an image and the target output quality. The output quality is a number between 1 and 100 where the higher the number the higher the quality. When it is done the program will output to a file called `out.jpg`. You can also provide the program with a gif file and a third argument and the program will compress the gif and output it into a file called `out.gif`. The third argument is supposed to be between 1 and 30 where the lower the number the higher the quality. One thing to note is that setting the quality (not the gif quality) when compressing a gif will not necessarily make the file smaller.

Here are a few example calls:

`compressor.exe "C:/path/to/image.png" 10`
`compressor.exe "C:/path/to/image.png" 50`
`compressor.exe "C:/path/to/gif.gif" 100 30`
`compressor.exe "C:/path/to/gif.gif" 1 1`