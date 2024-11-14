use clap::Parser;
use image::{Rgb, RgbImage};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short='W', long="width", default_value_t = 1920)]
    width: u32,
    #[arg(short='H', long="height", default_value_t = 1080)]
    height: u32
}

fn main() {
    let args = Args::parse();

    // Create a new image buffer with the specified dimensions
    let mut img = RgbImage::new(args.width, args.height);

    // Define the color you want to fill, in this case, solid red
    let color = Rgb([255, 0, 0]); // RGB for red

    // Fill the image with the color
    for pixel in img.pixels_mut() {
        *pixel = color;
    }

    // Save the image to a file
    img.save("solid_color_image.png").expect("Failed to save image");
}
