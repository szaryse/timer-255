#![allow(non_snake_case)]
use dioxus::prelude::*;

#[derive(Props)]
pub struct WrapperProps<'a> {
    #[props(default = "100%")]
    width: &'a str,
    #[props(default = "auto")]
    height: &'a str,
    children: Element<'a>,
}

pub fn Wrapper<'a>(cx: Scope<'a, WrapperProps<'a>>) -> Element {
    cx.render(rsx! {
        div {
            width: cx.props.width,
            height: cx.props.height,
            flex_shrink: 0,
            &cx.props.children
        }
    })
}
