use std::{
    error,
    path::{self, Path, PathBuf},
};

use fltk::{
    app, dialog,
    enums::{Color, Event},
    frame::Frame,
    image::{self, JpegImage},
    prelude::*,
    text, window, group::Flex,
};
use menu::MainMenu;
use message::Message;

mod menu;
mod message;

pub struct PhotoMenges {
    app: app::App,
    modified: bool,
    filename: Option<PathBuf>,
    r: app::Receiver<Message>,
    main_win: window::Window,
    menu: MainMenu,
    img_frame: Frame,
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
        let og_image: Option<image::JpegImage> = None;

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

        let mut img_frame = Frame::default().size_of_parent();
        img_frame.set_pos(0, 35);
        img_frame.set_frame(fltk::enums::FrameType::EngravedBox);
        img_frame.set_image(og_image);

        main_win.make_resizable(true);
        img_frame.resize_callback(|i_f, _, _, _, _| {
            if let img = i_f.image() {
                println!("{:?}", img.);
            }
        });
        // only resize editor, not the menu bar
        main_win.end();
        main_win.show();
        main_win.set_callback(move |_| {
            if app::event() == Event::Close {
                s.send(Message::Quit);
            }
        });

        let filename = if args.len() > 1 {
            let file = path::Path::new(&args[1]);
            assert!(
                file.exists() && file.is_file(),
                "An error occurred while opening the file!"
            );
            match JpegImage::load(&args[1]) {
                Ok(sh) => {
                    img_frame.set_image_scaled(Some(sh));
                    Some(PathBuf::from(args[1].clone()))
                }
                Err(e) => {
                    dialog::alert(
                        center().0 - 200,
                        center().1 - 100,
                        &format!("An issue occured while loading the file: {}", e),
                    );
                    None
                }
            }
        } else {
            None
        };

        Self {
            app,
            modified,
            filename,
            r,
            main_win,
            menu,
            img_frame,
        }
    }

    pub fn save_file_as(&mut self) -> Result<bool, Box<dyn error::Error>> {
        let mut file_dialog = dialog::FileDialog::new(dialog::FileDialogType::BrowseSaveFile);
        file_dialog.set_option(dialog::FileDialogOptions::SaveAsConfirm);
        file_dialog.show();

        if file_dialog
            .filename()
            .to_string_lossy()
            .to_string()
            .is_empty()
        {
            dialog::alert(center().0 - 200, center().1 - 100, "Please specify a file!");
            return Ok(false);
        }

        self.modified = false;

        self.menu
            .menu
            .find_item("&File/Save as...\t")
            .unwrap()
            .deactivate();

        self.menu
            .menu
            .find_item("&File/Quit\t")
            .unwrap()
            .set_label_color(Color::Black);

        self.filename = Some(file_dialog.filename());

        self.main_win.set_label(&format!(
            "{:?} - PhotoMenges",
            self.filename.as_ref().unwrap()
        ));
        Ok(true)
    }

    pub fn launch(&mut self) {
        while self.app.wait() {
            use Message::*;

            if let Some(msg) = self.r.recv() {
                match msg {
                    Changed => todo!(),
                    Open => {
                        let mut open_dialog = dialog::FileDialog::new(dialog::FileDialogType::BrowseFile);
                        open_dialog.set_option(dialog::FileDialogOptions::NoOptions);
                        open_dialog.set_filter("*.{jpeg, jpg}");
                        open_dialog.show();

                        let filename = open_dialog.filename();

                        if !filename.to_string_lossy().to_string().is_empty() {
                            if filename.exists() {
                                match JpegImage::load(&filename) {
                                    Ok(mut sh) => {
                                        sh.scale(self.img_frame.width(), self.img_frame.height(), true, true);
                                        self.img_frame.set_image(Some(sh));
                                        self.img_frame.redraw();
                                        self.filename = Some(filename);
                                    },
                                    Err(e) => {
                                        self.img_frame.set_image::<JpegImage>(None);
                                        dialog::alert(center().0 - 200, center().1 - 100, &format!("An issue occured while loading the file: {}", e));},
                                }
                            } else {
                                dialog::alert(center().0 - 200, center().1 - 100, "File does not exist!");
                            };
                        };
                    },
                    SaveAs => {self.save_file_as().unwrap();},
                    Quit => {
                        if self.modified {
                            match dialog::choice2(center().0 - 200, center().1 - 100,
                                "Would you like to save your work?", "Yes", "No", "") {
                                Some(0) => {
                                    if self.save_file_as().unwrap() {
                                        self.app.quit();
                                    }
                                },
                                Some(1) => { self.app.quit() },
                                Some(_) | None  => (),
                            }
                        } else {
                            self.app.quit();
                        }
                    },
                    Copy => todo!(),
                    MirrorVertical => todo!(),
                    MirorrHorizontal => todo!(),
                    GrayScale => todo!(),
                    Quantize => todo!(),
                    About => dialog::message(center().0 - 300, center().1 - 100, "This is an application developed for the Image Processing Fundamentals discipline at UFRGS."),
                }
            }
        }
    }
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let mut app = PhotoMenges::new(args);
    app.launch();
}
