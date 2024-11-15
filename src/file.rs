use crate::args::Args;
use std::path::Path;

pub fn get_filename(args: &Args) -> String {
    let filename = args.output.clone().unwrap_or_else(|| {
        format!(
            "{}-x={:.2}-y={:.2}-z={:.2}-{}x{}.png",
            args.fractal, args.center_x, args.center_y, args.zoom, args.width, args.height
        )
    });

    Path::new(&args.directory)
        .join(filename)
        .to_string_lossy()
        .to_string()
}
