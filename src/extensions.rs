use anyhow::Result;
use bitvec::prelude::*;
use image::RgbImage;
use std::path::Path;

pub trait RgbImageExt {
    fn from_bitvec<O: BitOrder>(width: u32, height: u32, bits: BitVec<u8, O>) -> RgbImage;
    fn open(path: impl AsRef<Path>) -> Result<RgbImage>;
}

impl RgbImageExt for RgbImage {
    fn from_bitvec<O: BitOrder>(width: u32, height: u32, bits: BitVec<u8, O>) -> RgbImage {
        RgbImage::from_vec(width, height, bits.as_raw_slice().to_vec())
            .expect("Image dimensions should be appropriate for the size of the BitVec")
    }
    fn open(path: impl AsRef<Path>) -> Result<RgbImage> {
        let image: RgbImage = image::open(path)?.into();
        Ok(image)
    }
}
