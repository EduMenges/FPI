use std::path::PathBuf;

use crate::{
    file_loading::{load_img_from_path, open_dialog},
    histogram_graph::*,
    image_decorator::ImageDecorator,
    image_wrapper::ImageWrapper,
    preview::*,
    side_menu::{self, general},
};

use basic_ops::filters::{Kernel, GAUSSIAN_FILTER};
use eframe::App;
use egui::{menu, CentralPanel, Context, SidePanel, TopBottomPanel, Ui};

pub struct PhotoMenges {
    side_menu: fn(&mut Self, &mut Ui),
    img_file_path: Option<PathBuf>,
    pub kernel: Kernel,
    og_image: Option<ImageWrapper>,
    preview: Option<Preview>,
    pub new_image: Option<ImageDecorator>,
    pub zoom_factor: (u8, u8),
    pub quantization_value: u8,
    pub brightness_value: i16,
    pub contrast_value: f64,
    pub histograms: Vec<HistogramGraph>,
}

impl Default for PhotoMenges {
    fn default() -> Self {
        Self {
            side_menu: side_menu::no_panel,
            img_file_path: Default::default(),
            histograms: Vec::default(),
            og_image: Default::default(),
            new_image: Default::default(),
            quantization_value: 4,
            brightness_value: 50,
            contrast_value: 1.5,
            zoom_factor: (1, 1),
            kernel: GAUSSIAN_FILTER.clone(),
            preview: None,
        }
    }
}

impl PhotoMenges {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        PhotoMenges::default()
    }

    pub fn app_menu(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        menu::bar(ui, |ui| {
            ui.menu_button("File", |ui| {
                if ui.button("Open...").clicked() {
                    if let Some(path) = open_dialog() {
                        self.img_file_path = Some(path);
                        self.load_og_img(ui.ctx());
                        ui.close_menu();
                    }
                }

                if let Some(img) = &mut self.new_image {
                    if ui.button("Copy").clicked() {
                        img.copy(&self.og_image.as_ref().unwrap().img);
                        ui.close_menu();

                    } else if ui.button("Save as...").clicked() {
                        img.save_image();
                        ui.close_menu();
                    }
                }

                if ui.button("Quit").clicked() {
                    frame.close();
                }
            });

            if let Some(img) = &mut self.new_image {
                ui.menu_button("Edit", |ui| {
                    ui.menu_button("Adjustments", |ui| {
                        if ui.button("Negative").clicked() {
                            img.negative();
                        } else if ui.button("Brightness...").clicked() {
                            self.side_menu = side_menu::brightness;
                        } else if ui.button("Contrast...").clicked() {
                            self.side_menu = side_menu::contrast;
                        } else if ui.button("Equalize...").clicked() {
                            let mut equalized = ImageDecorator::new(img.wrapper.img.clone(), ui.ctx(), "equalized".to_owned(), "Equalized".to_owned());
                            equalized.equalize();

                            let og_image = ImageDecorator::new(img.wrapper.img.clone(), ui.ctx(), "original".to_owned(), "Original".to_owned());

                            self.preview = Some(Preview::new(og_image, equalized));
                        } else if ui.button("Histogram matching...").clicked() {
                            let source_path = open_dialog();

                            if let Some(path) = source_path {
                                let source_img = load_img_from_path(&path).grayscale();

                                let mut matched = img.clone();
                                matched.gray_scale();
                                matched.match_histogram(&source_img);

                                let source_decorator = ImageDecorator::new(source_img, ui.ctx(), "source-image".to_owned(), "Source image".to_owned());

                                self.preview = Some(Preview::new(source_decorator, matched));
                            }
                        }
                    });

                    ui.menu_button("Transform", |ui| {
                        ui.menu_button("Flip", |ui| {
                            if ui.button("Horizontal").clicked() {
                                img.flip_horizontal();
                            } else if ui.button("Vertical").clicked() {
                                img.flip_vertical();
                            }
                        });

                        ui.menu_button("Rotate", |ui| {
                            if ui.button("Clockwise").clicked() {
                                img.rotate_clockwise();
                            } else if ui.button("Counter-clockwise").clicked() {
                                img.rotate_counter();
                            }
                        });
                    });

                    if ui.button("Luminance").clicked() {
                        img.gray_scale();
                    } else if ui.button("Quantize...").clicked() {
                        self.side_menu = side_menu::quantization;
                    }
                });

                ui.menu_button("Filter", |ui| {
                    if ui.button("Convolve...").clicked() {
                        self.side_menu = side_menu::convolve;
                    };
                });

                if let Some(preview) = &mut self.preview {
                    if let Some(wrapper) = preview.ui(ui) {
                        self.preview = None;
                        img.change_wrapper(wrapper);
                    }
                }
            };
        });

        for window in &mut self.histograms {
            window.plot_histogram(ui.ctx());
        }
    }

    fn load_og_img(&mut self, ctx: &Context) {
        let loaded_image = load_img_from_path(self.img_file_path.as_ref().unwrap());

        self.new_image = Some(ImageDecorator::new(
            loaded_image.clone(),
            ctx,
            "new-image".to_owned(),
            "New image".to_owned(),
        ));

        self.og_image = Some(ImageWrapper::new(
            loaded_image,
            "og-image".to_owned(),
            "Original image".to_owned(),
            ctx,
        ));
    }
}

impl App for PhotoMenges {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        TopBottomPanel::top("menu").show(ctx, |ui| {
            self.app_menu(ui, frame);
        });

        SidePanel::left("old_image").show(ctx, |ui| {
            if let Some(img) = &mut self.og_image {
                img.ui(ui);
            }
        });

        SidePanel::right("options").show(ctx, |ui| {
            ui.heading("Options");
            (self.side_menu)(self, ui);
            ui.separator();
            general(self, ui);
        });

        CentralPanel::default().show(ctx, |ui| {
            if let Some(img) = &mut self.new_image {
                img.ui(ui);
            }
        });
    }
}
