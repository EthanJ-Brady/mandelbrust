use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short = 'W', long = "width", default_value_t = 1920)]
    pub width: u32,
    #[arg(short = 'H', long = "height", default_value_t = 1080)]
    pub height: u32,
    #[arg(short = 'C', long = "color", default_value = "000000")]
    pub color: String, // Accepts the color as a string in "rrggbb" format
    #[arg(short = 'b', long = "background", default_value = "ffffff")]
    pub background: String, // Accepts the color as a string in "rrggbb" format
    #[arg(short = 'o', long = "output")]
    pub output: Option<String>,
    #[arg(short = 'd', long = "directory", default_value = ".")]
    pub directory: String,
    #[arg(short = 'm', long = "max-iter", default_value_t = 255)]
    pub max_iter: u32,
    #[arg(short = 'f', long = "fractal", default_value = "mandelbrot")]
    pub fractal: String,
    #[arg(short = 'x', long = "center_x", default_value_t = -0.5, value_parser = clap::value_parser!(f64))]
    pub center_x: f64,
    #[arg(short = 'y', long = "center_y", default_value_t = 0.0, value_parser = clap::value_parser!(f64))]
    pub center_y: f64,
    #[arg(short = 'z', long = "zoom", default_value_t = 1.0)]
    pub zoom: f64,
    #[arg(short = 't', long = "template")]
    pub template: Option<String>,
}

impl Args {
    pub fn validate(&self) {
        if let Some(_template) = &self.template {
            let conflict_args = vec![
                ("max_iter", self.max_iter != 255),
                ("fractal", self.fractal != "mandelbrot"),
                ("center_x", self.center_x != -0.5),
                ("center_y", self.center_y != 0.0),
                ("zoom", self.zoom != 1.0),
            ]
            .into_iter()
            .filter(|(_, is_present)| *is_present)
            .map(|(name, _)| name)
            .collect::<Vec<_>>();

            if !conflict_args.is_empty() {
                eprintln!(
                    "Error: '--template' cannot be used with the following options: {:?}",
                    conflict_args
                );
                std::process::exit(1);
            }
        }
    }
}
