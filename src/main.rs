use caesium::compress;
use caesium::parameters::CSParameters;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let mut parameters = CSParameters::new();

    parameters.gif.quality = 100;
    parameters.png.optimize = true;
    parameters.jpeg.optimize = true;
    parameters.webp.lossless = true;

    let files = fs::read_dir("./")?;

    let images = files.filter_map(|file| match file {
        Ok(file) => {
            let path = file.path();

            if path.is_file() {
                let valid = path.extension().map_or(false, |ext| {
                    ext.eq_ignore_ascii_case("jpg")
                        || ext.eq_ignore_ascii_case("jpeg")
                        || ext.eq_ignore_ascii_case("png")
                        || ext.eq_ignore_ascii_case("gif")
                        || ext.eq_ignore_ascii_case("webp")
                        || ext.eq_ignore_ascii_case("tiff")
                });

                if valid {
                    return Some(file);
                }
            }

            None
        }
        Err(_) => None,
    });

    for file in images {
        let name = file.file_name().display().to_string();
        let target = file.path().to_string_lossy().to_string();
        let input = target.clone();
        let output = target.clone();

        compress(input, output, &parameters)?;

        println!("Compressed: {}", name);
    }

    Ok(())
}
