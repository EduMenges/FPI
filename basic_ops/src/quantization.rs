use image::{DynamicImage, GenericImage, GenericImageView, Rgba};

pub fn quantize(img: &mut DynamicImage, n: u8) {
    let t_1 = img.pixels().min_by_key(|(_, _, p)| p[0]).unwrap().2[0];
    let t_2 = img.pixels().max_by_key(|(_, _, p)| p[0]).unwrap().2[0];

    let tam_int = t_2 - t_1;

    if n < tam_int {
        let tb: f64 = tam_int as f64 / n as f64;
        let center_offset: f64 = tam_int as f64 / (2 * n) as f64;
        let mut bin_vec: Vec<u8> = Vec::with_capacity(n as usize);

        for i in (t_1..t_2).step_by(tb.round() as usize) {
            bin_vec.push(i + center_offset.round() as u8);
        }

        for h in 0..img.height() {
            for w in 0..img.width() {
                let pixel = img.get_pixel(w, h);
                let bin_i = usize::from(pixel[0] - t_1) / tb.round() as usize;
                let res = bin_vec.get(bin_i);

                let quantized_value = match res {
                    Some(value) => *value,
                    None => *bin_vec.last().unwrap(),
                };

                let quantized_pixel =
                    Rgba([quantized_value, quantized_value, quantized_value, pixel[3]]);

                img.put_pixel(w, h, quantized_pixel);
            }
        }
    }
}
