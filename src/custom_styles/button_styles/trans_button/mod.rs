use iced::{Theme, widget::button};

mod pause_button;
mod play_button;
mod reset_button;
mod rest_button;
mod work_button;

pub use pause_button::*;
pub use play_button::*;
pub use reset_button::*;
pub use rest_button::*;
pub use work_button::*;

/// 置顶按钮样式。
pub fn trans_style(theme: &Theme, status: button::Status) -> button::Style {
    let palette = theme.extended_palette();
    let base = super::styled(palette.primary.strong.text);

    match status {
        button::Status::Active | button::Status::Pressed => base,
        button::Status::Hovered => button::Style {
            background: Some(palette.primary.strong.color.into()),
            ..base
        },
        button::Status::Disabled => super::disabled(base),
    }
}
