use crate::extensions::*;
use crate::extract::*;
use crate::helpers::*;
use crate::iteration_order::IterationOrder;
use anyhow::Result;
use bitvec::prelude::*;
use image::*;

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
    Ok(())
}

/// Decodes Abominable.png
/// - data is encoded left to right, top to bottom
/// - data is stored in the first lsb (index 0)
/// - source image is 2048 by 2048
pub fn abominable() -> Result<()> {
    // The internal image is hidden in the red channel in bit index 0. It is an image of a graph
    // public key. I have not yet checked if this contains anything else. If it did, it would
    // probably show on the white background, but it may be worth diving deeper later on.
    write_extracted_image(
        "assets/working/Abominable/Abominable.png",
        &IterationOrder::top_to_bottom_left_to_right([0], [0]),
    )?;
    Ok(())
}
/// Decodes Acorn.png
/// - data is encoded left to right, top to bottom
/// - data is stored in the third lsb (index 2)
/// - source image is 1024 by 1024
pub fn acorn() -> Result<()> {
    // This is a mirrored (?), scaled-down version of the original.
    write_extracted_image("assets/working/Acorn/Acorn.png", &IterationOrder::default())?;
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
    // This yields an image with three Poke Balls and someone looking inquisitively at them.
    write_extracted_image(
        "assets/working/Cookies/Cookies.png",
        &IterationOrder::default(),
    )?;
    // write_amplified_images("assets/working/Cookies/Cookies-extract.png")?;
    // The least significant bits contain a block of mostly white with some chunks of other data in
    // it. Not sure what to do with the extracted bytes.
    write_extracted_bytes(
        "assets/working/Cookies/Cookies-extract.png",
        &IterationOrder::default(),
    )?;
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
    // This yields the same image but mirrored/rotated.
    write_extracted_image(
        "assets/working/Dream/Dream.png",
        &IterationOrder::top_to_bottom_left_to_right([0, 1, 2], [0, 1, 2]),
    )?;
    write_amplified_images("assets/working/Dream/Dream-extract.png")?;
    Ok(())
}

/// Decodes Friendship.png
/// - data encoded top to bottom
/// - only stores data in the second lsb (index 1)
pub fn friendship() -> Result<()> {
    // This yields the same image but smaller.
    write_extracted_image(
        "assets/working/Friendship/Friendship.png",
        &IterationOrder::top_to_bottom_left_to_right([0, 1, 2], [1]),
    )?;
    write_amplified_images("assets/working/Friendship/Friendship-extract.png")?;
    Ok(())
}

pub fn ideal() -> Result<()> {
    // Black-and-white image for XORing.
    write_extracted_image(
        "assets/working/Ideal/Ideal.png",
        &IterationOrder::top_to_bottom_left_to_right([0], [0]),
    )?;
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

pub fn music_to_my_ears() -> Result<()> {
    Ok(())
}

/// Decodes `Myself.png`
/// - data is encoded top to bottom, starting from the left
/// - the image only hides data in the first lsb (index 0)
/// - source image is 1024 by 1024
pub fn myself() -> Result<()> {
    Ok(())
}

/// Decodes Phishing.png
/// - data is stored left to right, top to bottom
/// - the image only hides data in the first lsb (index 0)
/// - source image is 1024 by 1024
pub fn phishing() -> Result<()> {
    // This is a scaled-down version of the original.
    write_extracted_image(
        "assets/working/Phishing/Phishing.png",
        &IterationOrder::default(),
    )?;
    Ok(())
}

pub fn professor_alfeld() -> Result<()> {
    // This yields a rotated version of the original.
    write_extracted_image(
        "assets/working/ProfessorAlfeld/ProfessorAlfeld.png",
        &IterationOrder::top_to_bottom_left_to_right([2], [0]),
    )?;
    write_amplified_images("assets/working/ProfessorAlfeld/ProfessorAlfeld-extract.png")?;
    Ok(())
}

pub fn robot_on_real_cat() -> Result<()> {
    // This yields a rotated version of the original.
    write_extracted_image(
        "assets/working/RobotOnRealCat/RobotOnRealCat.png",
        &IterationOrder::top_to_bottom_left_to_right([2], [0]),
    )?;
    write_amplified_images("assets/working/RobotOnRealCat/RobotOnRealCat-extract.png")?;
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
