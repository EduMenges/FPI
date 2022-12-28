use image::{DynamicImage, GenericImage, GenericImageView};

pub fn flip_vertical(img: &mut DynamicImage) {
    let (width, height) = img.dimensions();

    for h in 0..=(height / 2) {
        for w in 0..width {
            let opposite_pos = height - h - 1;

            let pixel_top = img.get_pixel(w, h);
            let pixel_bottom = img.get_pixel(w, opposite_pos);

            img.put_pixel(w, h, pixel_bottom);
            img.put_pixel(w, opposite_pos, pixel_top);
        }
    }
}

pub fn flip_horizontal(img: &mut DynamicImage) {
    let (width, height) = img.dimensions();

    for w in 0..=(width / 2) {
        for h in 0..height {
            let opposite_pos = width - w - 1;

            let pixel_left = img.get_pixel(w, h);
            let pixel_right = img.get_pixel(opposite_pos, h);

            img.put_pixel(w, h, pixel_right);
            img.put_pixel(opposite_pos, h, pixel_left);
        }
    }
}
