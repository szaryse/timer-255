use dioxus::prelude::*;
use dioxus::desktop::{LogicalSize, use_window};
use std::time::Duration;
use crate::{
    ui::{components::button::Button, elements::view_wrapper::ViewWrapper, icons::exit::ExitIcon},
};
use crate::ui::elements::{controls::Controls, settings::Settings, timer_view::TimerView};
use crate::ui::global_styles::global_styles;

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

#[allow(non_snake_case)]
pub fn App() -> Element {
    let window = use_window();

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

    use_effect(move || window.set_inner_size(LogicalSize::new(340, 56)));

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
        }
    );

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
        return rsx!();
    }
}
