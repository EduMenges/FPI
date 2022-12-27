use image::{DynamicImage, GenericImage, GenericImageView};

pub fn rotate(img: DynamicImage, clockwise: bool) -> DynamicImage {
    let mut new_image = DynamicImage::new_rgba8(img.height(), img.width());

    for pixel in img.pixels() {
        new_image.put_pixel(
            calc_pos(pixel.1, new_image.width(), clockwise),
            calc_pos(pixel.0, new_image.height(), clockwise),
            pixel.2,
        );
    }

    new_image
}

#[inline]
fn calc_pos(og_pos: u32, boundary: u32, clockwise: bool) -> u32 {
    if clockwise {
        og_pos
    } else {
        boundary - og_pos - 1
    }
}
