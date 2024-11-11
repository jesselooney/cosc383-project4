use anyhow::Result;
use bitvec::prelude::*;
use std::{fs, path::Path};

/// Helper function to write each byte of BitVec to a file. Watch out for the bit order.
fn write_raw(path: impl AsRef<Path>, bv: BitVec<u8>) -> Result<()> {
    fs::write(path, bv.as_raw_slice())?;
    Ok(())
}
