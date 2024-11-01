#![allow(non_snake_case)]
use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct FlexboxProps {
    #[props(default = "row".to_string())]
    direction: String,
    #[props(default = "center".to_string())]
    justify_content: String,
    #[props(default = "center".to_string())]
    align_items: String,
    #[props(default = "0".to_string())]
    padding: String,
    #[props(default = "auto".to_string())]
    height: String,
    #[props(default = "100%".to_string())]
    width: String,
    #[props(default = "1".to_string())]
    flex_grow: String,
    #[props(default = "0".to_string())]
    outline: String,
    children: Element,
}

pub fn Flexbox(props: FlexboxProps) -> Element {
    rsx! {
        div {
            display: "flex",
            flex_direction: "{ props.direction }",
            justify_content: "{ props.justify_content }",
            align_items: "{ props.align_items }",
            flex_grow: "{ props.flex_grow }",
            width: "{ props.width }",
            height: "{ props.height }",
            padding: "{ props.padding }",
            outline: "{props.outline}",
            {props.children}
        }
    }
}