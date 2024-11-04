use std::time::SystemTime;
mod bit_patterns;
mod detect;
mod transform;
use anyhow::Result;

fn main() -> Result<()> {
    let img = image::open("assets/hide_image.png").unwrap();

    let start = SystemTime::now();

    let modified_img = transform::amplify_least_significant_bits(img.into());
    modified_img.save("output.png")?;

    let end = SystemTime::now();

    let duration = end.duration_since(start).unwrap();
    println!("it took {} seconds", duration.as_secs());

    Ok(())
}
