#![allow(non_snake_case)]
use crate::ui::components::flexbox::Flexbox;

use dioxus::prelude::*;

#[derive(Props)]
pub struct ViewWrapperProps<'a> {
    #[props(default = "center")]
    justify_content: &'a str,
    children: Element<'a>,
}

pub fn ViewWrapper<'a>(cx: Scope<'a, ViewWrapperProps<'a>>) -> Element {
    let app_style = r"
        width: 100vw;
        height: 100vh;
        background-color: #181818;
        color: #c0c0c0;
    ";

    cx.render(rsx! {
        div {
            style: app_style,
            Flexbox {
                padding: "8px",
                height: "100%",
                justify_content: cx.props.justify_content,
                &cx.props.children
            }
        }
    })
}
