use crate::bit_patterns::{eject, patterns};
use bitvec::field::BitField;
use image::RgbImage;
use itertools::izip;
use itertools::Itertools;
use std::fs;

fn chi_squared(n: f32, observed_count: f32, prob: f32) -> f32 {
    let expected_count = n * prob;
    let h = observed_count - expected_count;
    (h * h) / expected_count
}

fn block_frequencies(image: RgbImage) -> [f64; 4096] {
    // Read the LSB of each channel of each pixel of `image`.
    let least_significant_bits = eject(
        image.clone(),
        patterns::access_least_significant_bits,
        Some(image.len() * 3),
    );

    let pixels = least_significant_bits
        .chunks_exact(3)
        .map(BitField::load_le::<u16>);

    let width = image.width() as usize;
    // Create an iterator over every 2 by 2 block of pixels in `image`.
    let blocks = izip!(
        pixels.clone(),
        pixels.clone().skip(1),
        pixels.clone().skip(width),
        pixels.clone().skip(width + 1)
    );
    let blocks_count = blocks.clone().count() as f64;

    // Allocate space for the 4096 kinds of block there are, where each kind of block is represented
    // by a 12 bit number made by concatenating the bits of its component pixels.
    let mut block_counts = [0f64; 4096];
    for (w, x, y, z) in blocks {
        let block_id = w | x << 3 | y << 6 | z << 9;
        block_counts[block_id as usize] += 1f64;
    }

    // Convert the counts into frequencies.
    for block_id in 0..4096 {
        block_counts[block_id] /= blocks_count;
    }
    block_counts
}

fn mean_sd(data: impl IntoIterator<Item = f64> + Clone) -> (f64, f64) {
    let count = data.clone().into_iter().count() as f64;
    let mean = data.clone().into_iter().sum::<f64>() / count;
    let second_moment = data.into_iter().map(|x| x * x).sum::<f64>() / count;
    let variance = second_moment - (mean * mean);
    (mean, f64::sqrt(variance))
}

fn get_data() {
    let paths = fs::read_dir("assets/plain/").unwrap();

    let mut data: Vec<Vec<f64>> = vec![vec![]; 4096];

    // WARN outdated! Thihs file now stores the transpose of this data: the frequencies of each
    // block all together for each image.
    // Accumulate a vector of observed block frequencies for each kind of block.
    for path in paths {
        let image: RgbImage = image::open(path.unwrap().path()).unwrap().into();
        let block_frequencies = block_frequencies(image);
        for (block_id, frequency) in block_frequencies.into_iter().enumerate() {
            data[block_id].push(frequency);
        }
    }

    let data_string = serde_json::to_string(&data).unwrap();
    fs::write("plain_image_data.json", data_string).unwrap();
}

fn get_uniform_block_counts(block_counts: Vec<f64>) -> Vec<f64> {
    let uniform_block_ids: Vec<usize> = (0..8).map(|x| x | x << 3 | x << 6 | x << 9).collect();
    let uniform_block_counts: Vec<f64> = uniform_block_ids
        .into_iter()
        .map(|id| block_counts[id])
        .collect();
    uniform_block_counts
}

fn analyze_data() {
    let data_string = fs::read_to_string("assets/plain_image_data.json").unwrap();
    let data: Vec<Vec<f64>> = serde_json::from_str(data_string.as_str()).unwrap();

    let test_image: RgbImage = image::open("assets/hide_image-cropped.png").unwrap().into();
    let test_block_frequencies = block_frequencies(test_image);

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
        .map(get_uniform_block_counts)
        .map(|counts| {
            counts
                .into_iter()
                .enumerate()
                .sorted_by(|(_, x), (_, y)| y.partial_cmp(x).unwrap())
                .collect_vec()
        })
        .collect();

    println!("{:#?}", frequent_uniform_blocks);
    println!(
        "{:#?}",
        get_uniform_block_counts(test_block_frequencies.to_vec())
    );
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detect() {
        analyze_data();
    }
}
