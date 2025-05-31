use iced::{
    Color, Theme,
    widget::{self, button},
};

/// 关闭按钮样式。
pub fn close_style(theme: &Theme, status: button::Status) -> button::Style {
    let palette = theme.extended_palette();
    let base = super::styled(palette.primary.strong.text);

    match status {
        button::Status::Active | button::Status::Pressed => base,
        button::Status::Hovered => button::Style {
            background: Some(Color::from_rgb(0.94, 0.22, 0.22).into()),
            ..base
        },
        button::Status::Disabled => super::disabled(base),
    }
}

/// 关闭按钮图标。
pub fn close_icon<'a>(size: f32) -> widget::Svg<'a, Theme> {
    super::get_icon(
        size,
        r#"
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="white">
            <path d="M19 6.41L17.59 5 12 10.59 6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 12 13.41 17.59 19 19 17.59 13.41 12z"/>
        </svg>
        "#,
    )
}
