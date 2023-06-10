#![allow(non_snake_case)]
use dioxus::prelude::*;

#[derive(Props)]
pub struct ButtonProps<'a> {
    on_click: EventHandler<'a, MouseEvent>,
    children: Element<'a>,
}

pub fn Button<'a>(cx: Scope<'a, ButtonProps<'a>>) -> Element<'a> {
    cx.render(rsx!(button {
        width: "40px",
        height: "40px",
        font_family: "'Consolas', sans-serif",
        font_size: "16px",
        font_weight: "bold",
        background: "transparent",
        border: 0,
        border_radius: "8px",
        onclick: move |evt| cx.props.on_click.call(evt),
        &cx.props.children,
    }))
}
