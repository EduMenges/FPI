use fltk::{
    app,
    button::{self, Button},
    frame::{Frame, self},
    group, menu,
    prelude::*,
    window, dialog::{input, self}, input, enums,
};
use image::io::Reader;

mod flip;
mod luminance;
mod quntization;

fn main() {
    // let img = Reader::open("Flor 2.jpg").unwrap().decode().unwrap();
    // let new_image = quntization::quantize(&img, 2);
    // new_image.save("novo.png").unwrap();

    let app = app::App::default().load_system_fonts();
    // To load a font by path, check the App::load_font() method
    let fonts = app::fonts();
    // println!("{:?}", fonts);
    let mut wind = window::Window::default().with_size(400, 300);
    let mut frame = frame::Frame::default().size_of(&wind);
    frame.set_label_size(30);
    wind.set_color(enums::Color::White);
    wind.end();
    wind.show();
    println!("The system has {} fonts!\nStarting slideshow!", fonts.len());
    let mut i = 0;
    while app.wait() {
        if i == fonts.len() {
            i = 0;
        }
        frame.set_label(&format!("[{}]", fonts[i]));
        frame.set_label_font(enums::Font::by_index(i));
        app::sleep(0.5);
        i += 1;
    }
}
