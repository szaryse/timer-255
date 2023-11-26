#![allow(non_snake_case)]
use crate::contexts::state::TimerState;
use dioxus::prelude::*;
use dioxus_desktop::{
    tao::dpi::{LogicalSize, PhysicalPosition},
    Config, WindowBuilder,
};
use std::time::Duration;

use crate::ui::components::flexbox::Flexbox;
use crate::ui::elements::{controls::Controls, settings::Settings, time_label::TimeLabel};
use crate::ui::global_styles::global_styles;

pub mod contexts;
pub mod ui;

fn main() {
    dioxus_desktop::launch_cfg(
        App,
        Config::new()
            .with_window(
                WindowBuilder::new()
                    .with_inner_size(LogicalSize::new(480, 96))
                    .with_always_on_top(true)
                    .with_title("Timer 255")
                    // INFO: position only for development
                    .with_position(PhysicalPosition::new(2870, 60)),
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
    let is_settings_open = timer_state.read().is_settings_open;

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

    if is_settings_open {
        cx.render(rsx! {
            div {
                style: app_style,
                Flexbox {
                    padding: "8px",
                    height: "100%",
                    justify_content: "space-between",
                    Settings {}
                },
            }
        })
    } else {
        cx.render(rsx! {
            div {
                style: app_style,
                Flexbox {
                    padding: "8px",
                    height: "100%",
                    TimeLabel {
                        count: count.clone()
                    },
                    Controls {},
                },
            }
        })
    }
}
