use std::{ops::Deref, u8};

use fltk::{
    app, dialog,
    enums::{FrameType, Shortcut},
    group::Row,
    input::IntInput,
    prelude::*, menu,
};

use crate::{app::center, message::Message};

pub struct MainMenu {
    pub menu: menu::SysMenuBar,
    pub quantize_input: IntInput,
}

impl MainMenu {
    pub fn new(s: &app::Sender<Message>) -> Self {
        let mut row = Row::default().with_size(800, 35);
        let mut menu = menu::SysMenuBar::default();

        menu.set_frame(FrameType::FlatBox);

        menu.add_emit(
            "&File/Open...\t",
            Shortcut::Ctrl | 'o',
            menu::MenuFlag::Normal,
            *s,
            Message::Open,
        );

        menu.add_emit(
            "&File/Save as...\t",
            Shortcut::Ctrl | 'w',
            menu::MenuFlag::Normal,
            *s,
            Message::SaveAs,
        );

        menu.add_emit(
            "&File/Copy\t",
            Shortcut::Ctrl | 'c',
            menu::MenuFlag::Normal,
            *s,
            Message::Copy,
        );

        menu.add_emit(
            "&Edit/Flip/Vertical\t",
            Shortcut::None,
            menu::MenuFlag::Normal,
            *s,
            Message::FlipVertical,
        );

        menu.add_emit(
            "&Edit/Flip/Horizontal\t",
            Shortcut::None,
            menu::MenuFlag::Normal,
            *s,
            Message::FlipHorizontal,
        );

        menu.add_emit(
            "&Edit/Luminance",
            Shortcut::None,
            menu::MenuFlag::Normal,
            *s,
            Message::GrayScale,
        );

        menu.add_emit(
            "&Edit/Quantize",
            Shortcut::None,
            menu::MenuFlag::Normal,
            *s,
            Message::Quantize,
        );

        menu.add_emit(
            "&Help/About\t",
            Shortcut::None,
            menu::MenuFlag::Normal,
            *s,
            Message::About,
        );

        let mut quantize_input = IntInput::default()
            .with_label("Quantization number");
        quantize_input.set_maximum_size(3);
        quantize_input.set_value("5");

        row.add(&menu);
        row.add(&quantize_input);
        row.end();

        Self {
            menu,
            quantize_input,
        }
    }

    pub fn help_dialog(&self) {
        dialog::message(center().0 - 300, center().1 - 100, "This is an application developed for the Image Processing Fundamentals discipline at UFRGS.")
    }

    pub fn parse_input(&self) -> Option<u8> {
        let parsed = self.quantize_input.value().parse::<u8>();

        match parsed {
            Ok(parsed) => {
                if parsed > 1 {
                    Some(parsed)
                } else {
                    dialog::alert(center().0 - 300, center().1 - 100, "The quantization number must be greater than 1.");
                    None
                }
            },
            Err(res) => {
                let message = String::from("Error when parsing the quantization number:\n") + &res.to_string();

                dialog::alert(center().0 - 300, center().1 - 100, &message);
                None
            }
        }
    }
}

impl Deref for MainMenu {
    type Target = menu::SysMenuBar;

    fn deref(&self) -> &Self::Target {
        &self.menu
    }
}
