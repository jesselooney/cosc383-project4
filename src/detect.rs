use crate::bit_patterns::{eject, patterns};
use bitvec::prelude::*;
use image::RgbImage;
use itertools::izip;
use plotters::prelude::*;

fn detect() {
    let image: RgbImage = image::open("assets/hide_image.png").unwrap().into();
    let length = image.len() as usize;
    let width = image.width() as usize;
    let lsbs = eject(
        image,
        patterns::access_least_significant_bits,
        Some(length * 3),
    );

    let pixels = lsbs.chunks_exact(3).map(|bs| bs.load::<u16>());

    let blocks = izip!(
        pixels.clone(),
        pixels.clone().skip(1),
        pixels.clone().skip(width),
        pixels.clone().skip(width + 1)
    );

    let mut nums = [0i32; 4096]; // each block of 4 pixels' lsbs is 12 bits so has values in [0, 4095]
    let block_count = blocks
        .map(|(a, b, c, d)| a | b << 3 | c << 6 | d << 9)
        .map(|x| nums[x as usize] += 1)
        .count();

    let data: Vec<(i32, i32)> = nums
        .into_iter()
        .enumerate()
        .map(|(i, c)| (i as i32, c))
        .collect();

    let frequent_indices: Vec<(i32, i32)> =
        data.clone().into_iter().filter(|(_, c)| *c > 700).collect();

    let drawing_area = SVGBackend::new("histogram_vertical.svg", (1000, 1000)).into_drawing_area();
    drawing_area.fill(&WHITE).unwrap();
    let mut chart_builder = ChartBuilder::on(&drawing_area);
    chart_builder
        .margin(5)
        .set_left_and_bottom_label_area_size(20);
    let mut chart_context = chart_builder
        .build_cartesian_2d((0..100).into_segmented(), 0..1000)
        .unwrap();
    chart_context.configure_mesh().draw().unwrap();
    chart_context
        .draw_series(
            Histogram::vertical(&chart_context)
                .style(BLUE.filled())
                .margin(10)
                .data(data),
        )
        .unwrap();

    println!("{:?}", frequent_indices);
    println!("{block_count}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detect_works() {
        detect();
    }
}
