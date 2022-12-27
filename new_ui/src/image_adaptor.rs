use egui::ColorImage;
use image::DynamicImage;

pub fn load_image_from_memory(image: &DynamicImage) -> ColorImage {
    let size = [image.width() as _, image.height() as _];
    let image_buffer = image.to_rgba8();
    let pixels = image_buffer.as_flat_samples();

    ColorImage::from_rgba_unmultiplied(
        size,
        pixels.as_slice(),
    )
}