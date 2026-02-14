use basic_ops::filters::{self};
use egui::{Slider, Ui};

use crate::app::PhotoMenges;

pub fn no_panel(_: &mut PhotoMenges, ui: &mut Ui) {
    ui.label("No operation selected.");
}

pub fn quantization(app: &mut PhotoMenges, ui: &mut Ui) {
    ui.close_menu();
    ui.label("This is the quantization menu!");
    ui.label("Pick a value between 2 and 254 and quantize the image.");
    ui.add(Slider::new(&mut app.quantization_value, 2..=254));

    if ui.button("Apply").clicked() {
        app.new_image
            .as_mut()
            .unwrap()
            .quantize(app.quantization_value);
    }
}

pub fn brightness(app: &mut PhotoMenges, ui: &mut Ui) {
    ui.close_menu();
    ui.label("This is the brightness adjustment menu!");
    ui.label("Pick a value in [-255, 255].");
    ui.add(Slider::new(&mut app.brightness_value, -255..=255));

    if ui.button("Apply").clicked() {
        app.new_image
            .as_mut()
            .unwrap()
            .brightness(app.brightness_value);
    }
}

pub fn contrast(app: &mut PhotoMenges, ui: &mut Ui) {
    ui.close_menu();
    ui.label("This is the contrast adjustment menu!");
    ui.label("Pick a value in (0, 255].");
    ui.add(Slider::new(&mut app.contrast_value, f64::MIN_POSITIVE..=255.0));

    if ui.button("Apply").clicked() {
        app.new_image.as_mut().unwrap().contrast(app.contrast_value);
    }
}

pub fn general(app: &mut PhotoMenges, ui: &mut Ui) {
    if let Some(img) = &mut app.new_image {
        ui.label("Zoom in");
        if ui.button("Apply").clicked() {
            img.zoom_in();
        }

        ui.separator();
        ui.label("Zoom out");

        ui.add(Slider::new(&mut app.zoom_factor.0, 1..=10).text("X factor"));
        ui.add(Slider::new(&mut app.zoom_factor.1, 1..=10).text("Y factor"));

        if ui.button("Apply").clicked() {
            img.zoom_out(app.zoom_factor);
        };

        ui.separator();
    }
}

pub fn convolve(app: &mut PhotoMenges, ui: &mut Ui) {
    ui.close_menu();
    ui.label("Filter options");

    ui.menu_button("Preset filters ⏷", |ui| {
        filters::ALL_FILTERS.iter().for_each(|filter| {
            if ui.button(filter.name()).clicked() {
                app.kernel = (**filter).clone();
                ui.close_menu();
            };
        });
    });

    ui.separator();

    egui::Grid::new("filter-grid").show(ui, |ui| {
        app.kernel.kernel.iter_mut().for_each(|row| {
            row.iter_mut().for_each(|item| {
                ui.add(egui::DragValue::new(item).speed(0.05));
            });
            ui.end_row();
        });
    });

    if ui.button("Apply").clicked() {
        app.kernel.update_rotated();
        app.new_image.as_mut().unwrap().convolve(&app.kernel);
    }
}
