#![allow(non_snake_case)]
use dioxus::prelude::*;

#[derive(Props)]
pub struct FlexboxProps<'a> {
    #[props(default = "row")]
    direction: Option<&'a str>,
    children: Element<'a>,
    #[props(default = "center")]
    justify_content: Option<&'a str>,
}

pub fn Flexbox<'a>(cx: Scope<'a, FlexboxProps<'a>>) -> Element {
    cx.render(rsx! {
        div {
            display: "flex",
            flex_direction: cx.props.direction,
            justify_content: cx.props.justify_content,
            align_items: "center",
            flex_grow: 1,
            padding: "8px",
            &cx.props.children
        }
    })
}
