use std::path::PathBuf;

use fltk::{
    frame::Frame,
    image::{RgbImage, SharedImage},
    prelude::{ImageExt, WidgetExt},
};
use image::{io::Reader as ImageReader, DynamicImage, GenericImageView};

pub struct FrameImage {
    frame: Frame,
    pub image: Option<DynamicImage>,
}
impl FrameImage {
    pub fn new(frame: Frame, image: Option<DynamicImage>) -> Self {
        Self { frame, image }
    }

    pub fn update_frame(&mut self) {
        match &self.image {
            Some(img) => {
                let (x, y) = img.dimensions();
                let in_rgb8a = img.to_rgba8();
                let mut rgb = RgbImage::new(
                    in_rgb8a.as_raw(),
                    x as i32,
                    y as i32,
                    fltk::enums::ColorDepth::Rgba8,
                ).unwrap();
                rgb.scale(self.frame.width(), self.frame.height(), true, true);
                self.frame.set_image(Some(rgb));
            }
            None => {
                self.frame.set_image(None::<SharedImage>);
            }
        }
        self.frame.redraw();
    }

    pub fn load_image(&mut self, path: &PathBuf) {
        let img = ImageReader::open(path).unwrap().decode().unwrap();
        self.image = Some(img);
        self.update_frame();
    }
}
