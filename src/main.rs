use std::time::SystemTime;

use bitvec::prelude::*;
use image::{GenericImageView, ImageBuffer, Pixel, RgbImage};

fn transject(image: RgbImage, pattern: fn(u32, u32, usize, usize) -> bool) {
    /*
    I need to
    - iterate of the individual channels of the image
    - check if the channel can be accessed with the pattern
    - do the flipsy flipsy with the message
    */
    let mut index = 0;
    for (row, col, pixel) in image.enumerate_pixels() {
        for (channel_index, channel_value) in pixel.channels().into_iter().enumerate() {
            let bits = BitArray::<_, Msb0>::new(*channel_value);
            for (bindex, bit) in bits.into_iter().enumerate() {
                if pattern(row, col, channel_index, bindex) {
                    index += 1;
                };
            }
        }
    }
    println!("index {}", index);
}

fn inject() {}

fn eject() {}

fn access_all(row: u32, column: u32, channel: usize, index: usize) -> bool {
    true
}

fn main() {
    let img = image::open("blahaj-in-bando.png").unwrap();

    let start = SystemTime::now();
    transject(img.into(), access_all);
    let end = SystemTime::now();

    let duration = end.duration_since(start).unwrap();
    println!("it took {} seconds", duration.as_secs());
}
