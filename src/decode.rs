use crate::{bit_patterns::eject, transform};
use anyhow::Result;
use image::RgbImage;

/// This module will contain 1 function per image in the set that has data in it.
/// This module mainly exists to record what we had to do in order to solve the various images,
/// without clogging up `main.rs` with a bunch of scratchpad code. All functions here should be impure,
/// meaning they should contain all filesystem calls needed to function, in order to reduce the amount of code in main.
// Here's a list of files that might have information hidden in them
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

/// Decodes Acorn.png
/// - data is encoded left to right, top to bottom
/// - data is stored in the third lsb (index 2)
/// - source image is 1024 by 1024
pub fn acorn() -> Result<()> {
    Ok(())
}

/// Decodes BombAnswers.png
/// - data is encoded top to bottom, left to right
/// - data is stored in the first 3 lsbs (at least)
/// - source image is 1024 by 1024
pub fn bomb_answers() {}

/// Decodes Dream.png
/// - data is stored left to right, top to bottom
/// - data is stored in the first 3 lsbs (at least)
pub fn dream() {}

/// Decodes `Myself.png`
/// - appears to be encoded top to bottom, starting from the left
/// - image is 1024 by 1024
/// - the image only hides data in the first lsb (index 0)
pub fn myself() -> Result<()> {
    let image: RgbImage = image::open("./assets/project-images/Myself.png")
        .unwrap()
        .into();

    //let modified_image = transform::amplify_index_one_bit(image);
    //modified_image.save("./output.png")?;

    let pattern = |row, col, chn, idx| true;

    let bits = eject(image, pattern, Some(800));
    Ok(())
}

/// Decodes Friendship.png
/// - data encoded top to bottom
/// - only stores data in the second lsb (index 1)
pub fn friendship() -> Result<()> {
    Ok(())
}

/// Decodes Security.png
/// - data encoded top to bottom, left to right
/// - only stores data in the second lsb (index 1)
pub fn security() -> Result<()> {
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

/// Decodes TouchingGrass.png
/// - data encoded top to bottom, left to right
/// - stores data in the first 2 lsbs
pub fn touching_grass() -> Result<()> {
    Ok(())
}
