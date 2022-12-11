use fltk::{
    app,
    enums::{FrameType, Shortcut},
    menu,
    prelude::{MenuExt, WidgetExt},
};

use crate::message::Message;

pub struct MainMenu {
   pub menu: menu::SysMenuBar,
}

impl MainMenu {
    pub fn new(s: &app::Sender<Message>) -> Self {
        let mut menu = menu::SysMenuBar::default().with_size(800, 35);

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
            "&Edit/Mirror/Vertical\t",
            Shortcut::None,
            menu::MenuFlag::Normal,
            *s,
            Message::MirrorVertical,
        );

        menu.add_emit(
            "&Edit/Mirror/Horizontal\t",
            Shortcut::None,
            menu::MenuFlag::Normal,
            *s,
            Message::MirorrHorizontal,
        );

        menu.add_emit(
            "&Edit/Gray scale",
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

        Self { menu }
    }
}
