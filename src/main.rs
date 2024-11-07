use std::fs::File;
use std::io::Write;
mod bit_patterns;
mod detect;
mod transform;
use anyhow::Result;
use bit_patterns::{eject, patterns};
use bitvec::field::BitField;
use image::RgbImage;

fn main() -> Result<()> {
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

    let mut text_bits = eject(img, patterns::access_least_significant_bits, Some(length));
    text_bits = text_bits.split_off(32);
    text_bits = transform::flipsy_flipsy(text_bits);

    let mut file = File::create("output.txt")?;

    let bits: &[u8] = text_bits.as_raw_slice();

    file.write_all(bits).unwrap();

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
    let length: usize = 64 + (total_pixels) * 3 * 8 * 10;
    println!("length: {:?}", length);
    let mut image_bits = eject(img, patterns::access_least_significant_bits, Some(length));
    image_bits = image_bits.split_off(64);
    image_bits.reverse();
    //image_bits = transform::flipsy_flipsy(image_bits);

    let mut file = File::create("output.png")?;

    let bits: &[u8] = image_bits.as_raw_slice();

    file.write_all(bits).unwrap();

    Ok(())
}
