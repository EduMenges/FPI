use basic_ops::{
    flip::{flip_horizontal, flip_vertical},
    histogram::equalize_histogram,
    linear_operations::{adjust_brightness, adjust_contrast, negative},
    luminance::gray_scale,
    quantization,
    rotate::rotate,
    zoom::{zoom_in, zoom_out},
};
use egui::{Ui, Context};
use image::DynamicImage;

use crate::{histogram_graph::HistogramGraph, image_wrapper::ImageWrapper};

pub struct ImageDecorator {
    wrapper: ImageWrapper,
    gray_scale: bool,
    modified: bool,
    histograms: [HistogramGraph; 3],
}

impl ImageDecorator {
    pub fn new(img: DynamicImage, ctx: &Context, name: String, title: String) -> Self {
        let wrapper = ImageWrapper::new(img, name, title, ctx);
        let histograms = HistogramGraph::all_histograms(&wrapper.img);

        Self {
            wrapper,
            gray_scale: false,
            modified: false,
            histograms,
        }
    }

    pub fn flip_horizontal(&mut self) {
        flip_horizontal(&mut self.wrapper.img);
        self.refresh();
    }

    pub fn flip_vertical(&mut self) {
        flip_vertical(&mut self.wrapper.img);
        self.refresh()
    }

    pub fn ui(&mut self, ui: &mut Ui) {
        self.wrapper.ui(ui);
    }

    pub fn brightness(&mut self, n: i16) {
        adjust_brightness(&mut self.wrapper.img, n);
        self.refresh();
    }

    pub fn contrast(&mut self, n: u8) {
        adjust_contrast(&mut self.wrapper.img, n);
        self.refresh();
    }

    pub fn quantize(&mut self, n: u8) {
        self.gray_scale();
        quantization::quantize(&mut self.wrapper.img, n);
        self.refresh();
    }

    pub fn gray_scale(&mut self) {
        if !self.gray_scale {
            gray_scale(&mut self.wrapper.img);
            self.gray_scale = true;
            self.refresh();
        }
    }

    pub fn copy(&mut self, img: &DynamicImage) {
        self.wrapper.img = img.clone();
        self.refresh();
    }

    pub fn zoom_in(&mut self) {
        self.wrapper.img = zoom_in(self.wrapper.img.clone());
        self.refresh();
    }

    pub fn zoom_out(&mut self, factors: (u8, u8)) {
        self.wrapper.img = zoom_out(self.wrapper.img.clone(), factors);
        self.refresh();
    }

    pub fn equalize(&mut self) {
        equalize_histogram(&mut self.wrapper.img, !self.gray_scale);
        self.refresh();
    }

    pub fn update_histograms(&mut self) {
        self.histograms = HistogramGraph::all_histograms(&self.wrapper.img);
    }

    pub fn refresh(&mut self) {
        self.update_histograms();
        self.wrapper.reset();
        self.modified = true;
    }

    pub fn negative(&mut self) {
        negative(&mut self.wrapper.img);
        self.refresh();
    }

    pub fn rotate_clockwise(&mut self) {
        self.wrapper.img = rotate(self.wrapper.img.clone(), true);
        self.refresh();
    }

    pub fn rotate_counter(&mut self) {
        self.wrapper.img = rotate(self.wrapper.img.clone(), false);
        self.refresh();
    }
}
