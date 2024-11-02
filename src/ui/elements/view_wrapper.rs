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
    rsx! {
        div {
            width: "calc(100% - 8px)",
            height: "calc(100vh - 8px)",
            background_color: "rgba(0,0,0,0.8)",
            margin: "4px",
            width: "calc(100% - 8px)",
            padding: "4px",
            Flexbox {
                padding: "8px",
                height: "100%",
                justify_content: props.justify_content,
                {props.children}
            }
        }
    }
}
