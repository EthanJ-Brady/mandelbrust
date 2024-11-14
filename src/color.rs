use image::Rgb;

pub fn parse_color(hex: &str) -> Rgb<u8> {
    let r = u8::from_str_radix(&hex[0..2], 16).expect("Invalid red component");
    let g = u8::from_str_radix(&hex[2..4], 16).expect("Invalid green component");
    let b = u8::from_str_radix(&hex[4..6], 16).expect("Invalid blue component");
    Rgb([r, g, b])
}
