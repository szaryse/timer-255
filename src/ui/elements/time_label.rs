#![allow(non_snake_case)]
use crate::contexts::state::{TimerAction, TimerState};
use crate::ui::components::button::Button;
use crate::ui::components::flexbox::Flexbox;
use crate::ui::components::label::Label;
use crate::ui::icons::chevron_right::ChevronRightIcon;

use dioxus::prelude::*;

#[derive(PartialEq, Props)]
pub struct TimeLabelProps {
    count: u32,
}

pub fn TimeLabel(cx: Scope<TimeLabelProps>) -> Element {
    let timer_state = use_shared_state::<TimerState>(cx).unwrap();

    let current_text = timer_state.read().select_label();
    let count = timer_state.read().count;
    let is_counting = timer_state.read().is_counting;

    let time = match is_counting {
        true => cx.props.count,
        false => count,
    };

    let minutes = time / 60;
    let seconds = time - minutes * 60;
    let time = format!("{}:{seconds:0>2}", minutes, seconds = seconds);

    let color = match cx.props.count {
        (0..=30) => 0,
        (31..=210) => cx.props.count - 30,
        _ => 180,
    };

    cx.render(rsx! {
        Flexbox {
            justify_content: "space-between",
            Label {
                font_size: "24px",
                text: "{current_text}"
            },
            Label {
                font_size: "48px",
                text: "{time}",
                color: "hsl({color}, 100%, 50%)",
            },
            Flexbox{
                width: "40px",
                flex_grow: 0,
                Button {
                    on_click: move |_event| {
                        timer_state.write().reduce(TimerAction::GoToControls);
                    },
                    ChevronRightIcon {},
                },
            }
        }
    })
}
