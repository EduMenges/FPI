use image::DynamicImage;

fn common_dialog() -> rfd::FileDialog {
    rfd::FileDialog::new()
        .add_filter("JPEG", &["jpg", "jpeg"])
        .add_filter("PNG", &["png"])
        .add_filter("BMP", &["bmp"])
}

pub fn open_dialog() -> Option<std::path::PathBuf> {
    common_dialog().set_title("Open image").pick_file()

}

pub fn save_dialog() -> Option<std::path::PathBuf> {
    common_dialog().set_title("Save image").save_file()
}

pub fn load_img_from_path(path: &std::path::PathBuf) -> DynamicImage {
    image::io::Reader::open(path).unwrap().decode().unwrap()
}
