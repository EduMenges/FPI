use image::{DynamicImage, GenericImage, GenericImageView, Rgba};

pub fn linear_template_function<F>(img: &mut DynamicImage, func: F)
where
    F: Fn(Rgba<u8>) -> Rgba<u8>,
{
    for x in 0..img.width() {
        for y in 0..img.height() {
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

pub fn adjust_contrast(img: &mut DynamicImage, n: u8) {
    linear_template_function(img, |mut pixel| {
        for i in 0..3_usize {
            pixel[i] = pixel[i].saturating_mul(n);
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
