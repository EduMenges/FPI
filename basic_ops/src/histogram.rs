use image::{DynamicImage, GenericImage, GenericImageView, GrayAlphaImage};

use crate::{linear_operations::linear_template_function};

pub type Histogram = [u32; 256];

pub fn calculate_histogram(img: &GrayAlphaImage) -> Histogram {
    let mut out: [u32; 256] = [0; 256];

    for pixel in img.pixels() {
        out[pixel.0[0] as usize] += 1;
    }

    out
}

pub fn cumulative_histogram(histogram: &Histogram) -> Histogram {
    let mut out: [u32; 256] = [0; 256];

    out[0] = histogram[0];
    for i in 1..256 {
        out[i] = out[i - 1] + histogram[i];
    }

    out
}

pub fn normalize_histogram(histogram: &Histogram) -> Histogram {
    let highest = *histogram.iter().max().unwrap();
    let mut out: [u32; 256] = [0; 256];

    for i in 0..out.len() {
        out[i] = histogram[i] * 255 / highest;
    }

    out
}

#[inline]
pub fn normalized_cumulative(img: &DynamicImage) -> Histogram {
    normalize_histogram(&cumulative_histogram(&calculate_histogram(
        &img.to_luma_alpha8(),
    )))
}

pub fn equalize_histogram(img: &mut DynamicImage) {
    let luminized = img.to_luma_alpha8();

    let histogram = normalize_histogram(&cumulative_histogram(&calculate_histogram(&luminized)));

    linear_template_function(img, |mut pixel| {
        for i in 0..3_usize {
            pixel[i] = histogram[pixel[i] as usize] as u8;
        }

        pixel
    })
}

pub fn matching(img: &mut DynamicImage, source: &DynamicImage) {
    let nch_img = normalized_cumulative(img);
    let nch_source = normalized_cumulative(source);

    let mut hm: [u8; 256] = [Default::default(); 256];

    for (index, shade) in nch_img.iter().enumerate() {
        hm[index] = nch_img[find_matching_val(shade, &nch_source)] as u8;
    }

    for y in 0..img.height() {
        for x in 0..img.width() {
            let mut pixel = img.get_pixel(x, y);

            for i in 0..3 {
                pixel.0[i] = hm[pixel.0[i] as usize];
            }

            img.put_pixel(x, y, pixel);
        }
    }
}

fn find_matching_val(val: &u32, histogram: &[u32; 256]) -> usize {
    let ans = histogram.iter().enumerate().find(|(_, hist)| val == *hist);

    match ans {
        Some((index, _)) => index,
        None => 0,
    }
}
