#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_desktop::{
    tao::dpi::{LogicalSize, PhysicalPosition},
    Config, WindowBuilder,
};

use crate::ui::components::button::Button;
use crate::ui::components::flexbox::Flexbox;
use crate::ui::components::label::Label;
use crate::ui::components::wrapper::Wrapper;
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
                    .with_position(PhysicalPosition::new(3470, 620)),
            )
            .with_custom_head(global_styles().to_string()),
    );
}

fn App(cx: Scope) -> Element {
    let app_style = r"
        margin: 0;
        padding: 8px;
        width: 100vw;
        height: 100vh;
        background-color: #181818;
        color: #c0c0c0;
    ";
    let default_times = vec![5, 25];

    let times = use_state(cx, || default_times);

    cx.render(rsx! {
        div {
            style: app_style,
            Flexbox {
                Flexbox {
                    direction: "column",
                    Label {
                        text: "Break Length"
                    },
                    Flexbox{
                        Button {
                            on_click: move |_event| {
                                let current_time = times.make_mut()[0];
                                if (1..=60).contains(&current_time) {
                                    times.make_mut()[0] -= 1;
                                }
                            },
                            text: "-"
                        },
                        Wrapper {
                            width: "40px",
                            Label {
                                font_size: "24px",
                                text: "{times[0]}"
                            },
                        },
                        Button {
                            on_click: move |_event| {
                                let current_time = times.make_mut()[0];
                                if (0..=59).contains(&current_time) {
                                    times.make_mut()[0] += 1;
                                }
                            },
                            text: "+"
                        }
                    }
                },
                Flexbox {
                    direction: "column",
                    Label {
                        text: "Session Length"
                    },
                    Flexbox{
                        Button {
                            on_click: move |_event| {
                                let current_time = times.make_mut()[1];
                                if (1..=60).contains(&current_time) {
                                    times.make_mut()[1] -= 1;
                                }
                            },
                            text: "-"
                        },
                        Wrapper {
                            width: "40px",
                            Label {
                                font_size: "24px",
                                text: "{times[1]}"
                            },
                        }
                        Button {
                            on_click: move |_event| {
                                let current_time = times.make_mut()[1];
                                if (0..=59).contains(&current_time) {
                                    times.make_mut()[1] += 1;
                                }
                            },
                            text: "+"
                        }
                    }
                }

            },
        }
    })
}
