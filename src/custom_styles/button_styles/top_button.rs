use iced::{
    Color, Theme,
    widget::{self, button},
};

/// 置顶按钮样式。
pub fn top_style(theme: &Theme, status: button::Status, is_checked: bool) -> button::Style {
    let palette = theme.palette();
    let base = super::styled(palette.text);
    let bg_color = if is_checked {
        palette.primary
    } else {
        Color::TRANSPARENT
    };

    match status {
        button::Status::Active | button::Status::Pressed => button::Style {
            background: Some(bg_color.into()),
            ..base
        },
        button::Status::Hovered => button::Style {
            background: Some(palette.primary.into()),
            ..base
        },
        button::Status::Disabled => super::disabled(base),
    }
}

/// 获取置顶按钮样式。
pub fn top_icon<'a>(size: f32, is_checked: bool) -> widget::Svg<'a, Theme> {
    let icon = if is_checked {
        r#"
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1024 1024" fill="white">
            <path d="M829.2 555.7L704 371.2v-192h22.4c23 0 41.6-18.6 41.6-41.6S749.4 96 726.4 96H297.6c-23 0-41.6 18.6-41.6 41.6s18.6 41.6 41.6 41.6H320v192L194.8 555.7c-1.8 2.7-2.8 5.8-2.8 9v30.5c0 17.7 14.3 32 32 32h246.4v259.2c0 23 18.6 41.6 41.6 41.6s41.6-18.6 41.6-41.6V627.2H800c17.7 0 32-14.3 32-32v-30.5c0-3.2-1-6.4-2.8-9z"/>
        </svg>
        "#
    } else {
        r#"
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1024 1024" fill="white">
            <path d="M629.6 843l41.9-219 135.8-135.8 15.8 15.8c16.2 16.2 42.6 16.2 58.8 0 16.2-16.2 16.2-42.6 0-58.8L578.8 142c-16.2-16.2-42.6-16.2-58.8 0-16.2 16.2-16.2 42.6 0 58.8l15.8 15.8L400 352.5l-219 41.9c-3.1 0.6-6 2.1-8.3 4.4l-21.6 21.6c-12.5 12.5-12.5 32.8 0 45.3l174.2 174.2L142 823.1c-16.2 16.2-16.2 42.6 0 58.8 16.2 16.2 42.6 16.2 58.8 0l183.3-183.3 174.2 174.2c12.5 12.5 32.8 12.5 45.3 0l21.6-21.6c2.3-2.2 3.8-5 4.4-8.2z"/>
        </svg>
        "#
    };
    super::get_icon(size, icon)
}
