use fltk::{
    window::{Window, WindowType},
    button::Button,
    file_chooser::{FileChooser, FileChooserAction},
};

struct MyDataModel {
    window: Window,
    button: Button,
}

impl MyDataModel {
    fn new() -> Self {
        let mut window = Window::new(100, 100, 200, 100, "File chooser example");
        window.set_type(WindowType::Popup);

        let mut button = Button::new(75, 50, 50, 25, "Open File");
        button.set_callback(Self::on_click);

        Self {
            window,
            button,
        }
    }

    fn on_click(&mut self) {
        let mut chooser = FileChooser::new(FileChooserAction::Open);
        chooser.show();

        // Do something with the selected file here...
    }

    fn show(&mut self) {
        self.window.show();
    }
}

fn main() {
    let img = ImageReader::open("Flor 2.jpg").unwrap().decode().unwrap();    
    let new_image = quntization::quantize(&img, 2);
    new_image.save("novo.png").unwrap();
    
    let mut app = MyDataModel::new();
    app.show();
}
