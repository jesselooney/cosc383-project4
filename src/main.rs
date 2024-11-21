mod decode;
mod extensions;
mod extract;
mod helpers;
mod iteration_order;
use crate::iteration_order::{IterationOrder, Order::Forward};

use std::fs;

use crate::extensions::*;
use crate::extract::*;
use crate::helpers::*;
use anyhow::Result;
use bitvec::field::BitField;
use bitvec::prelude::*;
use image::*;
use itertools::iproduct;
use itertools::Itertools;

fn find_extractions(image: RgbImage) {
    let row_indices = 0..image.width();
    let col_indices = 0..image.height();
    let channel_iterators = (0..=2).permutations(3);
    let bit_indices = 0..8;
}

fn main() -> Result<()> {
    // Amplify all the source images
    // write_amplified_dir("assets/sources/", "assets/amplified/")?;

    // Investigate a specific image
    /*
    write_amplified_images("assets/working/Phishing/Phishing.png")?;
    write_extracted_image(
        "assets/working/Phishing/Phishing.png",
        patterns::access_index(0),
    )?;
    write_amplified_images("assets/working/Phishing/Phishing-extract.png")?;

    write_extracted_bytes(
        "assets/working/Phishing/Phishing-extract.png",
        patterns::access_index(0),
    )?;

    let mut bits = extract_bits(
        &RgbImage::open("assets/working/Phishing/Phishing-extract.png")?,
        patterns::access_index(0),
    );

    println!("{}", &bits[0..16]);
    bits.chunks_exact_mut(8).for_each(|bs| bs.reverse());
    println!("{}", &bits[0..16]);
    fs::write(
        "assets/working/Phishing/Phishing-extract-extract_rev.bin",
        bits.as_raw_slice(),
    )?;
    */

    //write_extracted_image("assets/working/Acorn/Acorn.png", patterns::access_index(2))?;
    //write_amplified_images("assets/working/Acorn/Acorn-extract.png")?;
    /*
        write_extracted_bytes(
            "assets/working/Acorn/Acorn-extract.png",
            patterns::access_index(2),
        )?;
    */
    /*
    write_extracted_image(
        "assets/working/Abominable/Abominable.png",
        |_, _, chn, idx| (chn == 0) && (idx == 0),
    )?;*/

    let image: RgbImage = image::open("assets/working/Phishing/Phishing.png")?.into();
    /*let image2 = extract_image_with_order(
        &image,
        &IterationOrder::new(Forward, Forward, [0, 1, 2], [0], [1, 0, 2, 3]),
    )?;
    image2.save("test2.png")?;*/

    try_extraction_orders(&image)?;

    Ok(())
}
