#![windows_subsystem = "windows"]

mod app;
mod custom_styles;

use app::App;
// use iced::Font;

fn main() {
    iced::application(App::default(), App::update, App::view)
        .subscription(App::subscriptions)
        .theme(App::theme)
        .decorations(false)
        .window_size([230.0, 50.0])
        // .font(include_bytes!("C:/Windows/Fonts/simsun.ttc"))
        // .default_font(Font::with_name("新宋体"))
        .run()
        .unwrap();
}
