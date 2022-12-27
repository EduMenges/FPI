use image::{
    DynamicImage, GenericImageView, Rgba, Rgba32FImage,
};

pub const KERNEL_SIZE: usize = 3;
pub type Kernel = [[f32; KERNEL_SIZE]; KERNEL_SIZE];

pub fn convolve(img: DynamicImage, kernel: &Kernel) -> DynamicImage {
    let in_f32 = img.into_rgba32f();
    let mut out = in_f32.clone();
    let mut rotated: Kernel = *kernel;
    rotate_kernel(&mut rotated);

    for y in 1..in_f32.height() - 1 {
        for x in 1..in_f32.width() - 1 {
            let new_pixel = convolve_pixel(
                in_f32.view(x - 1, y - 1, KERNEL_SIZE as u32, KERNEL_SIZE as u32).inner(),
                &rotated,
            );
            out.put_pixel(x, y, new_pixel);
        }
    }

    DynamicImage::ImageRgba32F(out)
}

fn convolve_pixel(view: &Rgba32FImage, kernel: &Kernel) -> Rgba<f32> {
    let mut out = Rgba::<f32>([0.0, 0.0, 0.0, 0.0]);
    for y in 0..KERNEL_SIZE {
        kernel.iter().enumerate().for_each(|(x, kernel)| {
            let pixel = view.get_pixel(x as u32, y as u32).0;
            pixel.iter().enumerate().for_each(|(pos, val)| {
                out.0[pos] += val * kernel[x];
            });
        })
    }
    out
}

pub fn rotate_kernel(kernel: &mut Kernel) {
    kernel.swap(0, 2);
    for row in kernel {
        row.swap(0, 2);
    }
}
