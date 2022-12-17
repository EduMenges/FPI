use std::path::{PathBuf};

use fltk::{
    app, dialog,
    enums::Event,
    frame::Frame,
    group,
    prelude::*,
    window,
};


use crate::{
    flip::{flip_horizontal, flip_vertical},
    frame_image::FrameImage,
    luminance::gray_scale,
    menu::MainMenu,
    message::Message,
    quantization::quantize,
};

pub struct PhotoMenges {
    app: app::App,
    modified: bool,
    filename: Option<PathBuf>,
    r: app::Receiver<Message>,
    menu: MainMenu,
    og_image: FrameImage,
    new_image: FrameImage,
}

pub fn center() -> (i32, i32) {
    (
        (app::screen_size().0 / 2.0) as i32,
        (app::screen_size().1 / 2.0) as i32,
    )
}

impl PhotoMenges {
    pub fn new(args: Vec<String>) -> Self {
        let app = app::App::default().with_scheme(app::Scheme::Gtk);
        app::background(211, 211, 211);
        let (s, r) = app::channel::<Message>();

        let mut main_win = window::Window::default()
            .with_size(800, 600)
            .center_screen()
            .with_label("PhotoMenges");

        let menu = MainMenu::new(&s);
        let modified = false;
        menu.menu
            .find_item("&File/Save as...\t")
            .unwrap()
            .deactivate();

        let img_panel = group::Flex::default().with_pos(0, 35).size_of_parent();

        let mut og_image_f = Frame::default().with_label("Original image");
        let mut new_image_f = Frame::default().with_label("New image");
        og_image_f.set_frame(fltk::enums::FrameType::EngravedBox);
        new_image_f.set_frame(fltk::enums::FrameType::EngravedBox);

        img_panel.end();
        main_win.end();
        main_win.show();
        main_win.set_callback(move |_| {
            if app::event() == Event::Close {
                s.send(Message::Quit);
            }
        });

        let filename = if args.len() > 1 {
            Some(PathBuf::from(args[1].clone()))
        } else {
            None
        };

        Self {
            app,
            modified,
            filename,
            r,
            menu,
            og_image: FrameImage::new(og_image_f, None),
            new_image: FrameImage::new(new_image_f, None),
        }
    }

    pub fn changed(&mut self) {
        self.modified = true;
        self.menu.find_item("&File/Save as...\t").unwrap().activate();
    }

    pub fn launch(&mut self) {
        while self.app.wait() {
            use Message::*;

            if let Some(msg) = self.r.recv() {
                match msg {
                    Open => {
                        let mut open_dialog =
                            dialog::FileDialog::new(dialog::FileDialogType::BrowseFile);
                        open_dialog.set_option(dialog::FileDialogOptions::NoOptions);
                        open_dialog.set_filter("JPEG image\t*.{jpeg,jpg}");
                        open_dialog.set_title("Save image");
                        open_dialog.show();

                        let filename = open_dialog.filename();

                        if !filename.to_string_lossy().to_string().is_empty() {
                            if filename.exists() {
                                self.og_image.load_image(&filename);
                                self.filename = Some(filename);
                                self.copy_to_new();
                            } else {
                                dialog::alert(
                                    center().0 - 200,
                                    center().1 - 100,
                                    "File does not exist!",
                                );
                            };
                        };
                    }
                    SaveAs => {
                        let mut file_dialog =
                            dialog::FileDialog::new(dialog::FileDialogType::BrowseSaveFile);
                        file_dialog.set_option(dialog::FileDialogOptions::SaveAsConfirm);
                        file_dialog.set_filter("JPEG image\t*.{jpg,jpeg}");
                        file_dialog.show();

                        if file_dialog
                            .filename()
                            .to_string_lossy()
                            .to_string()
                            .is_empty()
                        {
                            dialog::alert(
                                center().0 - 200,
                                center().1 - 100,
                                "Please specify a file!",
                            );
                        } else {
                            self.modified = false;

                            self.menu
                                .menu
                                .find_item("&File/Save as...\t")
                                .unwrap()
                                .deactivate();

                            self.filename = Some(file_dialog.filename());

                            if let Err(error) = self.new_image
                                .image
                                .as_ref()
                                .unwrap()
                                .save(file_dialog.filename()) {
                                    dialog::alert(center().0, center().1, &("Error when saving the image:\n".to_owned() + &error.to_string()));
                                } else {
                                    self.modified = false;
                                }
                            
                            
                        }
                    }
                    Quit => {
                        self.app.quit();
                    }
                    Copy => {
                        self.copy_to_new();
                    }
                    FlipVertical => {
                        if let Some(img) = &mut self.new_image.image {
                            flip_vertical(img);
                            self.new_image.update_frame();
                            self.changed();
                        }
                    }
                    FlipHorizontal => {
                        if let Some(img) = &mut self.new_image.image {
                            flip_horizontal(img);
                            self.new_image.update_frame();
                            self.changed();
                        }
                    }
                    GrayScale => {
                        if let Some(img) = &mut self.new_image.image {
                            gray_scale(img);
                            self.new_image.update_frame();
                            self.changed();
                        }
                    }
                    Quantize => {
                        if let Some(img) = &mut self.new_image.image {
                            let number = self.menu.parse_input();

                            if let Some(number) = number {
                                quantize(img, number);
                                self.changed();
                                self.new_image.update_frame();
                            }                            
                        }
                    }
                    About => self.menu.help_dialog(),
                }
            }
        }
    }

    pub fn copy_to_new(&mut self) {
        if let Some(og_image) = &self.og_image.image {
            self.new_image.image = Some(og_image.clone());
            self.new_image.update_frame();
            self.modified = false;
        }
    }
}
