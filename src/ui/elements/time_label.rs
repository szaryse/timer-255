#![allow(non_snake_case)]
use crate::contexts::state::{ActivityTime, Timer};
use crate::ui::components::flexbox::Flexbox;
use crate::ui::components::label::Label;
use crate::ui::components::wrapper::Wrapper;

use dioxus::prelude::*;

#[derive(PartialEq, Props)]
pub struct TimeLabelProps {
    // text: String,
    count: u32,
}

pub fn TimeLabel(cx: Scope<TimeLabelProps>) -> Element {
    let times = use_shared_state::<Vec<ActivityTime>>(cx).unwrap();
    let timer_config = use_shared_state::<Timer>(cx).unwrap();

    let current_text = times.read()[timer_config.read().idx].activity_name.clone();
    let is_counting = timer_config.read().is_counting;
    let show_set_time = timer_config.read().show_set_time;
    let set_time = times.read()[timer_config.read().idx].set_time;

    let time = match is_counting {
        true => cx.props.count,
        false => match show_set_time {
            true => set_time * 60,
            false => cx.props.count,
        },
    };

    let minutes = time / 60;
    let seconds = time - minutes * 60;
    let time = format!("{}:{seconds:0>2}", minutes, seconds = seconds);

    let color = match cx.props.count {
        (0..=180) => cx.props.count,
        _ => 180,
    };

    cx.render(rsx! {
            Wrapper {
                width: "240px",
                Flexbox{
                    direction: "column",
                    Label {
                        font_size: "24px",
                        text: "{current_text}"
                    },
                    Label {
                        font_size: "48px",
                        text: "{time}",
                        color: "hsl({color}, 100%, 50%)",
                    },
                }
            }
    })
}
