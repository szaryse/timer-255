#![allow(non_snake_case)]
use crate::ui::components::flexbox::Flexbox;
use crate::ui::components::label::Label;
use crate::ui::components::wrapper::Wrapper;

use crate::contexts::state::{ActivityTime, IsCounting};

use dioxus::prelude::*;

#[derive(PartialEq, Props)]
pub struct TimeLabelProps {
    idx: usize,
}

pub fn TimeLabel(cx: Scope<TimeLabelProps>) -> Element {
    let times = use_shared_state::<Vec<ActivityTime>>(cx).unwrap();
    let is_counting = use_shared_state::<IsCounting>(cx).unwrap();

    let set_time = times.read()[cx.props.idx].set_time;
    // TODO: current time from u32
    let mut current_time = "--:--".to_string();

    if !is_counting.read().0 {
        current_time = set_time.to_string();
    }

    cx.render(rsx! {

            Wrapper {
                width: "240px",
                Flexbox{
                    direction: "column",
                    Label {
                        font_size: "24px",
                        text: "{times.read()[cx.props.idx].activity_name}"
                    },
                    Label {
                        font_size: "48px",
                        text: "{current_time}"
                    },
                }
            }
    })
}
