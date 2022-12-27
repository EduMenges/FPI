use std::path::PathBuf;

use crate::{
    histogram_graph::*,
    side_menu::{self, general},
};
use basic_ops::{
    flip::*,
    histogram::{
        calculate_histogram, cumulative_histogram, equalize_histogram, normalize_histogram,
    },
    linear_operations::{adjust_brightness, adjust_contrast, negative},
    luminance::*,
    quantization,
    zoom::{zoom_in, zoom_out},
};
use eframe::App;
use egui::{menu, CentralPanel, SidePanel, TextureHandle, TopBottomPanel, Ui};
use image::DynamicImage;

use crate::image_adaptor::load_image_from_memory;

pub struct PhotoMenges {
    side_menu: fn(&mut Self, &mut Ui),
    img_file_path: Option<PathBuf>,
    modified: bool,
    gray_scale: bool,
    og_image: Option<DynamicImage>,
    og_texture: Option<egui::TextureHandle>,
    new_image: Option<DynamicImage>,
    new_texture: Option<egui::TextureHandle>,
    pub zoom_factor: (u8, u8),
    pub quantization_value: u8,
    pub brightness_value: i16,
    pub contrast_value: u8,
    pub histograms: Vec<HistogramGraph>,
}

impl Default for PhotoMenges {
    fn default() -> Self {
        Self {
            side_menu: side_menu::no_panel,
            img_file_path: Default::default(),
            histograms: Vec::default(),
            modified: Default::default(),
            og_image: Default::default(),
            og_texture: Default::default(),
            new_image: Default::default(),
            new_texture: Default::default(),
            quantization_value: 4,
            gray_scale: false,
            brightness_value: 50,
            contrast_value: 5,
            zoom_factor: (1, 1),
        }
    }
}

impl PhotoMenges {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let mut new = PhotoMenges::default();
        new.histograms.push(HistogramGraph::default());
        new.histograms.push(HistogramGraph::default());
        new.histograms.push(HistogramGraph::default());
        new
    }

    pub fn app_menu(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        menu::bar(ui, |ui| {
            ui.menu_button("File", |ui| {
                if ui.button("Open...").clicked() {
                    if let Some(path) = rfd::FileDialog::new()
                        .add_filter("JPEG", &["jpg", "jpeg"])
                        .add_filter("PNG", &["png"])
                        .add_filter("BMP", &["bmp"])
                        .pick_file()
                    {
                        self.img_file_path = Some(path);
                        self.load_og_img();
                        ui.close_menu();
                    }
                } else if ui.button("Copy").clicked() {
                    self.copy_img();
                } else if ui.button("Save as...").clicked() {
                    todo!();
                } else if ui.button("Quit").clicked() {
                    frame.close();
                }
            });

            if self.new_image.is_some() {
                ui.menu_button("Edit", |ui| {
                    ui.menu_button("Adjustments", |ui| {
                        if ui.button("Negative").clicked() {
                            negative(self.new_image.as_mut().unwrap());
                            self.update_new_img();
                        } else if ui.button("Brightness...").clicked() {
                            self.side_menu = side_menu::brightness;
                        } else if ui.button("Contrast...").clicked() {
                            self.side_menu = side_menu::contrast;
                        } else if ui.button("Equalize").clicked() {
                            self.equalize_img();
                        }
                    });
                    ui.menu_button("Transform", |ui| {
                        ui.menu_button("Flip", |ui| {
                            if ui.button("Horizontal").clicked() {
                                flip_horizontal(self.new_image.as_mut().unwrap());
                                self.update_new_img();
                            } else if ui.button("Vertical").clicked() {
                                flip_vertical(self.new_image.as_mut().unwrap());
                                self.update_new_img();
                            }
                        })
                    });
                    if ui.button("Luminance").clicked() {
                        self.do_gray_scale();
                        self.update_new_img();
                    } else if ui.button("Quantize...").clicked() {
                        self.side_menu = side_menu::quantization;
                    }
                });
                ui.menu_button("View", |ui| {
                    if ui.button("Histogram").clicked() {
                        self.do_histogram();
                    } else if ui.button("Cumulative histogram").clicked() {
                        self.do_cumulative_histogram();
                    } else if ui.button("Normalized cumulative histogram").clicked() {
                        self.do_normalized_cumulative_histogram();
                    }
                });
            };
        });

        for window in &mut self.histograms {
            window.plot_histogram(ui.ctx());
        }
    }

    fn load_og_img(&mut self) {
        let in_path = self.img_file_path.as_ref().unwrap().as_path();
        let loaded_image = image::io::Reader::open(in_path).unwrap().decode().unwrap();
        self.og_image = Some(loaded_image);
        self.og_texture = None;
        self.copy_img();
    }

    fn update_new_img(&mut self) {
        self.modified = true;
        self.new_texture = None;
    }

    fn copy_img(&mut self) {
        self.modified = false;
        self.gray_scale = false;
        self.new_image = self.og_image.clone();
        self.update_new_img();
    }

    fn make_center_img(
        img: &Option<DynamicImage>,
        texture: &mut Option<TextureHandle>,
        ui: &mut egui::Ui,
    ) {
        ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
            if let Some(img) = img {
                let texture: &egui::TextureHandle = texture.get_or_insert_with(|| {
                    ui.ctx().load_texture(
                        "og_texture",
                        load_image_from_memory(img),
                        Default::default(),
                    )
                });

                ui.image(texture, texture.size_vec2());
            }
        });
    }

    pub fn quantize(&mut self) {
        self.do_gray_scale();
        quantization::quantize(self.new_image.as_mut().unwrap(), self.quantization_value);
        self.update_new_img();
    }

    fn do_gray_scale(&mut self) {
        if !self.gray_scale {
            gray_scale(self.new_image.as_mut().unwrap());
            self.gray_scale = true;
            self.update_new_img();
        }
    }

    fn do_histogram(&mut self) {
        // self.do_gray_scale();
        self.histograms[0] = HistogramGraph::new(
            "Histogram of the new image".to_owned(),
            calculate_histogram(self.new_image.as_ref().unwrap()),
        );
    }

    fn do_cumulative_histogram(&mut self) {
        // self.do_gray_scale();
        self.histograms[1] = HistogramGraph::new(
            "Cumulative histogram of the new image".to_owned(),
            cumulative_histogram(&calculate_histogram(self.new_image.as_ref().unwrap())),
        );
    }

    fn do_normalized_cumulative_histogram(&mut self) {
        let in_u8 = normalize_histogram(cumulative_histogram(&calculate_histogram(
            self.new_image.as_ref().unwrap(),
        )));
        let mut in_u32 = [0_u32; 256];

        for i in 0..in_u8.len() {
            in_u32[i] = in_u8[i] as u32;
        }

        self.histograms[2] =
            HistogramGraph::new("Normalized cumulative of the new image".to_owned(), in_u32);
    }

    pub fn do_brightness(&mut self) {
        adjust_brightness(self.new_image.as_mut().unwrap(), self.brightness_value);
        self.update_new_img();
    }

    pub fn do_contrast(&mut self) {
        adjust_contrast(self.new_image.as_mut().unwrap(), self.contrast_value);
        self.update_new_img();
    }

    fn equalize_img(&mut self) {
        equalize_histogram(self.new_image.as_mut().unwrap(), !self.gray_scale);
        self.update_new_img()
    }

    pub fn do_zoom_in(&mut self) {
        if self.new_image.is_some() {
            self.new_image = Some(zoom_in(self.new_image.take().unwrap()));
            self.update_new_img()
        }
    }

    pub fn do_zoom_out(&mut self) {
        if self.new_image.is_some() {
            self.new_image = Some(zoom_out(self.new_image.take().unwrap(), self.zoom_factor));
            self.update_new_img();
        };
    }
}

impl App for PhotoMenges {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        TopBottomPanel::top("menu").show(ctx, |ui| {
            self.app_menu(ui, frame);
        });

        SidePanel::left("old_image").show(ctx, |ui| {
            ui.heading("Old image");

            PhotoMenges::make_center_img(&self.og_image, &mut self.og_texture, ui);
        });

        SidePanel::right("options").show(ctx, |ui| {
            ui.heading("Options");
            (self.side_menu)(self, ui);
            ui.separator();
            general(self, ui);
        });

        CentralPanel::default().show(ctx, |ui| {
            ui.heading("New image");

            PhotoMenges::make_center_img(&self.new_image, &mut self.new_texture, ui);
        });
    }
}
