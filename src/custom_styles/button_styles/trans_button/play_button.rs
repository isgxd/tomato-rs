use iced::{Theme, widget};

/// 获取播放按钮样式。
pub fn play_icon<'a>(size: f32) -> widget::Svg<'a, Theme> {
    let icon = r#"
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1024 1024" fill="white">
            <path d="M844.704269 475.730473L222.284513 116.380385a43.342807 43.342807 0 0 0-65.025048 37.548353v718.692951a43.335582 43.335582 0 0 0 65.025048 37.541128l622.412531-359.342864a43.357257 43.357257 0 0 0 0.007225-75.08948z"/>
        </svg>
        "#;
    crate::custom_styles::button_styles::get_icon(size, icon)
}
