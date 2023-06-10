#![allow(non_snake_case)]
use crate::ui::components::button::Button;
use crate::ui::components::flexbox::Flexbox;
use crate::ui::components::label::Label;
use crate::ui::components::wrapper::Wrapper;

use crate::ActivityTime;

use dioxus::prelude::*;

#[derive(PartialEq, Props)]
pub struct TimeSetterProps {
    idx: usize,
}

pub fn TimeSetter(cx: Scope<TimeSetterProps>) -> Element {
    let times = use_shared_state::<Vec<ActivityTime>>(cx).unwrap();

    cx.render(rsx! {
        Flexbox{
            padding: "8px",
            Button {
                on_click: move |_event| {
                    times.write()[cx.props.idx].decrease();
                },
                text: "-"
            },
            Wrapper {
                width: "40px",
                Label {
                    font_size: "24px",
                    text: "{times.read()[cx.props.idx].set_time}"
                },
            },
            Button {
                on_click: move |_event| {
                    times.write()[cx.props.idx].increase();
                },
                text: "+"
            }
        }
    })
}
