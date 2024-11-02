use std::time::SystemTime;

use bitvec::prelude::*;
use image::{GenericImageView, ImageBuffer, Pixel, RgbImage};

fn transject(image: RgbImage, pattern: fn(u32, u32, usize, usize) -> bool, message: BitVec) {
    /*
    I need to
    - iterate of the individual channels of the image
    - check if the channel can be accessed with the pattern
    - do the flipsy flipsy with the message
    */
    let mut message_index = 0;
    for (row, col, pixel) in image.enumerate_pixels() {
        if message_index >= message.len() {
            break;
        }

        for (channel_index, channel_value) in pixel.channels().into_iter().enumerate() {
            let bits = BitArray::<_, Msb0>::new(*channel_value);
            for (bit_index, bit) in bits.into_iter().enumerate() {
                if pattern(row, col, channel_index, bit_index) {
                    message_index += 1;
                };
            }
        }
    }
}

fn inject() {}

fn eject() {}

fn access_all(row: u32, column: u32, channel: usize, index: usize) -> bool {
    true
}

fn main() {
    let img = image::open("blahaj-in-bando.png").unwrap();

    let start = SystemTime::now();
    let bits = bitvec![usize, LocalBits; 0; 40];
    println!("{:?}, {}", bits, bits.len());
    transject(img.into(), access_all, bits);
    let end = SystemTime::now();

    let duration = end.duration_since(start).unwrap();
    println!("it took {} seconds", duration.as_secs());
}
