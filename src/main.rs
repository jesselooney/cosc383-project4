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
    // Amplify all the source images
    // write_amplified_dir("assets/sources/", "assets/amplified/")?;

    if let Err(err) = write_extractions_dir("assets/sources/", "assets/extractions/") {
        println!("{}", err);
    }

    if let Err(err) = write_extractions_dir("assets/extractions/", "assets/extractions2/") {
        println!("{}", err);
    }

    Ok(())
}
