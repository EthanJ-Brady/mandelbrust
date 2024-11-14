use image::{Rgb, RgbImage};

fn main() {
    // Define image dimensions
    let width = 256;
    let height = 256;

    // Create a new image buffer with the specified dimensions
    let mut img = RgbImage::new(width, height);

    // Define the color you want to fill, in this case, solid red
    let color = Rgb([255, 0, 0]); // RGB for red

    // Fill the image with the color
    for pixel in img.pixels_mut() {
        *pixel = color;
    }

    // Save the image to a file
    img.save("solid_color_image.png").expect("Failed to save image");
}
