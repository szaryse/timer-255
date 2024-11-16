#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus::desktop::{use_window, LogicalSize};
use std::time::Duration;
use crate::ui::{components::button::Button, icons::exit::ExitIcon};
use crate::ui::components::view_wrapper::ViewWrapper;
use crate::views::{controls_view::ControlsView, session_view::SessionView, settings_view::SettingsView, timer_view::TimerView};
use crate::ui::global_styles::global_styles;

#[derive(PartialEq, Clone, Copy)]
pub enum Activity {
    Break,
    Session,
    StartingIn,
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

#[derive(PartialEq, Clone, Copy)]
pub enum Views {
    Timer,
    Controls,
    Session,
    Settings,
}

pub fn App() -> Element {
    let window = use_window();

    let mut activity_type = use_signal(|| Activity::StartingIn);
    let mut count = use_signal(|| 5 * 60u32);
    let mut is_counting = use_signal(|| false);
    let mut selected_view = use_signal(|| Views::Timer);
    let mut session_number = use_signal(|| 6i32);
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
    let mut starting_time = use_signal(|| ActivityTime {
        activity_name: "Starting in".to_string(),
        set_time: 5,
        activity_type: Activity::StartingIn,
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
                                session_number -= 1;
                                count.set(session_time().set_time * 60 - 1);
                            }
                            Activity::Session => {
                                activity_type.set(Activity::Break);
                                count.set(break_time().set_time * 60 - 1);
                            }
                            Activity::StartingIn => {
                                activity_type.set(Activity::Session);
                                count.set(session_time().set_time * 60 - 1);
                            }
                        }
                    } else {
                        count -= 1;
                    }
                }
            }
        }
    );

    match selected_view() {
        Views::Timer => {
            rsx! {
                ViewWrapper {
                    TimerView {
                        count: count(),
                        activity_type: activity_type(),
                        break_time: break_time(),
                        session_time: session_time(),
                        starting_time: starting_time(),
                        session_number: session_number(),
                        selected_view: selected_view,
                    },
                }
            }
        }
        Views::Controls => {
            rsx! {
            ViewWrapper {
                ControlsView {
                    is_counting: is_counting,
                    activity_type: activity_type,
                    count: count,
                    break_time: break_time,
                    session_time: session_time,
                    starting_time: starting_time,
                    selected_view: selected_view,
                },
            },
        }
        }
        Views::Session => {
            rsx! {
                ViewWrapper {
                    SessionView {
                        selected_view: selected_view,
                        session_number: session_number,
                        starting_time: starting_time,
                        activity_type: activity_type,
                        count: count,
                        is_counting: is_counting(),
                    }
                }
            }
        }
        Views::Settings => {
            rsx! {
                ViewWrapper {
                    SettingsView {
                        break_time: break_time,
                        session_time: session_time,
                        starting_time: starting_time,
                        is_counting: is_counting(),
                        count: count,
                        activity_type: activity_type,
                        selected_view: selected_view,
                    }
                },
            }
        }
    }
}
