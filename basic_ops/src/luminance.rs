use image::{DynamicImage, GenericImage, GenericImageView, GrayAlphaImage};

pub fn gray_scale(img: &mut DynamicImage) {
    match img {
        DynamicImage::ImageLuma8(_) => {}
        DynamicImage::ImageLumaA8(_) => {}
        DynamicImage::ImageLuma16(_) => {}
        DynamicImage::ImageLumaA16(_) => {}
        _ => {
            let mut grayed = GrayAlphaImage::new(img.width(), img.height());

            for y in 0..img.height() {
                for x in 0..img.width() {
                    let pixel = img.get_pixel(x, y);
                    let gray_pixel = (pixel[0] as f64 * 0.299
                        + pixel[1] as f64 * 0.587
                        + pixel[2] as f64 * 0.114)
                        .round() as u8;
                    grayed.put_pixel(x, y, image::LumaA([gray_pixel, pixel[3]]));
                }
            }
            
            *img = DynamicImage::ImageLumaA8(grayed);
        }
    }
}
