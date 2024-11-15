use crate::extensions::RgbImageExt;
use bitvec::prelude::*;
use image::*;

// A hack to effectively alias a trait, taken from https://stackoverflow.com/a/57937836.
pub trait Pattern: Fn(u32, u32, usize, usize) -> bool {}
impl<T: Fn(u32, u32, usize, usize) -> bool> Pattern for T {}

pub mod patterns {
    use super::Pattern;
    pub fn access_index(index: usize) -> impl Pattern {
        // `move` forces the closure to take ownership of `index` so that it can be returned.
        move |_, _, _, idx| idx == index
    }

    pub fn access_all() -> impl Pattern {
        |_, _, _, _| true
    }
}

pub fn extract_bits(image: &RgbImage, pattern: impl Pattern) -> BitVec<u8, Lsb0> {
    let mut message: BitVec<u8, Lsb0> = BitVec::new();
    for (pixel_row, pixel_col, pixel) in image.enumerate_pixels() {
        for (channel_index, channel_value) in pixel.channels().iter().enumerate() {
            // Consider the least significant bit to have index zero, but iterate in reverse so
            // that we add the most significant bit to the message first, consistent with the
            // message having Msb0 ordering.
            let bits = channel_value.view_bits::<Lsb0>();
            for (bit_index, bit_value) in bits.iter().enumerate() {
                if pattern(pixel_row, pixel_col, channel_index, bit_index) {
                    message.push(*bit_value);
                }
            }
        }
    }
    message
}

pub fn extract_image(image: &RgbImage, pattern: impl Pattern) -> RgbImage {
    let bits = extract_bits(image, pattern);

    // Slightly hacky way of reading the header in one bit order while reading the image in the
    // opposite order.
    let mut header = bits[0..64].to_bitvec();
    header.reverse();
    let width = header[0..32].load_le::<u32>();
    let height = header[32..64].load_le::<u32>();

    println!("{} x {}", width, height);

    let bit_len = (width * height * 3 * 8) as usize;

    let data: BitVec<u8, Lsb0> = bits[64..(bit_len + 64)].to_bitvec();

    RgbImage::from_bitvec(width, height, data)
}
