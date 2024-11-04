use crate::bit_patterns::{eject, patterns};
use bitvec::prelude::*;
use image::RgbImage;

fn detect() {
    let image: RgbImage = image::open("assets/blahaj-in-bando.png").unwrap().into();
    let length = image.len();
    let lsbs = eject(image, patterns::access_all, Some(length * 3 * 8));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detect_works() {
        detect();
    }
}
