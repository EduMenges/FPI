use basic_ops::{
    convolve::convolve,
    filters::Kernel,
    flip::{flip_horizontal, flip_vertical},
    histogram::{equalize_histogram, matching},
    linear_operations::{adjust_brightness, adjust_contrast, negative},
    luminance::gray_scale,
    quantization,
    rotate::rotate,
    zoom::{zoom_in, zoom_out},
};
use egui::{Context, Ui};
use image::DynamicImage;

use crate::{
    file_loading::save_dialog, histogram_graph::HistogramGraph, image_wrapper::ImageWrapper,
};

#[derive(Clone)]
pub struct ImageDecorator {
    histograms: [HistogramGraph; 3],
    pub wrapper: ImageWrapper,
}

impl ImageDecorator {
    pub fn brightness(&mut self, n: i16) {
        adjust_brightness(&mut self.wrapper.img, n);
        self.refresh();
    }

    pub fn contrast(&mut self, n: f64) {
        adjust_contrast(&mut self.wrapper.img, n);
        self.refresh();
    }

    pub fn convolve(&mut self, kernel: &Kernel) {
        self.wrapper.img = convolve(self.wrapper.img.clone(), kernel);
        self.refresh()
    }

    pub fn copy(&mut self, img: &DynamicImage) {
        self.wrapper.img = img.clone();
        self.refresh();
    }

    pub fn equalize(&mut self) {
        equalize_histogram(&mut self.wrapper.img);
        self.refresh();
    }

    pub fn flip_horizontal(&mut self) {
        flip_horizontal(&mut self.wrapper.img);
        self.refresh();
    }

    pub fn flip_vertical(&mut self) {
        flip_vertical(&mut self.wrapper.img);
        self.refresh()
    }

    pub fn gray_scale(&mut self) {
        gray_scale(&mut self.wrapper.img);
        self.refresh();
    }

    pub fn negative(&mut self) {
        negative(&mut self.wrapper.img);
        self.refresh();
    }

    pub fn match_histogram(&mut self, source: &DynamicImage) {
        matching(&mut self.wrapper.img, source);
        self.refresh();
    }

    pub fn new(img: DynamicImage, ctx: &Context, name: String, title: String) -> Self {
        let wrapper = ImageWrapper::new(img, name, title, ctx);
        let histograms = HistogramGraph::all_histograms(&wrapper);

        Self {
            wrapper,
            histograms,
        }
    }

    pub fn quantize(&mut self, n: u8) {
        self.gray_scale();
        quantization::quantize(&mut self.wrapper.img, n);
        self.refresh();
    }

    pub fn refresh(&mut self) {
        self.update_histograms();
        self.wrapper.reset();
    }

    pub fn rotate_clockwise(&mut self) {
        self.wrapper.img = rotate(self.wrapper.img.clone(), true);
        self.refresh();
    }

    pub fn rotate_counter(&mut self) {
        self.wrapper.img = rotate(self.wrapper.img.clone(), false);
        self.refresh();
    }

    pub fn ui(&mut self, ui: &mut Ui) {
        ui.vertical(|ui| {
            ui.menu_button("View ⏷", |ui| {
                for hist in &mut self.histograms {
                    if ui.button(hist.title()).clicked() {
                        hist.open = true;
                    }
                }
            });
            self.wrapper.ui(ui);
        });

        for hist in &mut self.histograms {
            if hist.open {
                hist.plot_histogram(ui.ctx());
            }
        }
    }

    pub fn update_histograms(&mut self) {
        self.histograms = HistogramGraph::all_histograms(&self.wrapper);
    }

    pub fn zoom_in(&mut self) {
        self.wrapper.img = zoom_in(self.wrapper.img.clone());
        self.refresh();
    }

    pub fn zoom_out(&mut self, factors: (u8, u8)) {
        self.wrapper.img = zoom_out(self.wrapper.img.clone(), factors);
        self.refresh();
    }

    pub fn change_wrapper(&mut self, wrapper: ImageWrapper) {
        self.wrapper = wrapper;
        self.refresh();
    }

    pub fn save_image(&self) {
        let path = save_dialog();

        if let Some(path) = path {
            self.wrapper.img.save(path).unwrap();
        }
    }
}
