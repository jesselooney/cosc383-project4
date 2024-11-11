use anyhow::Result;
use bitvec::prelude::*;
use std::{fs, path::Path};

fn write_raw(path: impl AsRef<Path>, bv: BitVec<u8>) -> Result<()> {
    fs::write(path, bv.as_raw_slice())?;
    Ok(())
}
