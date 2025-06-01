use iced::{Color, Theme, border, widget::progress_bar::Style};

/// 警告样式。
pub fn progress_warning(_theme: &Theme) -> Style {
    styled(Color::from_rgb(0.52, 0.51, 0.3))
}

/// 成功样式。
pub fn progress_success(theme: &Theme) -> Style {
    let palette = theme.extended_palette();

    styled(palette.success.base.color)
}

/// 错误样式。
pub fn progress_danger(theme: &Theme) -> Style {
    let palette = theme.extended_palette();

    styled(palette.danger.base.color)
}

fn styled(bar: Color) -> Style {
    Style {
        background: Color::TRANSPARENT.into(),
        bar: bar.into(),
        border: border::rounded(2).color(bar).width(1),
    }
}
