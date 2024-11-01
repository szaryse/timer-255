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
    #[props(default = "100%")]
    width: &'a str,
    #[props(default = "auto")]
    height: &'a str,
    #[props(default = 1)]
    flex_grow: i64,
    children: Element<'a>,
}

pub fn Flexbox<'a>(cx: Scope<'a, FlexboxProps<'a>>) -> Element {
    cx.render(rsx! {
        div {
            display: "flex",
            flex_direction: cx.props.direction,
            justify_content: cx.props.justify_content,
            align_items: "center",
            flex_grow: cx.props.flex_grow,
            width: cx.props.width,
            height: cx.props.height,
            padding: cx.props.padding,
            // border: "1px solid red",
            &cx.props.children
        }
    })
}
