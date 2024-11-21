use anyhow::anyhow;
use anyhow::Result;
use bitvec::prelude::*;
use image::RgbImage;
use std::path::Path;

pub trait RgbImageExt {
    fn from_bitvec<O: BitOrder>(width: u32, height: u32, bits: BitVec<u8, O>) -> Result<RgbImage>;
    fn open(path: impl AsRef<Path>) -> Result<RgbImage>;
}

impl RgbImageExt for RgbImage {
    fn from_bitvec<O: BitOrder>(width: u32, height: u32, bits: BitVec<u8, O>) -> Result<RgbImage> {
        Ok(
            RgbImage::from_vec(width, height, bits.as_raw_slice().to_vec()).ok_or(anyhow!(
                "Failed to make a right-sized image from the bit vector"
            ))?,
        )
    }
    fn open(path: impl AsRef<Path>) -> Result<RgbImage> {
        let image: RgbImage = image::open(path)?.into();
        Ok(image)
    }
}
