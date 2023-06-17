#![allow(non_snake_case)]
use crate::ui::components::flexbox::Flexbox;
use crate::ui::components::label::Label;
use crate::ui::components::wrapper::Wrapper;

use dioxus::prelude::*;

#[derive(PartialEq, Props)]
pub struct TimeLabelProps {
    text: String,
    count: u32,
}

pub fn TimeLabel(cx: Scope<TimeLabelProps>) -> Element {
    cx.render(rsx! {

            Wrapper {
                width: "240px",
                Flexbox{
                    direction: "column",
                    Label {
                        font_size: "24px",
                        text: "{cx.props.text}"
                    },
                    Label {
                        font_size: "48px",
                        text: "{cx.props.count}"
                    },
                }
            }
    })
}
