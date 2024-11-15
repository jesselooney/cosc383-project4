use crate::extensions::RgbImageExt;
use crate::extract::{extract_bits, extract_image, Pattern};
use anyhow::{anyhow, Result};
use bitvec::prelude::*;
use image::*;
use std::fs;
use std::path::{Path, PathBuf};

/// Inserts a suffix between a file name's stem and its extension.
pub fn with_file_path_suffix(file_path: impl AsRef<Path>, suffix: &str) -> Option<PathBuf> {
    // Start with the stem of the old file name (no extension), then push on `suffix`.
    let mut new_file_name = file_path.as_ref().file_stem()?.to_os_string();
    new_file_name.push(suffix);
    // If there was an extension on the original file name, add it back.
    if let Some(extension) = file_path.as_ref().extension() {
        new_file_name.push(".");
        new_file_name.push(extension);
    }

    let new_file_path = file_path.as_ref().with_file_name(new_file_name);
    Some(new_file_path)
}

/// Saves the image extracted from the given file using `pattern` with a slightly altered name.
pub fn write_extracted_image(image_path: impl AsRef<Path>, pattern: impl Pattern) -> Result<()> {
    let image = RgbImage::open(image_path.as_ref())?;
    let extracted_image = extract_image(&image, pattern);
    let new_image_path = with_file_path_suffix(image_path.as_ref(), "-extract")
        .ok_or(anyhow!("`image_path` must be a file path"))?;
    extracted_image.save(new_image_path)?;
    Ok(())
}

/// Saves the result of amplifying the `index`-th bit of the given image with a slightly altered
/// name.
pub fn write_amplified_image(image_path: impl AsRef<Path>, index: usize) -> Result<()> {
    let image = RgbImage::open(image_path.as_ref())?;
    let amplified_image = amplify_image(&image, index);
    let new_image_path =
        with_file_path_suffix(image_path.as_ref(), format!("-amplify{}", index).as_str())
            .ok_or(anyhow!("`image_path` must be a file path"))?;
    amplified_image.save(new_image_path)?;
    Ok(())
}

/// Calls `write_amplified_image` on `image_path` with `index` = 0, 1, ..., 7.
pub fn write_amplified_images(image_path: impl AsRef<Path>) -> Result<()> {
    for index in 0..8 {
        write_amplified_image(image_path.as_ref(), index)?;
    }
    Ok(())
}

/// Saves every amplified form of every image in the source directory into the output directory
/// with appropriate names.
pub fn write_amplified_dir(
    source_dir_path: impl AsRef<Path>,
    output_dir_path: impl AsRef<Path>,
) -> Result<()> {
    let dir_entries = fs::read_dir(source_dir_path)?;

    for dir_entry in dir_entries {
        let image_path = dir_entry?.path();
        if !image_path.is_file() {
            continue;
        }
        println!("Amplifying {}", image_path.display());

        let image_name = image_path
            .file_name()
            .expect("`image_path` should have a file name");
        let mut amplified_image_base_path = output_dir_path.as_ref().to_path_buf();
        amplified_image_base_path.push(image_name);

        let image: RgbImage = image::open(&image_path)?.into();
        for index in 0..8 {
            let amplified_image_path = with_file_path_suffix(
                &amplified_image_base_path,
                format!("-amplify{}", index).as_str(),
            )
            .expect("`amplified_image_base_path` should have a file name");
            let amplified_image = amplify_image(&image, index);
            amplified_image.save(&amplified_image_path)?;

            println!("\tWrote {}", amplified_image_path.display());
        }
    }
    Ok(())
}

pub fn amplify_bits<O: BitOrder>(bits: &BitVec<u8, O>) -> BitVec<u8, O> {
    let mut amplified_bits: BitVec<u8, O> = BitVec::new();
    for bit in bits {
        amplified_bits.extend([*bit; 8]);
    }
    amplified_bits
}

pub fn amplify_image(image: &RgbImage, index: usize) -> RgbImage {
    let bits = extract_bits(image, |_, _, _, idx| idx == index);
    let amplified_bits = amplify_bits(&bits);
    let amplified_image: RgbImage = RgbImage::from_raw(
        image.width(),
        image.height(),
        amplified_bits.as_raw_slice().to_vec(),
    )
    .expect("The container of amplified bytes should be right-sized.");
    amplified_image
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_file_path_suffix() {
        let path = PathBuf::from("foo/bar.txt");
        assert_eq!(
            with_file_path_suffix(path, "baz.qux").unwrap(),
            PathBuf::from("foo/barbaz.qux.txt")
        );
    }

    #[test]
    fn test_write_file_path_suffix_no_extension() {
        let path = PathBuf::from("foo/bar");
        assert_eq!(
            with_file_path_suffix(&path, "baz.qux").unwrap(),
            PathBuf::from("foo/barbaz.qux")
        );
    }
}
