use crate::bit_patterns::{eject, patterns};
use bitvec::field::BitField;
use image::RgbImage;
use itertools::izip;
use std::fs;

/// Compute the frequencies of every possible arrangement of the least-significant bits of all the
/// two-by-two squares of pixels in `image`.
pub fn compute_lsb_square_frequencies(image: RgbImage) -> [f64; 4096] {
    // Read the LSB of each channel of each pixel of `image`. The number of such bits should be
    // equal to thrice the number of pixels in the image.
    let least_significant_bits = eject(
        image.clone(),
        patterns::access_least_significant_bits,
        Some(image.len() * 3),
    );

    // Map each group of three LSBs (corresponding to the three channels of one pixel) to one
    // integer representing the "color" of that "LSB pixel".
    let lsb_pixels = least_significant_bits
        .chunks_exact(3)
        .map(BitField::load_le::<u16>);

    let image_width = image.width() as usize;
    // Create an iterator over every two-by-two square of LSB pixels.
    let lsb_squares = izip!(
        lsb_pixels.clone(),
        lsb_pixels.clone().skip(1),
        lsb_pixels.clone().skip(image_width),
        lsb_pixels.clone().skip(image_width + 1)
    );

    let lsb_squares_count = lsb_squares.clone().count() as f64;

    // Allocate space for the 4096 kinds of LSB square there are, where each kind of square is
    // represented by a 12 bit number made by concatenating the three bits of ech of its component
    // LSB pixels.
    let mut lsb_square_frequencies = [0f64; 4096];
    // Count the number of each kind of LSB square in the image.
    for (p1, p2, p3, p4) in lsb_squares {
        // Compute the unique 12 bit number representing the kind of square this is based on the
        // colors of the component LSB pixels.
        let square_kind = p1 | p2 << 3 | p3 << 6 | p4 << 9;
        lsb_square_frequencies[square_kind as usize] += 1f64;
    }
    // Convert the counts into frequencies by dividing by the total number of LSB squares.
    for frequency in &mut lsb_square_frequencies {
        *frequency /= lsb_squares_count;
    }

    lsb_square_frequencies
}

/// From an array of LSB square frequencies, select the frequencies of the seven kinds of
/// monochrome LSB square.
pub fn monochrome_frequencies(lsb_square_frequencies: [f64; 4096]) -> [f64; 8] {
    let mut monochrome_lsb_square_frequencies = [0f64; 8];
    // For each possible color of a three-bit "LSB pixel", compute the number representing the kind
    // of LSB square where each component pixel has that same color.
    for (color, frequency) in &mut monochrome_lsb_square_frequencies.iter_mut().enumerate() {
        let monochrome_square_kind = color | color << 3 | color << 6 | color << 9;
        *frequency = lsb_square_frequencies[monochrome_square_kind];
    }
    monochrome_lsb_square_frequencies
}

/// WIP - the intent is to compute the monochrome frequencies on smaller regions of images to see
/// how small we can go before the patterns I've observed break down. This will determine how small
/// a message we can detect with the statistical approach.
fn test_block_size(width: u64, height: u64, samples_per_image: u64) {
    let paths = fs::read_dir("assets/plain/").unwrap();
    for path in paths {
        let image: RgbImage = image::open(path.unwrap().path()).unwrap().into();
        for _ in 0..samples_per_image {
            // let subimage: RgbImage = ();
        }
    }
}

/// This function was used to compute the lsb square frequencies on a bunch of unmodified test
/// images and store them in a file for later use.
fn get_test_data() {
    let paths = fs::read_dir("assets/plain/").unwrap();

    let mut data: Vec<Vec<f64>> = vec![vec![]; 4096];

    // WARN outdated! This file now stores the transpose of this data: the frequencies of each
    // block all together for each image.
    // Accumulate a vector of observed block frequencies for each kind of block.
    for path in paths {
        let image: RgbImage = image::open(path.unwrap().path()).unwrap().into();
        let lsb_square_frequencies = compute_lsb_square_frequencies(image);
        for (square_kind, frequency) in lsb_square_frequencies.into_iter().enumerate() {
            data[square_kind].push(frequency);
        }
    }

    let data_string = serde_json::to_string(&data).unwrap();
    fs::write("plain_image_data.json", data_string).unwrap();
}

// This function is mostly scratch work to find patterns in the lsb square frequencies data.
// It no longer compiles because I broke the previous API.
/*
fn analyze_data() {
    let data_string = fs::read_to_string("assets/plain_image_data.json").unwrap();
    let data: Vec<Vec<f64>> = serde_json::from_str(data_string.as_str()).unwrap();

    let test_image: RgbImage = image::open("assets/hide_image-cropped.png").unwrap().into();
    let test_block_frequencies = compute_lsb_square_frequencies(test_image);

    /*
    // Transpose the data
    let data: Vec<Vec<f64>> = (0..data_transpose[0].len())
        .map(|observation_index| {
            data_transpose
                .clone()
                .into_iter()
                .map(|observations| observations[observation_index])
                .collect()
        })
        .collect();*/

    assert!(data.len() == 42);
    assert!(data[0].len() == 4096);

    let frequent_uniform_blocks: Vec<Vec<(usize, f64)>> = data
        .into_iter()
        .map(monochrome_frequencies)
        .map(|counts| {
            counts
                .into_iter()
                .enumerate()
                .sorted_by(|(_, x), (_, y)| y.partial_cmp(x).unwrap())
                .collect_vec()
        })
        .collect();

    println!("{:#?}", frequent_uniform_blocks);
    println!("{:#?}", monochrome_frequencies(test_block_frequencies));
    println!("The uniform frequencies are of one tenth the order in the modified image versus baseline. This should be detectable!");
    // Collect all block ids where each pixel (3 bits) has the same value.

    /*
    let stats: Vec<(f64, f64)> = data.into_iter().map(mean_sd).collect();
    let relative_deviation: Vec<(usize, f64)> = stats
        .into_iter()
        .map(|(mean, sd)| sd / mean)
        .enumerate()
        .sorted_by(|(_, x), (_, y)| x.partial_cmp(y).unwrap())
        .filter(|(_, x)| *x < 0.3)
        .collect();*/
    //println!("{:#?}", relative_deviation);
}
*/

#[cfg(test)]
mod tests {
    #[test]
    fn detect() {}
}
