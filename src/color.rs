use image::Rgb;

pub fn parse_color(hex: &str) -> Rgb<u8> {
    let r = u8::from_str_radix(&hex[0..2], 16).expect("Invalid red component");
    let g = u8::from_str_radix(&hex[2..4], 16).expect("Invalid green component");
    let b = u8::from_str_radix(&hex[4..6], 16).expect("Invalid blue component");
    Rgb([r, g, b])
}

pub fn interpolate_color(color1: Rgb<u8>, color2: Rgb<u8>, t: f64) -> Rgb<u8> {
    let r = (color1.0[0] as f64 * t + color2.0[0] as f64 * (1.0 - t)) as u8;
    let g = (color1.0[1] as f64 * t + color2.0[1] as f64 * (1.0 - t)) as u8;
    let b = (color1.0[2] as f64 * t + color2.0[2] as f64 * (1.0 - t)) as u8;
    Rgb([r, g, b])
}
