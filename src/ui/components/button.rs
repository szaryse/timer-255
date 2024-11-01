#![allow(non_snake_case)]
use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct ButtonProps {
    on_click: EventHandler<MouseEvent>,
    #[props(default = "max-content".to_string())]
    width: String,
    #[props(default = "auto".to_string())]
    height: String,
    children: Element,
}

pub fn Button(props: ButtonProps) -> Element {
    rsx! {
        button {
            width: props.width,
            height: props.height,
            font_family: "'Consolas', sans-serif",
            font_size: "16px",
            font_weight: "bold",
            background: "transparent",
            border: 0,
            border_radius: "8px",
            onclick: move |evt| props.on_click.call(evt),
            {props.children}
        }
    }
}
