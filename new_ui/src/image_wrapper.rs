use egui::{Align, Context, TextureHandle, Ui};
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
        ui.with_layout(egui::Layout::top_down(Align::LEFT), |ui| {
            ui.label(&self.title);
            let texture: &TextureHandle = self.texture.get_or_insert_with(|| {
                ui.ctx().load_texture(
                    &self.name,
                    load_image_from_memory(&self.img),
                    Default::default(),
                )
            });
            ui.image(texture, texture.size_vec2())
        });
    }
}
