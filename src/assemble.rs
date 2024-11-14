use anyhow::Result;
use bitvec::prelude::*;
use image::codecs::png::PngEncoder;
use image::ImageEncoder;

use std::{
    fs::{self, File},
    path::Path,
};

/// Helper function to write each byte of BitVec to a file. Watch out for the bit order.
pub fn write_raw<O: BitOrder>(path: impl AsRef<Path>, bv: BitVec<u8, O>) -> Result<()> {
    fs::write(path, bv.as_raw_slice())?;
    Ok(())
}

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
