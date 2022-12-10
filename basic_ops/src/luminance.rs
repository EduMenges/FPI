use image::{DynamicImage, Rgba, ImageBuffer};

pub fn luminance(img: &DynamicImage) -> ImageBuffer<Rgba<u8>, Vec<u8>>{
    let mut new_image = img.to_rgba8();

    for pixel in new_image.pixels_mut() {
        let new_value = (f64::from(pixel[0]) * 0.299 + f64::from(pixel[1]) * 0.587 + f64::from(pixel[2]) * 0.114) as u8;
        *pixel = Rgba([new_value, new_value, new_value, pixel[3]]);
    }

    new_image
}