#![allow(non_snake_case)]
use contexts::state::{Activity, TimerState};
use dioxus::prelude::*;
use dioxus_desktop::{
    tao::dpi::{LogicalSize, PhysicalPosition},
    Config, WindowBuilder,
};
use std::time::Duration;

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
                    .with_title("Timer 255")
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

    use_shared_state_provider(cx, || TimerState::new());

    let timer_state = use_shared_state::<TimerState>(cx).unwrap();
    let is_counting = timer_state.read().is_counting;
    let count = timer_state.read().count;

    use_future(cx, &is_counting, move |_| {
        let timer_state = timer_state.clone();

        async move {
            loop {
                tokio::time::sleep(Duration::from_millis(1000)).await;
                if is_counting {
                    timer_state.write().tick();
                }
            }
        }
    });

    cx.render(rsx! {
        div {
            style: app_style,
            Flexbox {
                padding: "16px",
                height: "100%",
                TimeLabel {
                    count: count.clone()
                },
                Controls {
                    count: count
                },
                Flexbox {
                    Flexbox {
                        direction: "column",
                        Label {
                            text: "Break Length"
                        },
                        TimeSetter {
                            activity_type: Activity::Break
                        }
                    },
                    Flexbox {
                        direction: "column",
                        Label {
                            text: "Session Length"
                        },
                        TimeSetter {
                            activity_type: Activity::Session
                        }
                    }
                }
            },
        }
    })
}
