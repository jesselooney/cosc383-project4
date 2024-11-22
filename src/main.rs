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
    let mut xor1 = RgbImage::open("./assets/sources/Ideal-extract_FF_0_0_0123.png")?;
    let mut xor2 = RgbImage::open("./assets/sources/Dance-extract_FF_1_0_0123.png")?;

    let one: Rgb<u8> = Rgb::from([255, 255, 255]);
    let zero: Rgb<u8> = Rgb::from([0, 0, 0]);

    let mut one_vec = BitVec::<u8, Msb0>::new();
    let mut two_vec = BitVec::<u8, Msb0>::new();

    for (_, _, pixel) in xor1.enumerate_pixels() {
        if pixel == &one {
            one_vec.push(true)
        } else if pixel == &zero {
            one_vec.push(false)
        } else {
            panic!("yeet")
        }
    }

    for (_, _, pixel) in xor2.enumerate_pixels() {
        if pixel == &one {
            two_vec.push(true)
        } else if pixel == &zero {
            two_vec.push(false)
        } else {
            panic!("yeet")
        }
    }

    let decoded = one_vec ^ two_vec;

    for (row, col, pixel) in xor1.enumerate_pixels_mut() {
        let index: usize = (row * col).try_into().unwrap();
        match decoded[index] {
            true => *pixel = one,
            false => *pixel = zero,
        };
    }

    xor1.save("output.png")?;

    // Amplify all the source images
    // write_amplified_dir("assets/sources/", "assets/amplified/")?;

    //if let Err(err) = write_extractions_dir("assets/sources/", "assets/extractions/") {
    //println!("{}", err);
    //}

    //if let Err(err) = write_extractions_dir("assets/extractions/", "assets/extractions2/") {
    //println!("{}", err);
    //}

    //decode::dream()?;

    Ok(())
}
