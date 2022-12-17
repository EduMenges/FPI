use app::PhotoMenges;

pub mod flip;
pub mod luminance;
pub mod quantization;
pub mod message;
pub mod app;
pub mod menu;
pub mod frame_image;

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let mut app = PhotoMenges::new(args);
    app.launch();
}
