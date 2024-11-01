#![allow(non_snake_case)]
use crate::ui::components::flexbox::Flexbox;

use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct ViewWrapperProps {
    #[props(default = "center".to_string())]
    justify_content: String,
    children: Element,
}

pub fn ViewWrapper(props: ViewWrapperProps) -> Element {
    let app_style = r"
        width: 100vw;
        height: 100vh;
        background-color: #181818;
        color: #c0c0c0;
    ";

    rsx! {
        div {
            style: app_style,
            Flexbox {
                padding: "8px",
                height: "100%",
                justify_content: props.justify_content,
                {props.children}
            }
        }
    }
}
