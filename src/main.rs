#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus::desktop::{
    tao::dpi::{LogicalSize, PhysicalPosition},
    Config, WindowBuilder,
    use_window,
};
use std::time::Duration;
use crate::{
    ui::{components::button::Button, elements::view_wrapper::ViewWrapper, icons::exit::ExitIcon},
};
use crate::ui::elements::{controls::Controls, settings::Settings, timer_view::TimerView};
use crate::ui::global_styles::global_styles;

pub mod ui;

#[derive(PartialEq, Clone, Copy)]
pub enum Activity {
    Break,
    Session,
}

#[derive(PartialEq, Clone)]
pub struct ActivityTime {
    pub activity_name: String,
    pub set_time: u32,
    pub activity_type: Activity,
}

#[derive(PartialEq, Clone)]
pub struct Timer {
    pub is_counting: bool,
    pub idx: usize,
    pub show_set_time: bool,
    pub is_pausing: bool,
}

fn main() {
    LaunchBuilder::desktop()
        .with_cfg(Config::new()
            .with_window(
                WindowBuilder::new()
                    // todo start with height 0 and change to 56px
                    .with_inner_size(LogicalSize::new(360, 80))
                    .with_always_on_top(true)
                    .with_title("Timer 255")
                    .with_resizable(false)
                    // INFO: position only for development
                    .with_position(PhysicalPosition::new(2870, 60)),
            )
            .with_custom_head(global_styles())
        )
        .launch(App);
}

fn App() -> Element {
    let mut activity_type = use_signal(|| Activity::Session);
    let mut count = use_signal(|| 25 * 60);
    let mut is_counting = use_signal(|| false);
    let mut is_timer_open = use_signal(|| true);
    let mut is_controls_open = use_signal(|| false);
    let mut is_settings_open = use_signal(|| false);
    let mut break_time = use_signal(|| ActivityTime {
        activity_name: "Break".to_string(),
        set_time: 5,
        activity_type: Activity::Break,
    });
    let mut session_time = use_signal(|| ActivityTime {
        activity_name: "Session".to_string(),
        set_time: 25,
        activity_type: Activity::Session,
    });

    let window = use_window();

    let _ = use_resource(
        move || async move {
            loop {
                tokio::time::sleep(Duration::from_millis(1000)).await;

                if is_counting() {
                    if count() == 0 {
                        match activity_type() {
                            Activity::Break => {
                                activity_type.set(Activity::Session);
                                count.set(session_time().set_time * 60 - 1);
                            }
                            Activity::Session => {
                                activity_type.set(Activity::Break);
                                count.set(break_time().set_time * 60 - 1);
                            }
                        }
                    } else {
                        count -= 1;
                    }
                }
            }
        });

    if is_timer_open() {
        rsx! {
            ViewWrapper {
                TimerView {
                    count: count(),
                    activity_type: activity_type(),
                    break_time: break_time(),
                    session_time: session_time(),
                    is_timer_open: is_timer_open,
                    is_controls_open: is_controls_open,
                },
            },
        }
    } else if is_controls_open() {
        rsx! {
            ViewWrapper {
                Controls {
                    is_timer_open: is_timer_open,
                    is_controls_open:is_controls_open,
                    is_counting: is_counting,
                    activity_type: activity_type,
                    count: count,
                    break_time: break_time,
                    session_time: session_time,
                    is_settings_open: is_settings_open,
                },
            },
        }
    } else if is_settings_open() {
        rsx! {
            ViewWrapper{
                Settings {
                    is_settings_open: is_settings_open,
                    is_controls_open:is_controls_open,
                    break_time: break_time,
                    session_time: session_time,
                    is_counting: is_counting(),
                    count: count,
                    activity_type: activity_type,
                }
            },
        }
    } else {
        rsx! {
            ViewWrapper {
                justify_content: "space-evenly",
                "Something went wrong.",
                Button {
                    on_click: move |_event| {
                        window.close();
                    },
                    ExitIcon {}
                },
            }
        }
    }
}
