use std::time::Duration;

use iced::{
    Element, Length, Subscription, Task,
    alignment::{Horizontal, Vertical},
    application::Title,
    border, event, mouse, padding,
    widget::{self, button, container, progress_bar, text, tooltip},
    window::{self, Level},
};

use crate::custom_styles::{
    close_icon, close_style, pause_icon, play_icon, progress_danger, progress_success,
    progress_warning, reset_icon, rest_icon, top_icon, top_style, trans_style, work_icon,
};

const WORK_SECONDS: i32 = 25 * 60;
const REST_SECONDS: i32 = 5 * 60;

#[derive(Debug, Clone, Copy, PartialEq)]
enum TimerState {
    Working,
    Resting,
    Paused,
}

#[derive(Debug, Clone)]
pub enum Message {
    Topmost(bool),
    ExitApp,
    Drag,
    TimeChanged,
    Work,
    Rest,
    Pause,
    Reset,
    None,
}

#[derive(Debug)]
pub struct App {
    is_topmost: bool,
    state: TimerState,
    remaining_seconds: i32,
}

impl Default for App {
    fn default() -> Self {
        Self {
            is_topmost: false,
            state: TimerState::Working,
            remaining_seconds: WORK_SECONDS,
        }
    }
}

impl App {
    pub fn view(&self) -> Element<Message> {
        let title_bar = widget::column![
            container(widget::row![
                container(with_tooltip(
                    widget::button(top_icon(14.0, self.is_topmost))
                        .style(|t, s| top_style(t, s, self.is_topmost))
                        .padding(3.0)
                        .on_press(Message::Topmost(self.is_topmost))
                        .into(),
                    if self.is_topmost {
                        "Cancel Always On Top"
                    } else {
                        "Always On Top"
                    }
                ))
                .padding(padding::right(3.0)),
                with_tooltip(
                    widget::button(close_icon(14.0))
                        .style(close_style)
                        .padding(3.0)
                        .on_press(Message::ExitApp)
                        .into(),
                    "Close"
                )
            ])
            .width(Length::Fill)
            .align_x(Horizontal::Right),
            container(widget::row![
                container(text(content_label(&self.state)).size(12))
                    .padding(padding::right(1).bottom(1))
                    .height(Length::Fill)
                    .align_y(Vertical::Bottom),
                container(text(content(self.remaining_seconds)).size(14))
                    .height(Length::Fill)
                    .align_y(Vertical::Bottom),
            ])
            .width(Length::Fill)
            .align_x(Horizontal::Right)
        ];

        let main_command = widget::row![with_tooltip(
            button(reset_icon(14.0))
                .style(trans_style)
                .on_press(Message::Reset)
                .into(),
            "Reset"
        )]
        .push(match self.state {
            // 休息状态下，显示工作按钮
            TimerState::Resting => container(with_tooltip(
                button(work_icon(14.0))
                    .on_press(Message::Work)
                    .style(trans_style)
                    .into(),
                "Work",
            ))
            .padding(padding::left(1.0)),

            // 暂停状态下，显示继续按钮
            TimerState::Paused => container(with_tooltip(
                button(play_icon(14.0))
                    .on_press(Message::Work)
                    .style(trans_style)
                    .into(),
                "Work",
            ))
            .padding(padding::left(1.0)),

            // 工作状态下，显示暂停按钮
            TimerState::Working => container(with_tooltip(
                button(pause_icon(14.0))
                    .style(trans_style)
                    .on_press(Message::Pause)
                    .into(),
                "Pause",
            ))
            .padding(padding::left(1.0)),
        })
        .push_maybe(if matches!(self.state, TimerState::Resting) {
            None
        } else {
            // 工作和暂停状态下，显示休息按钮
            let rest_button = container(with_tooltip(
                button(rest_icon(14.0))
                    .style(trans_style)
                    .on_press(Message::Rest)
                    .into(),
                "Rest",
            ))
            .padding(padding::left(1.0));
            Some(rest_button)
        });

        let command =
            container(widget::stack![widget::center(main_command), title_bar]).style(|t| {
                let palette = t.palette();
                container::Style {
                    background: Some(palette.background.into()),
                    border: border::color(palette.primary).width(1),
                    ..Default::default()
                }
            });

        let body = widget::stack![
            progress_bar(0.0..=100.0, progress(&self.state, self.remaining_seconds))
                .height(Length::Fill)
                .width(Length::Fill)
                .style(|t| progress_style(t, &self.state, self.remaining_seconds)),
            container(widget::row![
                container(text(content_label(&self.state)).size(20))
                    .padding(padding::right(10.0))
                    .center_y(Length::Fill),
                container(text(content(self.remaining_seconds)).size(24)).center_y(Length::Fill),
            ])
            .center_x(Length::Fill)
        ];

        widget::hover(body, command)
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Topmost(topmost) => {
                self.is_topmost = !topmost;
                let level = if self.is_topmost {
                    Level::AlwaysOnTop
                } else {
                    Level::Normal
                };
                window::get_latest().and_then(move |id| window::change_level(id, level))
            }
            Message::ExitApp => window::get_latest().and_then(window::close),
            Message::Drag => window::get_latest().and_then(window::drag),
            Message::TimeChanged => {
                if !matches!(self.state, TimerState::Paused) {
                    self.remaining_seconds -= 1;
                }
                Task::none()
            }
            Message::Work => {
                // 若当前状态为暂停，则恢复工作状态；否则，重新计时
                if !matches!(self.state, TimerState::Paused) {
                    self.remaining_seconds = WORK_SECONDS;
                }
                self.state = TimerState::Working;
                Task::none()
            }
            Message::Pause => {
                self.state = TimerState::Paused;
                Task::none()
            }
            Message::Rest => {
                self.state = TimerState::Resting;
                self.remaining_seconds = REST_SECONDS;
                Task::none()
            }
            Message::Reset => {
                self.remaining_seconds = if matches!(self.state, TimerState::Working) {
                    WORK_SECONDS
                } else {
                    REST_SECONDS
                };
                self.state = TimerState::Working;
                Task::none()
            }
            _ => Task::none(),
        }
    }

    pub fn subscriptions(&self) -> Subscription<Message> {
        let time_event = iced::time::every(Duration::from_secs(1)).map(|_| Message::TimeChanged);

        let mouse_event = event::listen().map(|e| match e {
            iced::Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => Message::Drag,
            _ => Message::None,
        });

        Subscription::batch([time_event, mouse_event])
    }

    pub fn theme(&self) -> iced::Theme {
        iced::theme::Theme::Dark
    }
}

impl Title<App> for App {
    fn title(&self, _state: &App) -> String {
        "Tomato Timer".to_owned()
    }
}

fn content_label(is_working: &TimerState) -> &str {
    if matches!(is_working, TimerState::Working) {
        "Work"
    } else {
        "Rest"
    }
}

fn content(seconds: i32) -> String {
    let seconds = seconds.abs();

    let minute = seconds / 60;
    let second = seconds % 60;
    format!("{}:{:02}", minute, second)
}

fn progress(state: &TimerState, seconds: i32) -> f32 {
    let total = if matches!(state, TimerState::Working) {
        WORK_SECONDS
    } else {
        REST_SECONDS
    };

    (total - seconds) as f32 / total as f32 * 100.0
}

fn progress_style(
    theme: &widget::Theme,
    state: &TimerState,
    seconds: i32,
) -> widget::progress_bar::Style {
    if seconds < 0 {
        progress_danger(theme)
    } else {
        let progress = progress(state, seconds);
        if progress > 80.0 {
            progress_warning(theme)
        } else {
            progress_success(theme)
        }
    }
}

fn with_tooltip<'a>(content: Element<'a, Message>, tip: &'a str) -> Element<'a, Message> {
    tooltip(content, text(tip).size(10), tooltip::Position::FollowCursor).into()
}
