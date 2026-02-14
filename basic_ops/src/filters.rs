use std::sync::LazyLock;

pub const KERNEL_SIZE: usize = 3;
pub type RawKernel = [[f32; KERNEL_SIZE]; KERNEL_SIZE];

#[derive(Default, Clone)]
pub struct Kernel {
    pub kernel: RawKernel,
    name: String,
    rotated: RawKernel,
}

impl Kernel {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn new(kernel: RawKernel, name: String) -> Self {
        let mut rotated = kernel;
        rotate_kernel(&mut rotated);

        Self {
            kernel,
            name,
            rotated,
        }
    }

    pub const fn rotated(&self) -> RawKernel {
        self.rotated
    }

    pub fn update_rotated(&mut self) {
        let mut new_rotated = self.kernel;
        rotate_kernel(&mut new_rotated);
        self.rotated = new_rotated;
    }

    pub fn needs_clamping(&self) -> bool {
        self.kernel
            .iter()
            .any(|row| row.iter().any(|val| *val < 0.0))
    }
}

pub static GAUSSIAN_FILTER: LazyLock<Kernel> = LazyLock::new(|| {
    Kernel::new(
        [
            [0.0625, 0.125, 0.0625],
            [0.125, 0.25, 0.125],
            [0.0625, 0.125, 0.0625],
        ],
        "Gaussian".to_owned(),
    )
});

pub static LAPLACIAN_FILTER: LazyLock<Kernel> = LazyLock::new(|| {
    Kernel::new(
        [
            [0.0, -1.0, 0.0],  //
            [-1.0, 4.0, -1.0], //
            [0.0, -1.0, 0.0],
        ],
        "Laplacian".to_owned(),
    )
});

pub static HIGH_PASS: LazyLock<Kernel> = LazyLock::new(|| {
    Kernel::new(
        [
            [-1.0, -1.0, -1.0], //
            [-1.0, 8.0, -1.0],  //
            [-1.0, -1.0, -1.0],
        ],
        "High pass".to_owned(),
    )
});

pub static PREWITT_HX: LazyLock<Kernel> = LazyLock::new(|| {
    Kernel::new(
        [
            [-1.0, 0.0, 1.0], //
            [-1.0, 0.0, 1.0], //
            [-1.0, 0.0, 1.0],
        ],
        "Prewitt Hx".to_owned(),
    )
});

pub static PREWITT_HY: LazyLock<Kernel> = LazyLock::new(|| {
    Kernel::new(
        [
            [-1.0, -1.0, -1.0], //
            [0.0, 0.0, 0.0],    //
            [1.0, 1.0, 1.0],
        ],
        "Prewitt Hy".to_owned(),
    )
});

pub static SOBEL_HX: LazyLock<Kernel> = LazyLock::new(|| {
    Kernel::new(
        [[-1.0, 0.0, 1.0], [-2.0, 0.0, 2.0], [-1.0, 0.0, 1.0]],
        "Sobel Hx".to_owned(),
    )
});

pub static SOBEL_HY: LazyLock<Kernel> = LazyLock::new(|| {
    Kernel::new(
        [[-1.0, -2.0, -1.0], [0.0, 0.0, 0.0], [1.0, 2.0, 1.0]],
        "Sobel Hy".to_owned(),
    )
});

pub static ALL_FILTERS: LazyLock<[&LazyLock<Kernel>; 7]> = LazyLock::new(|| {
    [
        &GAUSSIAN_FILTER,
        &LAPLACIAN_FILTER,
        &HIGH_PASS,
        &PREWITT_HX,
        &PREWITT_HY,
        &SOBEL_HX,
        &SOBEL_HY,
    ]
});

fn rotate_kernel(kernel: &mut RawKernel) {
    kernel.swap(0, 2);
    for row in kernel {
        row.swap(0, 2);
    }
}
