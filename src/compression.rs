use caesium::compress;
use caesium::parameters::CSParameters;
use std::error::Error;
use std::path::PathBuf;

use crate::types::PathFormatter;

pub const VALID_EXTENSIONS: [&str; 6] = ["jpg", "jpeg", "png", "gif", "webp", "tiff"];

/// In-place compression.
pub fn compress_image(path: PathBuf) -> Result<u64, Box<dyn Error>> {
    let mut parameters = CSParameters::new();

    parameters.gif.quality = 100;
    parameters.png.optimize = true;
    parameters.jpeg.optimize = true;
    parameters.webp.lossless = true;

    let target = path.to_string_lossy().to_string();
    let input = target.clone();
    let output = target.clone();

    compress(input, output, &parameters)?;

    let bytes = PathFormatter::new(path.to_owned()).file_bytes();

    Ok(bytes)
}
