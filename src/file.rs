use std::path::Path;

pub fn get_filename(output: &str, fractal_name: String, width: u32, height: u32) -> String {
    let filename = format!("{}-{}x{}.png", fractal_name, width, height);
    let output_path = Path::new(output);

    if output_path.is_dir() {
        output_path.join(filename).to_string_lossy().to_string()
    } else {
        output_path.to_string_lossy().to_string()
    }
}
