use std::path::Path;

pub fn get_filename(dir: &str, name: &str, width: u32, height: u32) -> String {
    let filename = format!("{}-{}x{}.png", name, width, height);
    Path::new(dir).join(filename).to_string_lossy().to_string()
}
