use image::{DynamicImage, GenericImage, GenericImageView, Rgba, SubImage};

fn calculate_dimension(dimension: u32, factor: u8) -> u32 {
    (dimension as f64 / factor as f64).round() as u32
}

fn zoom_out_pixel(view: SubImage<&DynamicImage>, factors: (u8, u8)) -> Rgba<u8> {
    let mut computed = Rgba::<u16>([0, 0, 0, 0]);

    for x in 0..factors.0 as u32 {
        for y in 0..factors.1 as u32 {
            for i in 0..4_usize {
                computed.0[i] += view.get_pixel(x, y).0[i] as u16;
            }
        }
    }

    let mut out = Rgba::<u8>([0_u8; 4]);

    for i in 0..computed.0.len() {
        computed.0[i] /= factors.0 as u16 * factors.1 as u16;
        out.0[i] = computed.0[i] as u8;
    }

    out
}

pub fn adjust_dimension(dimension: u32, factor: u8, boundary: u32) -> (u32, u32) {
    let coordinate = dimension * factor as u32;
    if coordinate + factor as u32 >= boundary {
        (coordinate, boundary - coordinate)
    } else {
        (coordinate, factor as u32)
    }
}

pub fn zoom_out(img: DynamicImage, factors: (u8, u8)) -> DynamicImage {
    let w = calculate_dimension(img.width(), factors.0);
    let h = calculate_dimension(img.height(), factors.1);
    let mut new_img = DynamicImage::new_rgba8(w, h);

    for x in 0..new_img.width() {
        let (next_x, factor_x) = adjust_dimension(x, factors.0, img.width());

        for y in 0..new_img.height() {
            let (next_y, factor_y) = adjust_dimension(y, factors.1, img.height());

            let view = img.view(next_x, next_y, factor_x, factor_y);

            new_img.put_pixel(x, y, zoom_out_pixel(view, (factor_x as u8, factor_y as u8)));
        }
    }

    new_img
}

pub fn zoom_in(img: DynamicImage) -> DynamicImage {
    let mut new_img = DynamicImage::new_rgba8(img.width() * 2 - 1, img.height() * 2 - 1);

    for pixel in img.pixels() {
        new_img.put_pixel(pixel.0 * 2, pixel.1 * 2, pixel.2);
    }

    for y in (0..new_img.height()).step_by(2) {
        fill_row(&mut new_img, y);
    }

    for x in (0..new_img.width()).step_by(2) {
        fill_column(&mut new_img, x);
    }

    for y in (1..new_img.height()).step_by(2) {
        fill_row(&mut new_img, y);
    }

    for x in (1..new_img.width()).step_by(2) {
        fill_column(&mut new_img, x);
    }

    new_img
}

fn fill_row(img: &mut DynamicImage, y: u32) {
    for i in (1..img.width()).step_by(2) {
        fill_pixel(img, (i, y), (1, 0));
    }
}

fn fill_column(img: &mut DynamicImage, x: u32) {
    for i in (1..img.height()).step_by(2) {
        fill_pixel(img, (x, i), (0, 1));
    }
}

fn fill_pixel(img: &mut DynamicImage, (x, y): (u32, u32), incr: (u32, u32)) {
    let pixel_1 = img.get_pixel(x - incr.0, y - incr.1);
    let pixel_2 = img.get_pixel(x + incr.0, y + incr.1);
    let average = average_pixel(&pixel_1, &pixel_2);
    img.put_pixel(x, y, average);
}

pub fn average_pixel(pixel_1: &Rgba<u8>, pixel_2: &Rgba<u8>) -> Rgba<u8> {
    let mut computed = Rgba::<f64>([0.0; 4]);
    let mut out = Rgba::<u8>([0_u8; 4]);

    for i in 0..4 {
        computed.0[i] = (pixel_1.0[i] as f64 + pixel_2.0[i] as f64) / 2.0;
        out.0[i] = computed.0[i].round() as u8;
    }

    out
}
