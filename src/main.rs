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
    color: String, // Accepts the color as a string in "rrggbb" format
    #[arg(short = 'b', long = "background", default_value = "ffffff")]
    background: String, // Accepts the color as a string in "rrggbb" format
    #[arg(short = 'o', long = "output", default_value = "mandelbrot.png")]
    output: String,
    #[arg(short = 'm', long = "max-iter", default_value_t = 255)]
    max_iter: u32,
    #[arg(short = 'x', long = "center_x", default_value_t = -0.5, value_parser = clap::value_parser!(f64))]
    center_x: f64,
    #[arg(short = 'y', long = "center_y", default_value_t = 0.0, value_parser = clap::value_parser!(f64))]
    center_y: f64,
    #[arg(short = 'z', long = "zoom", default_value_t = 1.0)]
    zoom: f64,
}

fn main() {
    let args = Args::parse();
    let color = parse_color(&args.color);
    let background = parse_color(&args.background);
    let mut img = RgbImage::new(args.width, args.height);

    let scale = 2.0 * (1.0 / args.height as f64) / args.zoom;

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let cx = args.center_x + (x as f64 - args.width as f64 / 2.0) * scale;
        let cy = args.center_y + (y as f64 - args.height as f64 / 2.0) * scale;

        let iter = mandelbrot(cx, cy, args.max_iter);
        let intensity = ((args.max_iter - iter) * 255 / args.max_iter) as u8;
        let color = interpolate_color(background, color, intensity as f64 / 255.0);

        *pixel = color;
    }

    img.save(args.output).expect("Failed to save image");
}
