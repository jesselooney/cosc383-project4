mod automatic;
mod decode;
mod extensions;
mod extract;
mod helpers;
mod iteration_order;
use crate::iteration_order::{IterationOrder, Order::Forward};

use std::fs;

use crate::automatic::*;
use crate::extensions::*;
use crate::extract::*;
use crate::helpers::*;
use anyhow::Result;
use bitvec::field::BitField;
use bitvec::prelude::*;
use image::*;
use itertools::iproduct;
use itertools::Itertools;

fn main() -> Result<()> {
    /*
    if let Err(err) = write_extractions_dir("assets/sources/", "assets/extractions/") {
        println!("{}", err);
    }

    if let Err(err) = write_extractions_dir("assets/extractions/", "assets/extractions2/") {
        println!("{}", err);
    }*/

    //write_amplified_dir("tests/sources/", "tests/amplified").expect("failed to amplify all");
    //
    //write_extracted_bytes("tests/sources/383.png", &IterationOrder::default())
    //    .expect("should work");

    let im1: RgbImage = image::open("assets/extractions/Ideal-extract_FF_0_0_0123.png")?.into();
    let im2: RgbImage = image::open("assets/extractions/Dance-extract_FF_1_0_0123.png")?.into();

    let bits1 = extract_bits(&im1, &IterationOrder::all());
    let bits2 = extract_bits(&im2, &IterationOrder::all());

    let bits3 = bits1 ^ bits2;

    RgbImage::from_bitvec(im1.width(), im1.height(), bits3)?.save("xor-out.png")?;

    Ok(())
}
