mod decode;
mod detect;
mod transform;

use std::io::Write;
use std::path::PathBuf;
mod bit_patterns;
use anyhow::Result;
use bit_patterns::{eject, patterns};
use bitvec::field::BitField;
use image::{ImageReader, RgbImage};

fn main() -> Result<()> {
    transform::amplify_all()?;
    //decode::decode_myself()?;

    Ok(())
}
