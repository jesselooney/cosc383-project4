use crate::extensions::RgbImageExt;
use crate::iteration_order::IterationOrder;
use anyhow::{anyhow, Result};
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

// ---- Old functions ---
pub fn extract_bits(image: &RgbImage, pattern: impl Pattern) -> BitVec<u8, Lsb0> {
    let mut message: BitVec<u8, Lsb0> = BitVec::new();
    let mut x = 0;
    for (pixel_row, pixel_col, pixel) in image.enumerate_pixels() {
        for (channel_index, channel_value) in pixel.channels().iter().enumerate() {
            let bits = channel_value.view_bits::<Lsb0>();
            for (bit_index, bit_value) in bits.iter().enumerate() {
                if x < 8 {
                    println!(
                        "row: {}; col: {}; chn: {}; idx: {}",
                        pixel_row, pixel_col, channel_index, bit_index
                    );
                }
                if pattern(pixel_row, pixel_col, channel_index, bit_index) {
                    if x < 8 {
                        println!("pattern matched!");
                    }
                    message.push(*bit_value);
                    x += 1;
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

// --- New functions ---
pub fn extract_bits_with_pattern_order(
    image: &RgbImage,
    pattern: impl Pattern,
    order: &IterationOrder,
) -> BitVec<u8> {
    extract_bits_with_pattern_order_count(image, pattern, order, None)
}

pub fn extract_bits_with_pattern_order_count(
    image: &RgbImage,
    pattern: impl Pattern,
    order: &IterationOrder,
    max_length: Option<usize>,
) -> BitVec<u8> {
    let mut bits = BitVec::new();
    for (row, col, channel, bit_index) in order.into_iter(image.width(), image.height()) {
        if let Some(max_length_num) = max_length {
            if bits.len() >= max_length_num {
                break;
            }
        }
        if pattern(row, col, channel as usize, bit_index as usize) {
            bits.push(get_bit(image, row, col, channel, bit_index));
        }
    }
    bits
}

pub fn extract_image_with_pattern_order(
    image: &RgbImage,
    pattern: impl Pattern,
    order: &IterationOrder,
) -> Result<RgbImage> {
    let (width, height) = extract_image_header(image, &pattern, order);
    if width * height <= image.width() * image.height() {
        let bits = extract_bits_with_pattern_order(image, &pattern, order);
        Ok(RgbImage::from_bitvec(width, height, bits[64..].to_bitvec()))
    } else {
        Err(anyhow!("inner image would have more pixels than original"))
    }
}

pub fn extract_image_header(
    image: &RgbImage,
    pattern: impl Pattern,
    order: &IterationOrder,
) -> (u32, u32) {
    let mut header_bits = extract_bits_with_pattern_order_count(image, pattern, order, Some(64));
    header_bits.reverse();
    let width = header_bits[0..32].load_le::<u32>();
    let height = header_bits[32..64].load_le::<u32>();
    (width, height)
}

pub fn get_bit(image: &RgbImage, row: u32, col: u32, channel: u32, bit_index: u32) -> bool {
    let pixel = image.get_pixel(col, row);
    let channel_value = pixel.0[channel as usize];
    let bits = channel_value.view_bits::<Lsb0>();
    bits[bit_index as usize]
}
