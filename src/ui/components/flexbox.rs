#![allow(non_snake_case)]
use dioxus::prelude::*;

#[derive(Props)]
pub struct FlexboxProps<'a> {
    #[props(default = "row")]
    direction: &'a str,
    #[props(default = "center")]
    justify_content: &'a str,
    #[props(default = "0")]
    padding: &'a str,
    children: Element<'a>,
}

pub fn Flexbox<'a>(cx: Scope<'a, FlexboxProps<'a>>) -> Element {
    cx.render(rsx! {
        div {
            display: "flex",
            flex_direction: cx.props.direction,
            justify_content: cx.props.justify_content,
            align_items: "center",
            flex_grow: 1,
            width: "100%",
            height: "100%",
            padding: cx.props.padding,
            &cx.props.children
        }
    })
}
