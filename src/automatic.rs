use crate::iteration_order::{IterationOrder, Order::*};
use crate::{extract::*, with_file_path_suffix};
use anyhow::{anyhow, Result};
use bitvec::prelude::*;
use image::*;
use itertools::{iproduct, Itertools};
use std::fs;
use std::path::Path;

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
    let index_orders = [vec![0, 1, 2, 3], vec![1, 0, 2, 3]];

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

pub fn write_extractions_dir(
    input_dir_path: impl AsRef<Path>,
    output_dir_path: impl AsRef<Path>,
) -> Result<()> {
    let dir_entries = fs::read_dir(input_dir_path)?;
    for dir_entry in dir_entries {
        let path = dir_entry?.path();
        if path.is_file() {
            println!("Extracting from {}", path.display());
            if let Err(err) = write_extractions(path, &output_dir_path) {
                println!("Failed to write extractions: {}", err);
            }
        }
    }

    Ok(())
}

pub fn write_extractions(
    image_path: impl AsRef<Path>,
    output_dir_path: impl AsRef<Path>,
) -> Result<()> {
    let image: RgbImage = image::open(&image_path)?.into();
    let image_name = image_path
        .as_ref()
        .file_name()
        .ok_or(anyhow!("`image_path` must be a file path"))?;
    let iteration_orders = make_reasonable_iteration_orders();
    for order in iteration_orders {
        let new_file_name = with_file_path_suffix(
            image_name,
            format!("-extract_{}", order.identifier()).as_str(),
        )
        .expect("`image_path` should be a file path");

        let mut new_file_path = output_dir_path.as_ref().to_path_buf();
        new_file_path.push(new_file_name);
        let new_file_path = new_file_path;

        if let Ok(inner_image) = try_extract_image(&image, &order) {
            println!(
                "Found {} by {} image using order {}",
                inner_image.width(),
                inner_image.height(),
                order.identifier()
            );
            println!("{}", new_file_path.display());
            if let Err(err) = inner_image.save(&new_file_path) {
                println!("Failed to save image file: {}", err);
            }
        }

        if let Ok(bytes) = try_extract_text(&image, &order) {
            println!(
                "Found {}-byte text using order {}",
                bytes.len(),
                order.identifier()
            );
            if let Err(err) = fs::write(new_file_path.with_extension("txt"), bytes) {
                println!("Failed to save text file: {}", err);
            }
        }
    }

    Ok(())
}

pub fn try_extract_image(image: &RgbImage, order: &IterationOrder) -> Result<RgbImage> {
    let (width, height) = extract_image_header(image, order)?;

    let pixel_count = (width as u64) * (height as u64);
    let max_pixel_count = (image.width() as u64) * (image.height() as u64) / 2;

    if (pixel_count <= max_pixel_count) && (width != 0) && (height != 0) {
        return extract_image(image, order);
    }
    Err(anyhow!("heuristic tests failed"))
}

pub fn try_extract_text(image: &RgbImage, order: &IterationOrder) -> Result<Vec<u8>> {
    let length = extract_bytes_header(image, order)? as u64;

    let max_length = ((image.width() as u64) * (image.height() as u64) / 2) * 3;

    if (length <= max_length) && (length >= 10) {
        let bytes = extract_bytes(image, order)?;
        if is_text(bytes.as_ref()) {
            return Ok(bytes);
        }
    }
    Err(anyhow!("heuristic tests failed"))
}

pub fn is_text(bytes: &[u8]) -> bool {
    let mut printable_count = 0f32;
    let mut letter_count = 0f32;
    for byte in bytes {
        if *byte >= 32 {
            printable_count += 1f32;
        }
        if (65..=90).contains(byte) || (97..=122).contains(byte) {
            letter_count += 1f32;
        }
    }
    let printable_frequency = printable_count / (bytes.len() as f32);
    let letter_frequency = letter_count / (bytes.len() as f32);
    printable_frequency > 0.8 && letter_frequency > 0.6
}
