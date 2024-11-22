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
    /*
        let image: RgbImage = image::open("Acorn-extract.png")?.into();

        let bits = extract_bits_with_order_count(
            &image,
            &IterationOrder::new(Forward, Forward, [0, 1, 2], [0], [0, 1, 2, 3]),
            Some(32 + 569 * 8),
        );
        // read 32 bits, reverse, load LE
        let len1 = bits[0..32].to_bitvec();
        println!("{}", len1);
        let mut len2 = len1.clone();
        len2.reverse();
        println!("{} {}", len1.load_be::<u32>(), len2.load_le::<u32>());
        let mut message_bits = bits[32..].to_bitvec();
        message_bits.chunks_exact_mut(8).for_each(|bs| bs.reverse());
        fs::write("test2.bytes", message_bits.as_raw_slice())?;
    */

    let dir_entries = fs::read_dir("tests")?;

    for dir_entry in dir_entries {
        let image_path = dir_entry?.path();
        if !image_path.is_file() {
            continue;
        }
        println!("=== Working on {}", image_path.display());

        let image_name = image_path
            .file_name()
            .expect("`image_path` should have a file name");

        let image: RgbImage = image::open(&image_path)?.into();
        try_extraction_orders(
            &image,
            format!("tests_out/{}-", image_name.to_str().unwrap()).as_str(),
        )?;
    }

    Ok(())
}
