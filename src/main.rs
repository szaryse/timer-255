#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_desktop::{
    tao::dpi::{LogicalSize, PhysicalPosition},
    Config, WindowBuilder,
};

use crate::ui::components::flexbox::Flexbox;
use crate::ui::components::label::Label;
use crate::ui::global_styles::*;

pub mod ui;

fn main() {
    dioxus_desktop::launch_cfg(
        App,
        Config::new()
            .with_window(
                WindowBuilder::new()
                    .with_inner_size(LogicalSize::new(360, 360))
                    .with_always_on_top(true)
                    .with_title("Timer 255 - Dioxus.rs")
                    // INFO: position only for development
                    .with_position(PhysicalPosition::new(3470, 630)),
            )
            .with_custom_head(global_styles().to_string()),
    );
}

fn App(cx: Scope) -> Element {
    let app_style = r"
        margin: 0;
        padding: 0;
        width: 100vw;
        height: 100vh;
        background-color: #181818;
        color: #c0c0c0;
    ";

    cx.render(rsx! {
        div {
            style: app_style,
            Flexbox {
                    Flexbox {
                        direction: "column",
                        Label {
                            text: "Break Length"
                        },
                        Label {
                            text: "5"
                        },
              },
                Flexbox {
                    direction: "column",
                        Label {
                            text: "Session Length"
                        },
                        Label {
                            text: "25"
                        },
                }
        },
    }
    })
}
