#![allow(non_snake_case)]
use dioxus::prelude::*;

#[derive(Props)]
pub struct LabelProps<'a> {
    text: &'a str,
}

pub fn Label<'a>(cx: Scope<'a, LabelProps<'a>>) -> Element {
    let label_style = r"
        font-size: 18px;
        font-family: 'Consolas', sans-serif;
        text-align: center;
    ";

    cx.render(rsx! {
        div {
            style: label_style,
            "{cx.props.text}"
        }
    })
}
