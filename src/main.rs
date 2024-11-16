mod args;
mod color;
mod file;
mod fractal;
mod fractal_template;

use args::Args;
use clap::Parser;
use color::{interpolate_color, parse_color};
use file::get_filename;
use fractal::{burning_ship, mandelbrot};
use fractal_template::{get_fractal_template, FractalTemplate};
use image::RgbImage;

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

fn main() {
    let args = Args::parse();
    args.validate();

    let color = parse_color(&args.color);
    let background = parse_color(&args.background);

    let mut ft = FractalTemplate {
        fractal: args.fractal.clone(),
        center_x: args.center_x,
        center_y: args.center_y,
        zoom: args.zoom,
        max_iter: args.max_iter,
    };
    let mut name = args.fractal.clone();
    if let Some(template) = &args.template {
        ft = get_fractal_template(template).unwrap()[0].clone();
        name = args.template.clone().unwrap();
    }
    let fractal_func = select_fractal_function(&ft.fractal);

    let mut img = RgbImage::new(args.width, args.height);
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let cx =
            ft.center_x + (2.0 * x as f64 - args.width as f64) / (args.height as f64 * ft.zoom);
        let cy =
            ft.center_y + (2.0 * y as f64 - args.height as f64) / (args.height as f64 * ft.zoom);

        let iter = fractal_func(cx, cy, ft.max_iter);
        let intensity = ((ft.max_iter - iter) * 255 / ft.max_iter) as u8;
        let color = interpolate_color(background, color, intensity as f64 / 255.0);

        *pixel = color;
    }

    let file_path = get_filename(&args.directory, name, args.width, args.height);
    img.save(file_path).expect("Failed to save image");
}
