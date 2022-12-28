use once_cell::sync::Lazy;

pub const KERNEL_SIZE: usize = 3;
pub type Kernel = [[f32; KERNEL_SIZE]; KERNEL_SIZE];

#[derive(Default, Clone)]
pub struct KernelWrp {
    pub kernel: Kernel,
    name: String,
    rotated: Kernel,
}

impl KernelWrp {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn new(kernel: Kernel, name: String) -> Self {
        let mut rotated = kernel;
        rotate_kernel(&mut rotated);
        
        Self {
            kernel,
            name,
            rotated,
        }
    }

    pub const fn rotated(&self) -> Kernel {
        self.rotated
    }

    pub fn update_rotated(&mut self) {
        let mut new_rotated = self.kernel;
        rotate_kernel(&mut new_rotated);
        self.rotated = new_rotated;
    }
}

pub static GAUSSIAN_FILTER: Lazy<KernelWrp> = Lazy::new(|| {
    KernelWrp::new(
        [
            [0.0625, 0.125, 0.0625],
            [0.125, 0.25, 0.125],
            [0.0625, 0.125, 0.0625],
        ],
        "Gaussian".to_owned(),
    )
});

pub static LAPLACIAN_FILTER: Lazy<KernelWrp> = Lazy::new(|| {
    KernelWrp::new(
        [[0.0, -1.0, 0.0], [-1.0, 4.0, -1.0], [0.0, -1.0, 0.0]],
        "Laplacian".to_owned(),
    )
});

pub static HIGH_PASS: Lazy<KernelWrp> = Lazy::new(|| {
    KernelWrp::new(
        [[-1.0, -1.0, -1.0], [-1.0, 8.0, -1.0], [-1.0, -1.0, -1.0]],
        "High pass".to_owned(),
    )
});

pub static PREWITT_HX: Lazy<KernelWrp> = Lazy::new(|| {
    KernelWrp::new(
        [[-1.0, 0.0, 1.0], [-1.0, 0.0, 1.0], [-1.0, 0.0, 1.0]],
        "Prewitt Hx".to_owned(),
    )
});

pub static PREWITT_HY: Lazy<KernelWrp> = Lazy::new(|| {
    KernelWrp::new(
        [[-1.0, -1.0, -1.0], [0.0, 0.0, 0.0], [1.0, 1.0, 1.0]],
        "Prewitt Hy".to_owned(),
    )
});

pub static SOBEL_HX: Lazy<KernelWrp> = Lazy::new(|| {
    KernelWrp::new(
        [[-1.0, 0.0, 1.0], [-2.0, 0.0, 2.0], [-1.0, 0.0, 1.0]],
        "Prewitt Hx".to_owned(),
    )
});

pub static SOBEL_HY: Lazy<KernelWrp> = Lazy::new(|| {
    KernelWrp::new(
        [[-1.0, -2.0, -1.0], [0.0, 0.0, 0.0], [1.0, 2.0, 1.0]],
        "Prewitt Hy".to_owned(),
    )
});

pub static ALL_FILTERS: Lazy<[&Lazy<KernelWrp>; 7]> = Lazy::new(|| {
    [&GAUSSIAN_FILTER, &LAPLACIAN_FILTER, &HIGH_PASS, &PREWITT_HX, &PREWITT_HY, &SOBEL_HX, &SOBEL_HY]
});

pub fn rotate_kernel(kernel: &mut Kernel) {
    kernel.swap(0, 2);
    for row in kernel {
        row.swap(0, 2);
    }
}
