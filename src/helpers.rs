use anyhow::Result;
use bitvec::prelude::*;
use image::*;
use std::fs;
use std::path::Path;

// A hack to effectively alias a trait, taken from https://stackoverflow.com/a/57937836.
pub trait Pattern: Fn(u32, u32, usize, usize) -> bool {}
impl<T: Fn(u32, u32, usize, usize) -> bool> Pattern for T {}

mod patterns {
    use super::Pattern;
    pub fn access_index(index: usize) -> impl Pattern {
        // `move` forces the closure to take ownership of `index` so that it can be returned.
        move |_, _, _, idx| idx == index
    }

    pub fn access_all() -> impl Pattern {
        |_, _, _, _| true
    }
}

pub fn extract(image: &RgbImage, pattern: impl Pattern) -> BitVec<u8, Msb0> {
    let mut message: BitVec<u8, Msb0> = BitVec::new();
    for (pixel_row, pixel_col, pixel) in image.enumerate_pixels() {
        for (channel_index, channel_value) in pixel.channels().iter().enumerate() {
            // Consider the least significant bit to have index zero, but iterate in reverse so
            // that we add the most significant bit to the message first, consistent with the
            // message having Msb0 ordering.
            let bits = channel_value.view_bits::<Lsb0>();
            for (bit_index, bit_value) in bits.iter().enumerate().rev() {
                if pattern(pixel_row, pixel_col, channel_index, bit_index) {
                    message.push(*bit_value);
                }
            }
        }
    }
    message
}

/// Helper function to write each byte of BitVec to a file. Watch out for the bit order.
pub fn write_raw<O: BitOrder>(path: impl AsRef<Path>, bv: BitVec<u8, O>) -> Result<()> {
    fs::write(path, bv.as_raw_slice())?;
    Ok(())
}
/*
pub fn write_image<O: BitOrder>(
    path: impl AsRef<Path>,
    bv: BitVec<u8, O>,
    width: u32,
    height: u32,
) -> Result<()> {
    let image = PngEncoder::new(File::create(path)?);
    image.write_image(
        bv.as_raw_slice(),
        width,
        height,
        image::ExtendedColorType::Rgb8,
    )?;
    Ok(())
}
*/

pub fn amplify_bits<O: BitOrder>(bits: &BitVec<u8, O>) -> BitVec<u8, O> {
    let mut amplified_bits: BitVec<u8, O> = BitVec::new();
    for bit in bits {
        amplified_bits.extend([*bit; 8]);
    }
    amplified_bits
}

pub fn amplify_image(image: &RgbImage, index: usize) -> RgbImage {
    let bits = extract(image, |_, _, _, idx| idx == index);
    let amplified_bits = amplify_bits(&bits);
    let amplified_image: RgbImage = RgbImage::from_raw(
        image.width(),
        image.height(),
        amplified_bits.as_raw_slice().to_vec(),
    )
    .expect("The container of amplified bytes should be right-sized.");
    amplified_image
}
