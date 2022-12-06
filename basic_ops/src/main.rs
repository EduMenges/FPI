use druid::{
    platform_menus::win::file::new,
    widget::{Button, Flex, Label},
    AppLauncher, FileDialogOptions, FileSpec, PlatformError, Widget, WidgetExt, WindowDesc, commands::{SHOW_SAVE_PANEL, SHOW_OPEN_PANEL},
};

fn main() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(build_ui())
        .title("BasicOps")
        .window_size_policy(druid::WindowSizePolicy::Content);

    let original_window = WindowDesc::new(original_window())
        .title("Original image")
        .window_size_policy(druid::WindowSizePolicy::Content);

    let new_window = WindowDesc::new(new_window())
        .title("New image")
        .window_size_policy(druid::WindowSizePolicy::Content);

    AppLauncher::with_window(main_window).launch(())?;
    Ok(())
}

fn build_ui() -> impl Widget<()> {
    Flex::column()
        .with_child(Button::new("Load image").on_click(
            move |ctx, _, _| {
                ctx.submit_command(SHOW_OPEN_PANEL.with(image_file_dialog()))
            }
        ))
        .with_child(Button::new("Copy"))
        .with_child(Button::new("Flip image"))
        .with_child(Button::new("Quantize"))
        .with_child(Button::new("Luminance"))
}

fn original_window() -> impl Widget<()> {
    Label::new("Original image")
}

fn new_window() -> impl Widget<()> {
    Label::new("New image")
}

fn image_file_dialog() -> FileDialogOptions {
    FileDialogOptions::new().allowed_types(vec![FileSpec::JPG])
}
