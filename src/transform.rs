use crate::bit_patterns::{eject, inject, patterns};
use anyhow::Result;
use bitvec::prelude::*;
use image::{ImageReader, RgbImage};
use std::{fs, path::PathBuf};

/// This file contains functions which will perform useful
/// transformations on images and binary data.

/// This function amplifies the least significant bit of
/// each channel so that hidden changes become more visible
/// to the human eye.
pub fn amplify_least_significant_bit(image: RgbImage) -> RgbImage {
    let least_significant_bits =
        eject(image.clone(), patterns::access_least_significant_bits, None);

    let mut transformed_image_bits: BitVec<u8> = BitVec::new();

    for bit in least_significant_bits {
        transformed_image_bits.extend([bit; 8]);
    }

    inject(image, patterns::access_all, transformed_image_bits)
}

/// Amplifies the bit right before the least significant bit of each channel.
pub fn amplify_index_one_bit(image: RgbImage) -> RgbImage {
    // FIXME: this function repeats a lot of code from `amplify_least_significant_bit`
    // I tried to reduce this by writing a generic version of amplify_least_significant_bit
    // that takes an index, but this would require a closure, and that would require breaking the interface for
    // `eject`, and also i dont feel like dealing with trait objects right now

    let least_significant_bits = eject(image.clone(), patterns::access_index_one, None);

    let mut transformed_image_bits: BitVec<u8> = BitVec::new();

    for bit in least_significant_bits {
        transformed_image_bits.extend([bit; 8]);
    }

    inject(image, patterns::access_all, transformed_image_bits)
}

// FIXME: this fails with an unclear error when presented with certain inputs, investigate why
// (i think it has to do with when the input isn't divisible by 8)
pub fn flipsy_flipsy(mut input: BitVec<u8>) -> BitVec<u8> {
    let mut result: BitVec<u8> = BitVec::new();

    for i in 0..input.len() - 1 {
        if i % 8 == 0 {
            let mut tmp = input.drain(0..8).collect::<BitVec>();
            tmp.reverse();
            result.extend(tmp);
        }
    }

    result
}

/// amplify all of the least significant bits of the given images
fn amplify_all() -> Result<()> {
    // yes i know this function is horrifically impure deal with it
    for path in fs::read_dir("./assets/project-images")? {
        let path = path?.path();

        if path.is_dir() {
            continue;
        }

        println!("Transforming: {:?}", path);
        let image = ImageReader::open(path.clone())?.decode()?;

        let modified_img = amplify_least_significant_bit(image.into());

        let mut result_path = PathBuf::from("./assets/project-images/transformed");
        let file_name = path.strip_prefix("./assets/project-images/")?;
        result_path.push(file_name);

        modified_img.save(result_path)?;
    }
    Ok(())
}
