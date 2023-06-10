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
use crate::ui::elements::time_setter::TimeSetter;
use crate::ui::global_styles::*;

pub mod ui;

fn main() {
    dioxus_desktop::launch_cfg(
        App,
        Config::new()
            .with_window(
                WindowBuilder::new()
                    .with_inner_size(LogicalSize::new(960, 160))
                    .with_always_on_top(true)
                    .with_title("Timer 255 - Dioxus.rs")
                    // INFO: position only for development
                    .with_position(PhysicalPosition::new(2870, -60)),
            )
            .with_custom_head(global_styles().to_string()),
    );
}

#[derive(Clone)]
pub struct ActivityTime {
    // default_time: u32,
    set_time: u32,
}

impl ActivityTime {
    fn decrease(&mut self) {
        if (1..=60).contains(&self.set_time) {
            self.set_time -= 1;
        }
    }
    fn increase(&mut self) {
        if (0..=59).contains(&self.set_time) {
            self.set_time += 1;
        }
    }
}

fn App(cx: Scope) -> Element {
    let app_style = r"
        width: 100vw;
        height: 100vh;
        background-color: #181818;
        color: #c0c0c0;
    ";

    let break_time = ActivityTime {
        // default_time: 5,
        set_time: 5,
    };
    let session_time = ActivityTime {
        // default_time: 25,
        set_time: 25,
    };

    use_shared_state_provider(cx, || vec![break_time, session_time]);

    cx.render(rsx! {
        div {
            style: app_style,
            Flexbox {
                padding: "16px",
                Flexbox {
                    "Buttons"
                }
                Flexbox {
                    "Timer Label"
                }
                Flexbox {
                    Flexbox {
                        direction: "column",
                        Label {
                            text: "Break Length"
                        },
                        TimeSetter {
                            idx: 0
                        }
                    },
                    Flexbox {
                        direction: "column",
                        Label {
                            text: "Session Length"
                        },
                        TimeSetter {
                            idx: 1
                        }
                    }
                }
            },
        }
    })
}
