use image::{DynamicImage, GenericImage, GenericImageView};

pub fn gray_scale(img: &mut DynamicImage) {
    for y in 0..img.height() {
    for x in 0..img.width() {
            let pixel = img.get_pixel(x, y);
            let gray_pixel =
                (pixel[0] as f64 * 0.299 + pixel[1] as f64 * 0.587 + pixel[2] as f64 * 0.114) as u8;
            img.put_pixel(
                x,
                y,
                image::Rgba([gray_pixel, gray_pixel, gray_pixel, pixel[3]]),
            );
        }
    }
}
