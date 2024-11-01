#![allow(non_snake_case)]
use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct WrapperProps {
    #[props(default = "100%".to_string())]
    width: String,
    #[props(default = "auto".to_string())]
    height: String,
    #[props(default = "0".to_string())]
    margin: String,
    #[props(default = "0".to_string())]
    padding: String,
    children: Element,
}

pub fn Wrapper(props: WrapperProps) -> Element {
    rsx! {
        div {
            width: props.width,
            height: props.height,
            margin: props.margin,
            padding: props.padding,
            flex_shrink: 0,
            {props.children}
        }
    }
}
