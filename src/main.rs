mod decode;
mod extensions;
mod extract;
mod helpers;

use crate::extract::patterns;
use crate::helpers::*;
use anyhow::Result;
use bitvec::prelude::*;
use image::*;

fn main() -> Result<()> {
    // Amplify all the source images
    // write_amplified_dir("assets/sources/", "assets/amplified/")?;

    // Investigate a specific image
    // write_amplified_images("assets/working/Phishing/Phishing.png")?;
    // write_extracted_image(
    //     "assets/working/Phishing/Phishing.png",
    //     patterns::access_index(0),
    // )?;
    // write_amplified_images("assets/working/Phishing/Phishing-extract.png")?;

    Ok(())
}
