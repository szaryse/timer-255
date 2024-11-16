#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus::desktop::use_window;
use crate::app::{Activity, ActivityTime};
use crate::ui::components::{button::Button, flexbox::Flexbox, text::Text};
use crate::ui::elements::time_setter::TimeSetter;
use crate::ui::icons::chevron_left::ChevronLeftIcon;
use crate::ui::icons::exit::ExitIcon;

#[derive(PartialEq, Props, Clone)]
pub struct SettingsProps {
    is_controls_open: Signal<bool>,
    is_settings_open: Signal<bool>,
    activity_type: Signal<Activity>,
    break_time: Signal<ActivityTime>,
    session_time: Signal<ActivityTime>,
    starting_time: Signal<ActivityTime>,
    is_counting: bool,
    count: Signal<u32>,
}

pub fn Settings(mut props: SettingsProps) -> Element {
    let window = use_window();

    rsx! {
        Flexbox {
            width: "100%",
            height: "40px",
            justify_content: "space-between",
            Flexbox {
                width: "40px",
                Button {
                    width: "40px",
                    on_click: move |_event| {
                        props.is_settings_open.set(false);
                        props.is_controls_open.set(true);
                    },
                    ChevronLeftIcon {},
                },
            },
            Flexbox {
                direction: "column",
                justify_content: "space-between",
                Flexbox {
                    height: "20px",
                    justify_content: "space-between",
                    Text {
                        font_size: "18px",
                        line_height: "20px",
                        text: "Session Length"
                    },
                    TimeSetter {
                        activity_type_label: Activity::Session,
                        activity_type: props.activity_type,
                        break_time: props.break_time,
                        session_time: props.session_time,
                        is_counting: props.is_counting,
                        count: props.count,
                        starting_time: props.starting_time,
                    }
                },
                Flexbox {
                    height: "20px",
                    justify_content: "space-between",
                    Text {
                        font_size: "18px",
                        line_height: "20px",
                        text: "Break Length"
                    },
                    TimeSetter {
                        activity_type_label: Activity::Break,
                        activity_type: props.activity_type,
                        break_time: props.break_time,
                        session_time: props.session_time,
                        is_counting: props.is_counting,
                        count: props.count,
                        starting_time: props.starting_time,
                    }
                },

            },
            Flexbox {
                width: "40px",
                Button {
                    width: "40px",
                    on_click: move |_event| {
                        window.close();
                    },
                    ExitIcon {}
                },
            }
        }
    }
}
