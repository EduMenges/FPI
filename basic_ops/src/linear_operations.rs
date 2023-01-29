use image::{DynamicImage, GenericImage, GenericImageView, Rgba};

pub fn linear_template_function<F>(img: &mut DynamicImage, func: F)
where
    F: Fn(Rgba<u8>) -> Rgba<u8>,
{
    for y in 0..img.height() {
        for x in 0..img.width() {
            let mut pixel = img.get_pixel(x, y);
            pixel = func(pixel);
            img.put_pixel(x, y, pixel);
        }
    }
}

pub fn adjust_brightness(img: &mut DynamicImage, n: i16) {
    linear_template_function(img, |mut pixel| {
        for i in 0..3_usize {
            if n > 0 {
                pixel[i] = pixel[i].saturating_add(n as u8);
            } else {
                pixel[i] = pixel[i].saturating_sub(-n as u8);
            }
        }
        pixel
    });
}

pub fn adjust_contrast(img: &mut DynamicImage, n: f64) {
    linear_template_function(img, |mut pixel| {
        for i in 0..3_usize {
            let in_f64 = (pixel[i] as f64 * n).round();
            pixel[i] = if in_f64 > u8::MAX as f64 {
                u8::MAX
            } else if in_f64 < u8::MIN as f64 {
                u8::MIN
            } else {
                in_f64 as u8
            };
        }
        pixel
    });
}

pub fn negative(img: &mut DynamicImage) {
    linear_template_function(img, |mut pixel| {
        for i in 0..3_usize {
            pixel[i] = 255u8.saturating_sub(pixel[i]);
        }
        pixel
    })
}
