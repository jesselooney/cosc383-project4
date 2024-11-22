use std::fs;

use crate::extensions::RgbImageExt;
use crate::iteration_order::{IterationOrder, Order::*};
use anyhow::{anyhow, Result};
use bitvec::prelude::*;
use image::*;
use itertools::{iproduct, Itertools};

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

    RgbImage::from_bitvec(width, height, data).unwrap()
}

// --- New functions ---

pub fn subpermutations<T, I>(iterator: I) -> Vec<Vec<T>>
where
    T: Clone,
    I: ExactSizeIterator<Item = T> + Clone,
{
    // TODO Make unique?
    (1..=iterator.len())
        .flat_map(|permutation_size| {
            iterator
                .clone()
                .permutations(permutation_size)
                .collect::<Vec<Vec<T>>>()
        })
        .collect()
}

pub fn make_reasonable_iteration_orders() -> impl Iterator<Item = IterationOrder> {
    let forward_or_reverse = [Forward, Reverse];
    let channel_index_subpermutations = subpermutations([0, 1, 2].into_iter());
    // Only check at most the four least significant bits.
    let bit_index_subpermutations = [vec![0], vec![1], vec![2], vec![0, 1], vec![0, 1, 2]];
    let index_orders = (0..4).permutations(4);

    let iteration_orders = iproduct!(
        forward_or_reverse,
        forward_or_reverse,
        channel_index_subpermutations,
        bit_index_subpermutations,
        index_orders
    );

    iteration_orders.map(
        |(row_order, column_order, channel_indices, bit_indices, index_order)| {
            IterationOrder::new(
                row_order,
                column_order,
                channel_indices,
                bit_indices,
                index_order,
            )
        },
    )
}

pub fn try_extraction_orders(image: &RgbImage, prefix: &str) -> Result<()> {
    // TODO we get lots of duplicate iteration orders when we change the index order but one or
    // more of the indices (typically, channel/bit) is only allowed to take one value
    // Maybe we can just cache the first N bits of the bitvec (after the header, which might be the
    // same for multiple images stored in one) and then abort processing if we start finding the
    // exact same data.

    const PEEK_SIZE: usize = 128;
    assert!(PEEK_SIZE >= 64);

    let mut prev_peek_bits: BitVec<u8> = bitvec![u8, Lsb0; 1; PEEK_SIZE];
    let iteration_orders = make_reasonable_iteration_orders();
    for iteration_order in iteration_orders {
        let peek_bits = extract_bits_with_order_count(image, &iteration_order, Some(PEEK_SIZE));
        let (width, height) = extract_image_header(&peek_bits);
        let length = extract_text_header(&peek_bits);

        /*
        if peek_bits == prev_peek_bits {
            continue;
        } else {
            // Update our cached peek bits so long as we were actually able to read out all
            // PEEK_SIZE of them (so that we dont change the length of `prev_peek_bits`).
            println!("{} \n {}", prev_peek_bits, peek_bits);
            if peek_bits.len() == prev_peek_bits.len() {
                prev_peek_bits = peek_bits;
            }
        }*/

        // We only read out at most half the bits in an image, so the subimage can be at most half
        // the size. Even this constraint is a little loose.
        let max_subimage_size = (image.width() as u64) * (image.height() as u64) / 2;
        let image_size = (width as u64) * (height as u64);

        // Image extraction
        if (image_size <= max_subimage_size) && (width != 0) && (height != 0) {
            // Go ahead with full extraction
            println!(
                "Found {} x {} image using {}",
                width,
                height,
                iteration_order.name()
            );
            if let Ok(out_image) = extract_image_with_order(image, &iteration_order) {
                let file_name = format!("{}{}.png", prefix, iteration_order.name());
                if out_image.save(file_name.as_str()).is_err() {
                    println!("Failed to save image as: {}", file_name);
                }
            } else {
                println!("Failed to extract!");
            }
        }

        // Every pixel in an image is 3 bytes.
        let max_text_size = max_subimage_size * 3;
        // Text extraction
        if (length <= max_text_size) && (length >= 10) {
            println!(
                "Fount {} byte message using {}",
                length,
                iteration_order.name()
            );

            let bits =
                extract_bits_with_order_count(image, &iteration_order, Some((length * 8) as usize));
            let target_bits = bits[64..].to_bitvec();
            let bytes = target_bits.as_raw_slice();
            if is_text(bytes) {
                let file_name = format!("{}{}.bytes", prefix, iteration_order.name());
                if fs::write(file_name.as_str(), bytes).is_err() {
                    println!("Failed to save bytes as: {}", file_name);
                }
            }
        }
    }

    Ok(())
}

pub fn is_text(bytes: &[u8]) -> bool {
    for byte in bytes {
        if *byte < 32 {
            return false;
        }
    }
    true
}

pub fn extract_bits_with_order(image: &RgbImage, order: &IterationOrder) -> BitVec<u8> {
    extract_bits_with_order_count(image, order, None)
}

pub fn extract_bits_with_order_count(
    image: &RgbImage,
    order: &IterationOrder,
    max_length: Option<usize>,
) -> BitVec<u8> {
    let mut bits = BitVec::new();
    for (row, col, channel, bit_index) in order.clone().into_iter(image.width(), image.height()) {
        if let Some(max_length_num) = max_length {
            /*
                        println!(
                            "row: {}, col: {}, chn: {}, idx: {}",
                            row, col, channel, bit_index
                        );
            */
            if bits.len() >= max_length_num {
                break;
            }
        }

        bits.push(get_bit(image, row, col, channel, bit_index));
    }
    bits
}

pub fn extract_image_with_order(image: &RgbImage, order: &IterationOrder) -> Result<RgbImage> {
    let bits = extract_bits_with_order(image, order);
    let (width, height) = extract_image_header(&bits);
    if width * height <= image.width() * image.height() {
        let bits = extract_bits_with_order(image, order);
        Ok(RgbImage::from_bitvec(
            width,
            height,
            bits[64..].to_bitvec(),
        )?)
    } else {
        Err(anyhow!("inner image would have more pixels than original"))
    }
}

pub fn extract_image_header(bits: &BitVec<u8>) -> (u32, u32) {
    let mut header_bits = bits[0..64].to_bitvec();
    header_bits.reverse();
    let width = header_bits[0..32].load_le::<u32>();
    let height = header_bits[32..64].load_le::<u32>();
    (width, height)
}

pub fn extract_text_header(bits: &BitVec<u8>) -> u64 {
    let mut header_bits = bits[0..64].to_bitvec();
    header_bits.reverse();
    header_bits.load_le::<u64>()
}

pub fn get_bit(image: &RgbImage, row: u32, col: u32, channel: u32, bit_index: u32) -> bool {
    let pixel = image.get_pixel(col, row);
    let channel_value = pixel.0[channel as usize];
    let bits = channel_value.view_bits::<Lsb0>();
    bits[bit_index as usize]
}
