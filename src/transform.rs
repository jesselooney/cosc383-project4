use crate::bit_patterns::{patterns, transject};
/// This file contains functions which will perform useful
/// transformations on images.
use bitvec::prelude::*;
use image::{GenericImageView, ImageBuffer, Pixel, RgbImage};

/// This function amplifies the least significant bit of
/// each channel so that hidden changes become more visible
/// to the human eye.
fn amplify_least_significant_bits(mut image: RgbImage) -> RgbImage {
    //let least_significant_bits = 
    image
}

#[cfg(test)]
mod tests {
    #[test]
    fn my_epic_test() {
        assert!(true)
    }
}