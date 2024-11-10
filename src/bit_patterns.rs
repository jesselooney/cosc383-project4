use bitvec::prelude::*;
use image::{Pixel, RgbImage};

/// `Patterns` specify which bits the `inject` and `eject` functions are allowed to access when running.
/// They do this by receiving the row and column of the pixels being queried, followed by the channel
/// index (r, g, b), and then the bit index (0 - 7, least significant bits first) within that channel.
/// The function then returns a `bool` to indicate whether the bit can be accessed or not.
///
/// # Examples
///
/// ```rs
/// // This pattern would allow every single bit to be accessed
/// fn access_all(row, column, channel, bit_index) {true}
///
/// // This pattern would allow only the least significant bits to be accessed
/// fn access_least_significant_bits(row, column, channel, bit_index) {bit_index == 0}
/// ````
type Pattern = fn(u32, u32, usize, usize) -> bool;

/// Contains the business logic for the `inject` and `eject` functions. This function is not meant to be called directly.
///
/// The function does these things:
/// 1. Encodes the contents of `message` into `image`. The `pattern` dictates which bits of the image can be used.
/// 2. Places all of the bits displaced by this encoding back into `message`
/// 3. Returns the new `image` and `message` in a tuple.
fn transject(
    mut image: RgbImage,
    pattern: Pattern,
    mut message: BitVec<u8>,
) -> (RgbImage, BitVec<u8>) {
    let mut message_index = 0;

    for (pixel_row, pixel_col, pixel) in &mut image.enumerate_pixels_mut() {
        for (channel_index, channel_value) in &mut pixel.channels_mut().iter_mut().enumerate() {
            let bits = channel_value.view_bits_mut::<Lsb0>();
            for (bit_index, mut bit) in bits.iter_mut().enumerate() {
                if message_index >= message.len() {
                    break;
                }

                if pattern(pixel_row, pixel_col, channel_index, bit_index) {
                    // TODO Find a better way to swap these things.
                    let tmp = *bit;
                    *bit = *message
                        .get(message_index)
                        .expect("Failed to get next message bit at message_index");
                    message.set(message_index, tmp);

                    message_index += 1;
                }
            }
        }
    }

    (image, message)
}

/// This function hides bits from a message into a provided image. This function will only check the bits
/// specified by the `pattern`.
pub fn inject(image: RgbImage, pattern: Pattern, message: BitVec<u8>) -> RgbImage {
    let (image, _) = transject(image, pattern, message);
    image
}

/// This function extracts bits from in image. This function will only check the bits specified
/// by the `pattern`.
pub fn eject(image: RgbImage, pattern: Pattern, length: Option<usize>) -> BitVec<u8> {
    let img_len = image.pixels().len();
    let length = length.unwrap_or(img_len * 8);

    let message = bitvec![u8, Lsb0; 0; length];

    let (_, parsed_message) = transject(image, pattern, message);
    parsed_message
}

/// Sample `Pattern`s to be used with `inject` and `eject`
pub mod patterns {
    pub fn access_all(_row: u32, _column: u32, _channel: usize, _index: usize) -> bool {
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

    pub fn access_index_one(_row: u32, _column: u32, _channel: usize, index: usize) -> bool {
        index == 1
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
        let mut message = bitvec![u8, LocalBits; 0; 40];
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
        let mut message = bitvec![u8, LocalBits; 0; 40];
        message[0..10].store::<u16>(0x3A8);
        message[10..20].store::<u16>(0x2F9);
        message[20..30].store::<u16>(0x154);
        message[30..40].store::<u16>(0x06D);

        let modified_img = inject(img.clone(), patterns::access_all, message.clone());

        assert_ne!(img, modified_img);

        let restored_msg = eject(modified_img.clone(), patterns::access_all, Some(40));

        assert_eq!(message, restored_msg);

        let incorrectly_restored_msg = eject(modified_img.clone(), patterns::access_all, Some(20));
        assert_ne!(message, incorrectly_restored_msg);
    }
}
