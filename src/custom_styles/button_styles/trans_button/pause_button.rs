use iced::{Theme, widget};

/// 获取暂停按钮样式。
pub fn pause_icon<'a>(size: f32) -> widget::Svg<'a, Theme> {
    let icon = r#"
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1024 1024" fill="white">
            <path d="M768 912c-44.16 0-80-35.84-80-80V192a80 80 0 0 1 160 0v640c0 44.16-35.84 80-80 80zM256 912c-44.16 0-80-35.84-80-80V192a80 80 0 0 1 160 0v640c0 44.16-35.84 80-80 80z"/>
        </svg>
        "#;
    crate::custom_styles::button_styles::get_icon(size, icon)
}
