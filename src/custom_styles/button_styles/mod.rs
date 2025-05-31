mod close_button;
mod top_button;
mod trans_button;

use iced::{
    border, widget::{self, button, svg}, Color, Theme
};

pub use close_button::*;
pub use top_button::*;
pub use trans_button::*;

/// 获取置顶按钮样式。
fn get_icon<'a>(size: f32, icon: &str) -> widget::Svg<'a, Theme> {
    let handle = svg::Handle::from_memory(icon.as_bytes().to_vec());
    widget::svg(handle).width(size).height(size)
}

/// 按钮样式。
fn styled(text_color: Color) -> button::Style {
    button::Style {
        background: Some(Color::TRANSPARENT.into()),
        text_color,
        border: border::rounded(2),
        ..button::Style::default()
    }
}

/// 按钮禁用样式。
fn disabled(style: button::Style) -> button::Style {
    button::Style {
        background: style
            .background
            .map(|background| background.scale_alpha(0.5)),
        text_color: style.text_color.scale_alpha(0.5),
        ..style
    }
}