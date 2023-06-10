#![allow(non_snake_case)]
use contexts::state::{create_initial_times, IsCounting};
use dioxus::prelude::*;
use dioxus_desktop::{
    tao::dpi::{LogicalSize, PhysicalPosition},
    Config, WindowBuilder,
};

use crate::ui::components::label::Label;
use crate::ui::elements::time_label::TimeLabel;
use crate::ui::elements::time_setter::TimeSetter;
use crate::ui::global_styles::*;
use crate::ui::{components::flexbox::Flexbox, elements::controls::Controls};

pub mod contexts;
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

fn App(cx: Scope) -> Element {
    let app_style = r"
        width: 100vw;
        height: 100vh;
        background-color: #181818;
        color: #c0c0c0;
    ";
    let initial_times = create_initial_times();

    use_shared_state_provider(cx, || initial_times);
    use_shared_state_provider(cx, || IsCounting(false));

    cx.render(rsx! {
        div {
            style: app_style,
            Flexbox {
                padding: "16px",
                Controls {
                    idx: 1,
                },
                TimeLabel {
                    idx: 1
                },
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
