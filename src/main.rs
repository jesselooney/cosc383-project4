mod assemble;
mod bit_patterns;
mod bitvec_extensions;
mod decode;
mod detect;
mod transform;
use anyhow::Result;

fn main() -> Result<()> {
    transform::amplify_all()?;
    //decode::decode_myself()?;

    Ok(())
}
