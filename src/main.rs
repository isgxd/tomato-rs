#![windows_subsystem = "windows"]

mod app;
mod custom_styles;

use app::App;
use iced::window;
// use iced::Font;

fn main() {
    let icon = window::icon::from_file("assets/clock.ico").unwrap();
    iced::application(App::default(), App::update, App::view)
        .subscription(App::subscriptions)
        .theme(App::theme)
        .window(window::Settings {
            icon: Some(icon),
            decorations: false,
            size: [230.0, 45.0].into(),
            ..Default::default()
        })
        // .font(include_bytes!("C:/Windows/Fonts/simsun.ttc"))
        // .default_font(Font::with_name("新宋体"))
        .run()
        .unwrap();
}
