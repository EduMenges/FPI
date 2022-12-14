use eframe::{run_native, NativeOptions};

use crate::app::*;

mod app;
mod file_loading;
mod histogram_graph;
mod image_adaptor;
pub mod image_decorator;
pub mod image_wrapper;
pub mod preview;
mod side_menu;

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
