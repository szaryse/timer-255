#![allow(non_snake_case)]
use crate::ui::components::flexbox::Flexbox;

use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct ViewWrapperProps {
    children: Element,
}

pub fn ViewWrapper(props: ViewWrapperProps) -> Element {
    rsx! {
        div {
            width: "calc(100vw - 8px)",
            height: "calc(100vh - 8px)",
            background_color: "rgba(0,0,0,0.8)",
            margin: "4px",
            padding: "4px",
            {props.children}
        }
    }
}
