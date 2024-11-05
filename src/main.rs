use std::{
    fs::File,
    io::{Read, Write},
    time::SystemTime,
};
mod bit_patterns;
mod detect;
mod transform;
use anyhow::Result;
use bit_patterns::{eject, patterns};
use bitvec::{
    bitvec,
    field::BitField,
    order::{Lsb0, Msb0},
};
use image::RgbImage;

pub fn get_width(row: u32, _column: u32, _channel: usize, index: usize) -> bool {
    // the first 10 pixels + 2
    if index == 0 && row <= 10 {
        true
    } else {
        false
    }
}

fn main() -> Result<()> {
    let start = SystemTime::now();
    let img: RgbImage = image::open("assets/hide_text.png").unwrap().into();
    let transformed_img = transform::amplify_least_significant_bits(img.clone());
    transformed_img.save("output.png")?;

    let mut length_bits = eject(
        img.clone(),
        patterns::access_least_significant_bits,
        Some(32),
    );
    println!("{:?}", length_bits);
    length_bits.reverse(); // no clue why i have to do this
    let length = length_bits.load::<usize>() + 32;
    println!("{length}");
    let mut text_bits = eject(img, patterns::access_least_significant_bits, Some(length));
    text_bits = text_bits.split_off(32);
    //text_bits.reverse();

    let mut file = File::create("output.txt")?;

    let bits: &[u8] = text_bits.as_raw_slice();

    file.write_all(bits).unwrap();

    /*
        let img: RgbImage = image::open("assets/hide_image.png").unwrap().into();
        let mut height_width_bits = eject(
            img.clone(),
            patterns::access_least_significant_bits,
            Some(64),
        );

        println!("{:?}", height_width_bits);
        height_width_bits.reverse(); // no clue why i have to do this
        let width_bits = height_width_bits.split_off(32);
        let height_bits = height_width_bits;
        println!("{:?}", width_bits);
        println!("{:?}", height_bits);

        let height = height_bits.load::<usize>();
        let width = width_bits.load::<usize>();
        println!("height: {height}");
        println!("height: {width}");

        let total_pixels = height * width;
        let length: usize = 64 + (total_pixels) * 3 * 8;
        let mut image_bits = eject(img, patterns::access_least_significant_bits, Some(length));
        image_bits = image_bits.split_off(64);
        image_bits.reverse();
        println!("{:?}", image_bits);

        let mut file = File::create("output.png")?;

        let bits: &[u8] = image_bits.as_raw_slice();

        file.write_all(bits).unwrap();

    */

    let end = SystemTime::now();
    let duration = end.duration_since(start).unwrap();
    println!("it took {} seconds", duration.as_secs());

    Ok(())
}
