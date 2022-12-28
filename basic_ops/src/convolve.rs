use crate::filters::KernelWrp;
use image::{DynamicImage, GenericImageView, Rgba, Rgba32FImage, SubImage};

use crate::filters::{Kernel, KERNEL_SIZE};

pub fn convolve(img: DynamicImage, kernel: &KernelWrp) -> DynamicImage {
    let in_f32 = img.into_rgba32f();
    let mut out = in_f32.clone();
    let rotated: Kernel = kernel.rotated();
    let needs_clamping = kernel.needs_clamping();

    for y in 1..in_f32.height() - 1 {
        for x in 1..in_f32.width() - 1 {
            let view = in_f32.view(x - 1, y - 1, KERNEL_SIZE as u32, KERNEL_SIZE as u32);

            let mut new_pixel = convolve_pixel(view, &rotated);

            if needs_clamping {
                for i in 0..3 {
                    new_pixel.0[i] += 0.5;
                }
            }

            out.put_pixel(x, y, new_pixel);
        }
    }

    DynamicImage::ImageRgba32F(out)
}

fn convolve_pixel(view: SubImage<&Rgba32FImage>, kernel: &Kernel) -> Rgba<f32> {
    let mut out = Rgba::<f32>([0.0; 4]);

    kernel.iter().enumerate().for_each(|(y, row)| {
        row.iter().enumerate().for_each(|(x, kernel_val)| {
            let pixel = view.get_pixel(x as u32, y as u32);

            for c in 0..3 {
                out.0[c] += pixel.0[c] * kernel_val;
            }
        });
    });

    out.0[3] = view.get_pixel(1, 1).0[3];
    out
}
