use crate::bit_patterns::{eject, inject, patterns};
/// This file contains functions which will perform useful
/// transformations on images.
use bitvec::prelude::*;
use image::RgbImage;

/// This function amplifies the least significant bit of
/// each channel so that hidden changes become more visible
/// to the human eye.
pub fn amplify_least_significant_bits(image: RgbImage) -> RgbImage {
    let least_significant_bits =
        eject(image.clone(), patterns::access_least_significant_bits, None);

    let mut transformed_image_bits = bitvec!();

    for bit in least_significant_bits {
        transformed_image_bits.extend([bit; 8]);
    }

    inject(image, patterns::access_all, transformed_image_bits)
}

#[cfg(test)]
mod tests {
    #[test]
    fn my_epic_test() {
        assert!(true)
    }
}
