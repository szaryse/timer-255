#![allow(non_snake_case)]
use dioxus::prelude::*;

#[derive(Props)]
pub struct LabelProps<'a> {
    #[props(default = "18px")]
    font_size: &'a str,
    text: &'a str,
}

pub fn Label<'a>(cx: Scope<'a, LabelProps<'a>>) -> Element {
    cx.render(rsx! {
        div {
            font_family: "'Consolas', sans-serif",
            text_align: "center",
            font_size: cx.props.font_size,
            margin: "0 4px",
            "{cx.props.text}"
        }
    })
}
