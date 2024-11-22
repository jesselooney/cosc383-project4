use std::fs;

use crate::extensions::RgbImageExt;
use crate::iteration_order::{IterationOrder, Order::*};
use anyhow::{anyhow, Result};
use bitvec::prelude::*;
use image::*;
use itertools::{iproduct, Itertools};

pub fn get_bit(image: &RgbImage, row: u32, col: u32, channel: u32, bit_index: u32) -> bool {
    let pixel = image.get_pixel(col, row);
    let channel_value = pixel.0[channel as usize];
    let bits = channel_value.view_bits::<Lsb0>();
    bits[bit_index as usize]
}

pub fn extract_bits_with_count(
    image: &RgbImage,
    order: &IterationOrder,
    count: Option<usize>,
) -> BitVec<u8> {
    let mut bits = BitVec::new();
    for (row, col, channel, bit_index) in order.clone().into_iter(image.width(), image.height()) {
        if let Some(max_length_num) = count {
            if bits.len() >= max_length_num {
                break;
            }
        }

        bits.push(get_bit(image, row, col, channel, bit_index));
    }
    bits
}

pub fn extract_bits(image: &RgbImage, order: &IterationOrder) -> BitVec<u8> {
    extract_bits_with_count(image, order, None)
}

pub fn extract_bytes_header(image: &RgbImage, order: &IterationOrder) -> Result<u32> {
    let mut header_bits = extract_bits_with_count(image, order, Some(32));
    if header_bits.len() < 32 {
        return Err(anyhow!("not enough bits for a bytes header"));
    }
    header_bits.reverse();
    Ok(header_bits.load_le::<u32>())
}

pub fn extract_bytes(image: &RgbImage, order: &IterationOrder) -> Result<Vec<u8>> {
    let length = extract_bytes_header(image, order)?;
    // For each byte of the inner message, read 8 bits beyond the 32-bit header.
    let bits = extract_bits_with_count(image, order, Some((length * 8) as usize + 32));
    let mut message_bits = bits[32..].to_bitvec();
    message_bits.chunks_exact_mut(8).for_each(|bs| bs.reverse());
    Ok(message_bits.as_raw_slice().to_vec())
}

pub fn extract_image_header(image: &RgbImage, order: &IterationOrder) -> Result<(u32, u32)> {
    let mut header_bits = extract_bits_with_count(image, order, Some(64));
    if header_bits.len() < 64 {
        return Err(anyhow!("not enough bits for an image header"));
    }
    header_bits.reverse();
    let width = header_bits[0..32].load_le::<u32>();
    let height = header_bits[32..64].load_le::<u32>();
    Ok((width, height))
}

pub fn extract_image(image: &RgbImage, order: &IterationOrder) -> Result<RgbImage> {
    let (width, height) = extract_image_header(image, order)?;
    // For each pixel of the inner image, read 24 bits (3 channels per pixel, 8 bits per channel)
    // beyond the 64-bit header.
    let bits = extract_bits_with_count(image, order, Some((width * height * 24) as usize + 64));
    RgbImage::from_bitvec(width, height, bits[64..].to_bitvec())
}
