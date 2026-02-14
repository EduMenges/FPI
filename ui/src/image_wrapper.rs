use egui::{Context, TextureHandle, Ui};
use image::DynamicImage;

use crate::image_adaptor::load_image_from_memory;

#[derive(Clone)]
pub struct ImageWrapper {
    pub img: DynamicImage,
    texture: Option<egui::TextureHandle>,
    name: String,
    title: String,
}

impl ImageWrapper {
    pub fn new(img: DynamicImage, name: String, title: String, ctx: &Context) -> Self {
        let texture =
            Some(ctx.load_texture(&name, load_image_from_memory(&img), Default::default()));
        Self {
            img,
            texture,
            name,
            title,
        }
    }

    pub fn reset(&mut self) {
        self.texture = None;
    }

    pub fn ui(&mut self, ui: &mut Ui) {
        ui.vertical(|ui| {
            ui.label(&self.title);

            egui::ScrollArea::new([true, true])
                .id_source(self.name().to_owned())
                .auto_shrink([true, true])
                .min_scrolled_width(400.0)
                .show(ui, |ui| {
                    let texture: &TextureHandle = self.texture.get_or_insert_with(|| {
                        ui.ctx().load_texture(
                            &self.name,
                            load_image_from_memory(&self.img),
                            Default::default(),
                        )
                    });

                    ui.image(texture, texture.size_vec2());
                });
        });
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
