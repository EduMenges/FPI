use eframe::{run_native, NativeOptions};

use crate::app::*;

mod app;
mod histogram_graph;
mod image_adaptor;
pub mod image_wrapper;
mod side_menu;
pub mod image_decorator;

fn main() {
    let win_options = NativeOptions {
        maximized: true,
        ..Default::default()
    };

    run_native(
        "PhotoMenges",
        win_options,
        Box::new(|cc| Box::new(PhotoMenges::new(cc))),
    );
}
