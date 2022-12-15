use fltk::{app, frame::Frame, image::JpegImage, prelude::*, window};

fn main() {
    let app = app::App::default().with_scheme(app::Scheme::Gtk);

    let mut main_win = window::Window::default()
        .with_size(800, 600)
        .center_screen()
        .with_label("PhotoMenges");

    let mut img_frame = Frame::default().size_of_parent();
    let img = JpegImage::load("Flor 2.jpg").unwrap();
    
    img_frame.set_image(Some(img));
    
    img_frame.resize_callback(|s, _, _, _, _| {
        s.image().unwrap().scale(s.width(), s.height(), true, true);
        s.redraw();
    });

    main_win.make_resizable(true);
    main_win.end();
    main_win.show();
    app.run().unwrap();
}
