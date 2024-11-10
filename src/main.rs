mod decode;
mod detect;
mod transform;

use std::io::Write;
use std::path::PathBuf;
mod bit_patterns;
use anyhow::Result;
use bit_patterns::{eject, patterns};
use bitvec::field::BitField;
use image::{ImageReader, RgbImage};

fn main() -> Result<()> {
    /*
        // This is all scratch work to extract stuff out of the examples.
        let img: RgbImage = image::open("assets/hide_text.png").unwrap().into();
        let mut length_bits = eject(
            img.clone(),
            patterns::access_least_significant_bits,
            Some(32),
        );
        length_bits.reverse(); // no clue why i have to do this
        println!("{:?}", length_bits);
        let length = length_bits.load::<usize>() * 8 + 32;
        println!("string length: {length}");
    */

    decode::decode_myself()?;

    Ok(())
}
