use image::{DynamicImage, GenericImageView};

use crate::{linear_operations::linear_template_function, luminance::gray_scale};

pub fn calculate_histogram(img: &DynamicImage) -> [u32; 256] {
    let mut out: [u32; 256] = [0; 256];
    for (_, _, p) in img.pixels() {
        out[p[0] as usize] += 1;
    }
    out
}

pub fn cumulative_histogram(histogram: &[u32; 256]) -> [u32; 256] {
    let mut out: [u32; 256] = [0; 256];
    out[0] = histogram[0];
    for i in 1..256 {
        out[i] = out[i - 1] + histogram[i];
    }
    out
}

pub fn normalize_histogram(histogram: [u32; 256]) -> [u32; 256] {
    let highest = *histogram.iter().max().unwrap();
    let mut out: [u32; 256] = [0; 256];

    for i in 0..out.len() {
        out[i] = histogram[i] * 255 / highest;
    }

    out
}

pub fn equalize_histogram(img: &mut DynamicImage, in_color: bool) {
    let histogram = normalize_histogram(if in_color {
        cumulative_histogram(&calculate_histogram(img))
    } else {
        let mut luminized = img.clone();
        gray_scale(&mut luminized);
        cumulative_histogram(&calculate_histogram(&luminized))
    });

    linear_template_function(img, |mut pixel| {
        for i in 0..3_usize {
            pixel[i] = histogram[pixel[i] as usize] as u8;
        }
        pixel
    })
}
