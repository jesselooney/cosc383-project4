use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;
mod bit_patterns;
mod detect;
mod transform;
use anyhow::Result;
use bit_patterns::{eject, patterns};
use bitvec::field::BitField;
use image::{ImageReader, RgbImage};

/// Here's a list of files that might have information hidden in them
/// - Myself.png
/// - RobotOnRealCat.png
/// - Soccer.png
/// - Steganography.png (data appears to be on the left side, maybe encoded top to bottom instead of left to right)
/// - Teach.png
/// - TouchingGrass.png
/// - 383.png
/// - Abominable.png
/// - BackdoorAtks.png
/// - BombAnswers.png
/// - Cookies.png
/// - Dance.png
/// - Dream.png
/// - Ideal.png (maybe, might just be weird artifacting)
/// - Lockpicking.png
/// - Lucy.png (maybe, might just be artifacting)
fn main() -> Result<()> {
    // This is all scratch work to extract stuff out of the examples.
    let img: RgbImage = image::open("assets/hide_text.png").unwrap().into();
    let mut length_bits = eject(
        img.clone(),
        patterns::access_least_significant_bits,
        Some(32),
    );
    length_bits.reverse(); // no clue why i have to do this
    println!("{:?}", length_bits);
    let length = length_bits.load::<usize>() * 8 + 32;
    println!("string length: {length}");

    // amplify all of the least significant bits of the given images
    for path in fs::read_dir("./assets/project-images")? {
        let path = path?.path();

        if path.is_dir() {
            continue;
        }

        println!("Transforming: {:?}", path);
        let image = ImageReader::open(path.clone())?.decode()?;

        let modified_img = transform::amplify_least_significant_bits(image.into());

        let mut result_path = PathBuf::from("./assets/project-images/transformed");
        let file_name = path.strip_prefix("./assets/project-images/")?;
        result_path.push(file_name);

        modified_img.save(result_path)?;
    }

    Ok(())
}
