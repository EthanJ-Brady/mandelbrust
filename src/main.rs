mod color;

use clap::Parser;
use color::parse_color;
use image::{Rgb, RgbImage};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short = 'W', long = "width", default_value_t = 1920)]
    width: u32,
    #[arg(short = 'H', long = "height", default_value_t = 1080)]
    height: u32,
    #[arg(short = 'C', long = "color", default_value = "ff0000")]
    color: String,  // Accepts the color as a string in "rrggbb" format
    #[arg(short = 'o', long = "output", default_value = "mandelbrot.png")]
    output: String,
}

fn mandelbrot(cx: f64, cy: f64, max_iter: u32) -> u32 {
    let mut x = 0.0;
    let mut y = 0.0;
    let mut iteration = 0;

    while x * x + y * y <= 4.0 && iteration < max_iter {
        let x_temp = x * x - y * y + cx;
        y = 2.0 * x * y + cy;
        x = x_temp;

        iteration += 1;
    }

    iteration
}

fn main() {
    let args = Args::parse();
    let color = parse_color(&args.color);
    let mut img = RgbImage::new(args.width, args.height);

    let max_iter = 255;
    let scale_x = 3.5 / args.width as f64;  // Scale width to the Mandelbrot range
    let scale_y = 2.0 / args.height as f64; // Scale height to the Mandelbrot range

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        // Map pixel coordinates to complex plane coordinates
        let cx = x as f64 * scale_x - 2.5;  // Shift to center Mandelbrot set
        let cy = y as f64 * scale_y - 1.0;

        // Get Mandelbrot iteration count
        let iter = mandelbrot(cx, cy, max_iter);

        // Map iteration count to color intensity
        let intensity = (255 - iter as u8) % 255;
        *pixel = Rgb([intensity, intensity, intensity]);
    }

    img.save(args.output).expect("Failed to save image");
}
