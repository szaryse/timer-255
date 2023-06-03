#![allow(non_snake_case)]
use dioxus::prelude::*;

#[derive(Props)]
pub struct ButtonProps<'a> {
    on_click: EventHandler<'a, MouseEvent>,
    text: &'a str,
}

pub fn Button<'a>(cx: Scope<'a, ButtonProps<'a>>) -> Element<'a> {
    cx.render(rsx!(button {
        width: "40px",
        height: "40px",
        font_family: "'Consolas', sans-serif",
        font_size: "16px",
        font_weight: "bold",
        background: "transparent",
        color: "#00FF00",
        border: "2px solid #008000",
        border_radius: "8px",
        onclick: move |evt| cx.props.on_click.call(evt),
        "{cx.props.text}"
    }))
}
