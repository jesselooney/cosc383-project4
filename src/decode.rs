use anyhow::Result;
/// This module will contain 1 function per image in the set that has data in it.
/// This module mainly exists to record what we had to do in order to solve the various images,
/// without clogging up `main.rs` with a bunch of scratchpad code. All functions here should be impure,
/// meaning they should contain all filesystem calls needed to function, in order to reduce the amount of code in main.
// Here's a list of files that might have information hidden in them
// - Myself.png
// - RobotOnRealCat.png
// - Soccer.png
// - Steganography.png (data appears to be on the left side, maybe encoded top to bottom instead of left to right)
// - Teach.png
// - TouchingGrass.png
// - 383.png
// - Abominable.png
// - BackdoorAtks.png
// - BombAnswers.png
// - Cookies.png
// - Dance.png
// - Dream.png
// - Ideal.png (maybe, might just be weird artifacting)
// - Lockpicking.png
// - Lucy.png (maybe, might just be artifacting)
use image::RgbImage;

use crate::transform;

/// Decodes `Myself.png`
/// - appears to be encoded top to bottom, starting from the left
/// - image is 1024 by 1024
/// - the image only hides data in the least significant bits
pub fn decode_myself() -> Result<()> {
    let image: RgbImage = image::open("./assets/project-images/Myself.png")
        .unwrap()
        .into();

    let modified_image = transform::amplify_index_one_bit(image);

    modified_image.save("./output.png")?;
    Ok(())
}
