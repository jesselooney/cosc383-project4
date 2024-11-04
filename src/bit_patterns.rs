use bitvec::prelude::*;
use image::{GenericImageView, ImageBuffer, Pixel, RgbImage};

type Pattern = fn(u32, u32, usize, usize) -> bool;

pub fn transject(mut image: RgbImage, pattern: Pattern, mut message: BitVec) -> (RgbImage, BitVec) {
    /*
    I need to
    - iterate of the individual channels of the image
    - check if the channel can be accessed with the pattern
    - do the flipsy flipsy with the message
    */

    let mut message_index = 0;
    for (pixel_row, pixel_col, pixel) in &mut image.enumerate_pixels_mut() {
        for (channel_index, channel_value) in &mut pixel.channels_mut().into_iter().enumerate() {
            let bits = channel_value.view_bits_mut::<Lsb0>();
            for (bit_index, mut bit) in bits.iter_mut().enumerate() {
                if message_index >= message.len() {
                    break;
                }

                if pattern(pixel_row, pixel_col, channel_index, bit_index) {
                    // i'm sure theres a better way to swap these two but idc
                    let tmp = bit.clone();
                    *bit = *message.get(message_index).unwrap();
                    message.set(message_index, tmp);

                    message_index += 1;
                }
            }
        }
    }
    return (image, message);
}

pub fn inject(image: RgbImage, pattern: Pattern, message: BitVec) -> RgbImage {
    // this is a very shallow interface but idk how much i care
    let (image, _) = transject(image, pattern, message);
    image
}

pub fn eject(image: RgbImage, pattern: Pattern, length: Option<usize>) -> BitVec {
    let img_len = image.pixels().len();
    let length = length.unwrap_or(img_len * 8);

    let message = bitvec![usize, Lsb0; 0; length];

    let (_, parsed_message) = transject(image, pattern, message);
    parsed_message
}

pub mod patterns {
    pub fn access_all(row: u32, column: u32, channel: usize, index: usize) -> bool {
        true
    }

    pub fn access_least_significant_bits(
        _row: u32,
        _column: u32,
        _channel: usize,
        index: usize,
    ) -> bool {
        index == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transject_invertible() {
        let img: RgbImage = image::open("assets/hide_text.png").unwrap().into();

        // TODO: theres gotta be a better way to create a message
        // ideally it'd be random and we'd run the test multiple times
        let mut message = bitvec![usize, LocalBits; 0; 40];
        message[0..10].store::<u16>(0x3A8);
        message[10..20].store::<u16>(0x2F9);
        message[20..30].store::<u16>(0x154);
        message[30..40].store::<u16>(0x06D);

        let (modified_img, modified_message) =
            transject(img.clone(), patterns::access_all, message.clone());

        assert_ne!(modified_img, img);
        assert_ne!(modified_message, message);

        let (restored_img, restored_message) = transject(
            modified_img.clone(),
            patterns::access_all,
            modified_message.clone(),
        );

        assert_eq!(restored_img, img);
        assert_eq!(restored_message, message);
    }

    #[test]
    fn inject_eject_reversible() {
        let img: RgbImage = image::open("assets/hide_text.png").unwrap().into();
        let mut message = bitvec![usize, LocalBits; 0; 40];
        message[0..10].store::<u16>(0x3A8);
        message[10..20].store::<u16>(0x2F9);
        message[20..30].store::<u16>(0x154);
        message[30..40].store::<u16>(0x06D);

        let modified_img = inject(img.clone(), patterns::access_all.clone(), message.clone());

        assert_ne!(img, modified_img);

        let restored_msg = eject(modified_img.clone(), patterns::access_all, Some(40));

        assert_eq!(message, restored_msg);

        let incorrectly_restored_msg = eject(modified_img.clone(), patterns::access_all, Some(20));
        assert_ne!(message, incorrectly_restored_msg);
    }
}
