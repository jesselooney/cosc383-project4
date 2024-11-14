use crate::{
    assemble::write_image,
    bit_patterns::{eject, extract, patterns},
    transform,
};
use anyhow::Result;
use bitvec::prelude::*;
use bitvec::{field::BitField, view::BitView};
use image::RgbImage;

/// This module will contain 1 function per image in the set that has data in it.
/// This module mainly exists to record what we had to do in order to solve the various images,
/// without clogging up `main.rs` with a bunch of scratchpad code. All functions here should be impure,
/// meaning they should contain all filesystem calls needed to function, in order to reduce the amount of code in main.

// Of the 29 total images these 20 images might have info in them:
// - 383.png
// - Abominable.png
// - acorn.png
// - BackdoorAtks.png
// - BombAnswers.png
// - Cookies.png
// - Dance.png
// - Dream.png
// - Friendship.png
// - Ideal.png (maybe, might just be weird artifacting)
// - Lockpicking.png
// - Lucy.png (maybe, might just be artifacting)
// - Teach.png
// - Myself.png
// - RobotOnRealCat.png
// - Soccer.png
// - Security.png
// - Spyware.png
// - Steganography.png (data appears to be on the left side, maybe encoded top to bottom instead of left to right)
// - TouchingGrass.png
//
// the following images were corrupted and then replaced with uncorrupted images, so their notes might not be up to date:
// - MusicToMyEars.png (this file wouldn't have had to be replaced if it didn't have data in it, am suspicious)
// - Dream.png (appears to be superficially unchanged)
// - Believe.png (looks like it has data in it, but is inconclusive)

/// Decodes 383.png
/// - data is encoded left to right, top to bottom
/// - data is stored in the first lsb (index 0)
/// - source image is 2048 by 2048
pub fn three_eight_three() -> Result<()> {
    let im: RgbImage = image::open("assets/sources/383.png")?.into();
    let bits = extract(&im, patterns::access_least_significant_bits);
    println!("{}; {}", &bits[0..32], &bits[32..64]);
    let height = bits[0..32].load_be::<u32>();
    let width = bits[32..64].load_be::<u32>();
    println!("{} x {}", width, height);
    let bit_len = (width * height * 3 * 8) as usize;
    let data: BitVec<u8, Msb0> = bits[64..(bit_len + 64)].to_bitvec();
    write_image("assets/messages/383.png", data, width, height)?;
    Ok(())
}

/// Decodes Abominable.png
/// - data is encoded left to right, top to bottom
/// - data is stored in the first lsb (index 0)
/// - source image is 2048 by 2048
pub fn abominable() -> Result<()> {
    Ok(())
}
/// Decodes Acorn.png
/// - data is encoded left to right, top to bottom
/// - data is stored in the third lsb (index 2)
/// - source image is 1024 by 1024
pub fn acorn() -> Result<()> {
    Ok(())
}

/// Decodes BombAnswers.png
/// - data is encoded top to bottom, left to right
/// - data is stored in the first 3 lsbs
/// - source image is 1024 by 1024
pub fn bomb_answers() -> Result<()> {
    Ok(())
}

/// Decodes Cookies.png
/// - data is encoded left to right, top to bottom
/// - data is stored in the first lsb
/// - source image is 2048 by 2048
pub fn cookies() -> Result<()> {
    Ok(())
}

/// Decodes Dance.png
/// - data is stored left to right, top to bottom
/// - data is stored in the first lsb
/// - source image is 2048 by 2048
pub fn dance() -> Result<()> {
    Ok(())
}

/// Decodes Dream.png
/// - data is stored left to right, top to bottom
/// - data is stored in the first 3 lsbs
pub fn dream() -> Result<()> {
    Ok(())
}

/// Decodes Friendship.png
/// - data encoded top to bottom
/// - only stores data in the second lsb (index 1)
pub fn friendship() -> Result<()> {
    Ok(())
}

/// Decodes Lockpicking.png
/// WARNING: make sure you read the encoding order correctly
/// - data is stored bottom to top, left to right
///
/// - data is stored in the first lsb
/// - source image is 1024 by 1024
pub fn lockpicking() -> Result<()> {
    Ok(())
}

/// Decodes `Myself.png`
/// - data is encoded top to bottom, starting from the left
/// - the image only hides data in the first lsb (index 0)
/// - source image is 1024 by 1024
pub fn myself() -> Result<()> {
    let image: RgbImage = image::open("./assets/project-images/Myself.png")
        .unwrap()
        .into();
    Ok(())
}

/// Decodes Phishing.png
/// - data is stored left to right, top to bottom
/// - the image only hides data in the first lsb (index 0)
/// - source image is 1024 by 1024
pub fn phishing() -> Result<()> {
    Ok(())
}

/// Decodes Security.png
/// - data encoded top to bottom, left to right
/// - only stores data in the second lsb (index 1)
pub fn security() -> Result<()> {
    Ok(())
}

/// Decodes Soccer.png
/// - data encoded left to right, top to bottom
///     - based on how the amplified image looks i'd guess the data is hidden in every other pixel, or something like that
/// - data is stored in the first lsb (index 0)
pub fn soccer() -> Result<()> {
    Ok(())
}

/// Decodes Spyware.png
/// - data encoded top to bottom, left to right
/// - only stores data in the third lsb (index 2)
pub fn spyware() -> Result<()> {
    Ok(())
}

/// Decodes Steganography.png
/// - data encoded top to bottom, left to right
/// - only stores data in the second lsb (index 1)
pub fn steganography() -> Result<()> {
    Ok(())
}

/// Decodes teach.png
/// - data encoded left to right, top to bottom
/// - looks like encoding skips pixels or channels
/// - data stored in first lsb
pub fn teach() -> Result<()> {
    Ok(())
}

/// Decodes TouchingGrass.png
/// - data encoded top to bottom, left to right
/// - stores data in the first 2 lsbs
pub fn touching_grass() -> Result<()> {
    Ok(())
}

pub fn hide_image() -> Result<()> {
    let im: RgbImage = image::open("assets/hide_image.png")?.into();
    let bits = extract(&im, patterns::access_least_significant_bits);
    let height = bits[0..32].load_be::<u32>();
    let width = bits[32..64].load_be::<u32>();
    let bit_len = (width * height * 3 * 8) as usize;
    let data: BitVec<u8, Msb0> = bits[64..(bit_len + 64)].to_bitvec();
    write_image("assets/hide_image-extraction.png", data, width, height)?;
    Ok(())
}
