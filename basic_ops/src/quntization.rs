use image::{DynamicImage, ImageBuffer, Rgba};

use crate::luminance;

pub fn quantize(img: &DynamicImage, n: u8) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let mut grayscale_img = luminance::luminance(img);
    let t_1 = grayscale_img.pixels().min_by_key(|p| p[0]).unwrap()[0];
    let t_2 = grayscale_img.pixels().max_by_key(|p| p[0]).unwrap()[0];
    let tam_int = t_2 - t_1;

    if n <= tam_int {
        let tb: usize = usize::from(tam_int / n);
        let center: u8 = tam_int / (2 * n);
        let mut bin_vec: Vec<u8> = Vec::with_capacity(tb);

        for i in (t_1..t_2).step_by(tb) {
            bin_vec.push(i + center);
        }

        for pixel in grayscale_img.pixels_mut() {
            let bin_i = usize::from(pixel[0] - t_1) / tb;
            let res = bin_vec.get(bin_i);
            let quantized_value = match res {
                Some(value) => *value,
                None => *bin_vec.last().unwrap(),
            };

            *pixel = Rgba([quantized_value, quantized_value, quantized_value, pixel[3]]);
        }
    }

    grayscale_img
}
