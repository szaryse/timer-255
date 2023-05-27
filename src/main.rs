#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_desktop::{
    tao::dpi::{LogicalSize, PhysicalPosition},
    Config, WindowBuilder,
};

fn global_styles() -> &'static str {
    r"
        <style>
            * {
                box-sizing:border-box;
            }
            body {
                margin:0;
            }
        </style>"
}

fn main() {
    dioxus_desktop::launch_cfg(
        App,
        Config::new()
            .with_window(
                WindowBuilder::new()
                    .with_inner_size(LogicalSize::new(360, 360))
                    // INFO: position only for development
                    .with_position(PhysicalPosition::new(3470, 680)),
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
            "Hello, world!"
        }
    })
}
