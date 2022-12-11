use image::{DynamicImage, Rgba};

fn flip_vertical(img: &DynamicImage) -> image::ImageBuffer<Rgba<u8>, Vec<u8>>{
    let mut img_slice = img.to_rgba8();
    let width = usize::try_from(img_slice.width()).unwrap();
    let height = usize::try_from(img_slice.height()).unwrap();

    for i in 0..height / 2 {
        for j in 0..width {
            img_slice.swap(i + j, width + height - i - j);
        }
    }

    img_slice
}

fn flip_horizontal(img: &DynamicImage) -> image::ImageBuffer<Rgba<u8>, Vec<u8>>{
    let mut img_slice = img.to_rgba8();
    let width = usize::try_from(img_slice.width()).unwrap();
    let height = usize::try_from(img_slice.height()).unwrap();
    
    for i in 0..width / 2 {
        for j in 0..height {
            img_slice.swap(i + j, width + height - i - j);
        }
    }

    img_slice
}
