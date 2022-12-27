use egui::{Ui, Slider};

use crate::app::PhotoMenges;

pub fn no_panel(_: &mut PhotoMenges, ui: &mut Ui) {
    ui.label("No operation selected.");
}

pub fn quantization(app: &mut PhotoMenges, ui: &mut Ui) {
    ui.label("This is the quantization menu!");
    ui.label("Pick a value between 2 and 254 and quantize the image.");
    ui.add(Slider::new(&mut app.quantization_value, 2..=254));

    if ui.button("Apply").clicked() {
        app.quantize();
    }
}

pub fn brightness(app: &mut PhotoMenges, ui: &mut Ui) {
    ui.label("This is the brightness adjustment menu!");
    ui.label("Pick a value in [-255, 255].");
    ui.add(Slider::new(&mut app.brightness_value, -255..=255));

    if ui.button("Apply").clicked() {
        app.do_brightness();
    }
}

pub fn contrast(app: &mut PhotoMenges, ui: &mut Ui) {
    ui.label("This is the contrast adjustment menu!");
    ui.label("Pick a value in (0, 255].");
    ui.add(Slider::new(&mut app.contrast_value, 1..=255));

    if ui.button("Apply").clicked() {
        app.do_contrast();
    }
}

pub fn general(app: &mut PhotoMenges, ui: &mut Ui) {
    ui.label("Zoom in");
    if ui.button("Apply").clicked() {
        app.do_zoom_in();
    }
    
    ui.separator();
    ui.label("Zoom out");

    ui.add(Slider::new(&mut app.zoom_factor.0, 1..=10).text("X factor"));
    ui.add(Slider::new(&mut app.zoom_factor.1, 1..=10).text("Y factor"));

    if ui.button("Apply").clicked() {
        app.do_zoom_out();
    };

    ui.separator();
    
}
