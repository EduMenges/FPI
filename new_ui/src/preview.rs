use egui::{Align, Layout, Ui, Window};

use crate::{image_decorator::ImageDecorator, image_wrapper::ImageWrapper};

pub struct Preview {
    og_image: ImageDecorator,
    new_image: ImageDecorator,
    open: bool,
}

impl Preview {
    pub fn new(og_image: ImageDecorator, new_image: ImageDecorator) -> Self {
        Self {
            og_image,
            new_image,
            open: true,
        }
    }

    pub fn ui(&mut self, ui: &mut Ui) -> Option<ImageWrapper> {
        let mut out: Option<ImageWrapper> = None;

        Window::new("Preview")
            .open(&mut self.open)
            .show(ui.ctx(), |ui| {
                ui.with_layout(Layout::top_down(Align::Center), |ui| {
                    ui.horizontal(|ui| {
                        self.og_image.ui(ui);
                        self.new_image.ui(ui);
                    });

                    if ui.button("Apply").clicked() {
                        out = Some(self.new_image.wrapper.to_owned());
                    }
                });
            });

        out
    }
}
