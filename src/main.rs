mod color;

use clap::Parser;
use color::parse_color;
use image::{RgbImage};


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short='W', long="width", default_value_t = 1920)]
    width: u32,
    #[arg(short='H', long="height", default_value_t = 1080)]
    height: u32,
    #[arg(short='C', long="color", default_value = "ff0000")]
    color: String,  // Accepts the color as a string in "rrggbb" format
    #[arg(short='o', long="output", default_value = "mandelbrot.png")]
    output: String,
}

fn main() {
    let args = Args::parse();
    let color = parse_color(&args.color);
    let mut img = RgbImage::new(args.width, args.height);

    for pixel in img.pixels_mut() {
        *pixel = color;
    }

    img.save(args.output).expect("Failed to save image");
}
