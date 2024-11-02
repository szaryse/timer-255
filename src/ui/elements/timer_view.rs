#![allow(non_snake_case)]

use dioxus::prelude::*;
use crate::{Activity, ActivityTime};
use crate::ui::components::button::Button;
use crate::ui::components::flexbox::Flexbox;
use crate::ui::components::text::Text;
use crate::ui::icons::chevron_right::ChevronRightIcon;

#[derive(PartialEq, Props, Clone)]
pub struct TimeLabelProps {
    count: u32,
    activity_type: Activity,
    break_time: ActivityTime,
    session_time: ActivityTime,
    is_timer_open: Signal<bool>,
    is_controls_open: Signal<bool>,
}

pub fn TimerView(mut props: TimeLabelProps) -> Element {
    let current_text = match props.activity_type {
        Activity::Break => props.break_time.activity_name,
        Activity::Session => props.session_time.activity_name,
    };

    let minutes = props.count / 60;
    let seconds = props.count - minutes * 60;
    let time = format!("{}:{seconds:0>2}", minutes, seconds = seconds);

    let color = match props.count {
        (0..=30) => 0,
        (31..=210) => props.count - 30,
        _ => 180,
    };

    rsx! {
        Flexbox {
            justify_content: "space-between",
            Text {
                font_size: "24px",
                text: "{current_text}"
            },
            Text {
                font_size: "48px",
                text: "{time}",
                color: "hsl({color}, 100%, 50%)",
            },
            Flexbox{
                width: "40px",
                flex_grow: 0,
                Button {
                    on_click: move |_event| {
                        props.is_controls_open.set(true);
                        props.is_timer_open.set(false);
                    },
                    ChevronRightIcon {},
                },
            }
        }
    }
}
