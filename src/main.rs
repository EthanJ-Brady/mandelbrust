mod color;
mod mandelbrot;

use clap::Parser;
use color::{parse_color, interpolate_color};
use image::RgbImage;
use mandelbrot::mandelbrot;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short = 'W', long = "width", default_value_t = 1920)]
    width: u32,
    #[arg(short = 'H', long = "height", default_value_t = 1080)]
    height: u32,
    #[arg(short = 'C', long = "color", default_value = "000000")]
    color: String,  // Accepts the color as a string in "rrggbb" format
    #[arg(short = 'b', long = "background", default_value = "ffffff")]
    background: String,  // Accepts the background color as a string in "rrggbb" format
    #[arg(short = 'o', long = "output", default_value = "mandelbrot.png")]
    output: String,
    #[arg(short = 'm', long = "max-iter", default_value_t = 255)]
    max_iter: u32,
}

fn main() {
    let args = Args::parse();
    let color = parse_color(&args.color);
    let background = parse_color(&args.background);
    let mut img = RgbImage::new(args.width, args.height);

    let scale_x = 3.5 / args.width as f64;  // Scale width to the Mandelbrot range
    let scale_y = 2.0 / args.height as f64; // Scale height to the Mandelbrot range

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        // Map pixel coordinates to complex plane coordinates
        let cx = x as f64 * scale_x - 2.5;  // Shift to center Mandelbrot set
        let cy = y as f64 * scale_y - 1.0;

        // Get Mandelbrot iteration count
        let iter = mandelbrot(cx, cy, args.max_iter);

        // Map iteration count to color intensity
        let intensity = ((args.max_iter - iter) * 255 / args.max_iter) as u8;
        let color = interpolate_color(background, color, intensity as f64 / 255.0);

        *pixel = color;
    }

    img.save(args.output).expect("Failed to save image");
}
