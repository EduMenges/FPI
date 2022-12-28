use image::{DynamicImage, GenericImage, GenericImageView};

pub fn rotate(img: DynamicImage, clockwise: bool) -> DynamicImage {
    let mut new_image = DynamicImage::new_rgba8(img.height(), img.width());

    let matrix = if clockwise {
        CLOCKWISE_MATRIX
    } else {
        COUNTER_MATRIX
    };

    let dimensions = new_image.dimensions();

    for pixel in img.pixels() {
        let coords = calc_pos(pixel.0, pixel.1, matrix, dimensions);
        new_image.put_pixel(coords[0], coords[1], pixel.2);
    }

    new_image
}

type RotationMatrix = [[i64; 2]; 2];

const CLOCKWISE_MATRIX: RotationMatrix = [[0, -1], [1, 0]];
const COUNTER_MATRIX: RotationMatrix = [[0, 1], [-1, 0]];

fn calc_pos(x: u32, y: u32, rot: RotationMatrix, dimensions: (u32, u32)) -> [u32; 2] {
    let mut new_x = x as i64 * rot[0][0] + y as i64 * rot[0][1];
    let mut new_y = x as i64 * rot[1][0] + y as i64 * rot[1][1];

    if new_x < 0 {
        new_x += dimensions.0 as i64
    }
    if new_y < 0 {
        new_y += dimensions.1 as i64
    }

    [new_x as u32, new_y as u32]
}
