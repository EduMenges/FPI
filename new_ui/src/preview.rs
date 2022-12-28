use egui::{Align, Layout, Separator, Ui, Window};
use egui_extras::{Size, StripBuilder};

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
            .resizable(true)
            .show(ui.ctx(), |ui| {
                StripBuilder::new(ui)
                    .size(Size::relative(0.5))
                    .size(Size::exact(3.0))
                    .size(Size::remainder())
                    .cell_layout(Layout::bottom_up(Align::Max))
                    .horizontal(|mut strip| {
                        strip.strip(|builder| {
                            builder
                                .size(Size::remainder().at_most(800.0))
                                .size(Size::exact(10.0))
                                .vertical(|mut strip| {
                                    strip.cell(|ui| {
                                        self.og_image.ui(ui);
                                    });
                                    strip.empty();
                                });
                        });
                        strip.cell(|ui| {
                            ui.add(Separator::default().vertical());
                        });
                        strip.strip(|builder| {
                            builder
                                .size(Size::remainder().at_most(800.0))
                                .size(Size::exact(10.0))
                                .vertical(|mut strip| {
                                    strip.cell(|ui| {
                                        self.new_image.ui(ui);
                                    });
                                    strip.cell(|ui| {
                                        if ui.button("Apply").clicked() {
                                            out = Some(self.new_image.wrapper.to_owned());
                                        }
                                    });
                                });
                        });
                    });
            });

        out
    }
}
