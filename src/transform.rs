use crate::bit_patterns::{eject, inject, patterns};
use bitvec::prelude::*;
use image::RgbImage;

/// This file contains functions which will perform useful
/// transformations on images and binary data.

/// This function amplifies the least significant bit of
/// each channel so that hidden changes become more visible
/// to the human eye.
pub fn amplify_least_significant_bits(image: RgbImage) -> RgbImage {
    let least_significant_bits =
        eject(image.clone(), patterns::access_least_significant_bits, None);

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
