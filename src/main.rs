mod assemble;
mod bit_patterns;
mod decode;
mod detect;
mod helpers;
mod transform;

use std::fs;

use anyhow::Result;
use image::RgbImage;
use transform::amplify_image;

fn amplify_all() -> Result<()> {
    let dir_entries = fs::read_dir("assets/sources/")?;
    for dir_entry in dir_entries {
        let path = dir_entry?.path();
        let source_file_stem = path.file_stem().unwrap().to_str().unwrap();
        println!("Amplifying {}:", source_file_stem);
        let source_image: RgbImage = image::open(&path)?.into();
        for bit_index in 0..8 {
            let amplified_file_name = format!("{}-amp{}.png", source_file_stem, bit_index);
            let amplified_image = amplify_image(&source_image, bit_index);
            amplified_image.save(&amplified_file_name)?;
            println!("\tWrote {}", amplified_file_name);
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    //transform::amplify_all()?;
    //decode::decode_myself()?;

    //decode::three_eight_three()?;

    Ok(())
}
