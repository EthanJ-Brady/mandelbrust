mod args;
mod color;
mod file;
mod fractal;
mod template;

use args::Args;
use clap::Parser;
use color::{interpolate_color, parse_color};
use file::get_filename;
use fractal::{burning_ship, mandelbrot};
use image::RgbImage;
use std::fs;
use template::FractalTemplates;

fn select_fractal_function(fractal_name: &str) -> fn(f64, f64, u32) -> u32 {
    match fractal_name {
        "mandelbrot" => mandelbrot,
        "burning_ship" => burning_ship,
        _ => {
            eprintln!(
                "Unknown fractal type: {}. Using mandelbrot by default.",
                fractal_name
            );
            mandelbrot
        }
    }
}

fn read() -> Result<FractalTemplates, Box<dyn std::error::Error>> {
    let toml_content = fs::read_to_string("templates.toml")?;
    let templates: FractalTemplates = toml::from_str(&toml_content)?;
    Ok(templates)
}

fn main() {
    let args = Args::parse();
    let color = parse_color(&args.color);
    let background = parse_color(&args.background);
    let mut img = RgbImage::new(args.width, args.height);

    let templates = match read() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Error reading objects.toml: {}", e);
            return;
        }
    };
    println!("Templates: {:?}", templates.get("mandelbrot"));

    let fractal_func = select_fractal_function(&args.fractal);

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let cx =
            args.center_x + (2.0 * x as f64 - args.width as f64) / (args.height as f64 * args.zoom);
        let cy = args.center_y
            + (2.0 * y as f64 - args.height as f64) / (args.height as f64 * args.zoom);

        let iter = fractal_func(cx, cy, args.max_iter);
        let intensity = ((args.max_iter - iter) * 255 / args.max_iter) as u8;
        let color = interpolate_color(background, color, intensity as f64 / 255.0);

        *pixel = color;
    }

    img.save(get_filename(&args)).expect("Failed to save image");
}
