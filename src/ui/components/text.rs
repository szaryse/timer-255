#![allow(non_snake_case)]
use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct TextProps {
    #[props(default = "18px".to_string())]
    font_size: String,
    text: String,
    #[props(default = "inherit".to_string())]
    color: String,
    #[props(default = "center".to_string())]
    text_align: String,
    #[props(default = "auto".to_string())]
    line_height: String,
}

pub fn Text(props: TextProps) -> Element {
    rsx! {
        div {
            text_align: "{ props.text_align }",
            font_size: "{ props.font_size }",
            color: "{ props.color }",
            line_height: "{ props.line_height }",
            white_space: "nowrap",
            {props.text}
        }
    }
}
