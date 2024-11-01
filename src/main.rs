#![allow(non_snake_case)]
use crate::{
    contexts::state::TimerState,
    ui::{components::button::Button, elements::view_wrapper::ViewWrapper, icons::exit::ExitIcon},
};
use dioxus::prelude::*;
use dioxus::desktop::{
    tao::dpi::{LogicalSize, PhysicalPosition},
    Config, WindowBuilder,
    use_window,
};
use std::time::Duration;

use crate::ui::elements::{controls::Controls, settings::Settings, time_label::TimeLabel};
use crate::ui::global_styles::global_styles;

pub mod contexts;
pub mod ui;

fn main() {
    LaunchBuilder::desktop()
        .with_cfg(Config::new()
            .with_window(
                WindowBuilder::new()
                    .with_inner_size(LogicalSize::new(360, 64))
                    .with_always_on_top(true)
                    .with_title("Timer 255")
                    .with_resizable(false)
                    // INFO: position only for development
                    .with_position(PhysicalPosition::new(2870, 60)),
            )
            .with_custom_head(global_styles())
        )
        .launch(App);
}

fn App() -> Element {
    // use_shared_state_provider(cx, || TimerState::new());
    // let timer_state = use_shared_state::<TimerState>(cx).unwrap();

    // let is_counting = timer_state.read().is_counting;
    // let count = timer_state.read().count;
    let is_timer_open = true; // timer_state.read().is_timer_open;
    let is_controls_open = false; // timer_state.read().is_controls_open;
    let is_settings_open = false; // timer_state.read().is_settings_open;
    let window = use_window();

    let _ = use_resource(
        move || async move {
            // let timer_state = timer_state.clone();

            //async move {
            loop {
                tokio::time::sleep(Duration::from_millis(1000)).await;

                println!("tick");

                // if is_counting {
                //     timer_state.write().tick();
                // }
            }
            //}
        });

    if is_timer_open {
        rsx! {
            ViewWrapper {
                TimeLabel {
                    count: 1, // count.clone()
                },
            },
        }
    } else if is_controls_open {
        rsx! {
            ViewWrapper {
                Controls {},
            },
        }
    } else if is_settings_open {
        rsx! {
            ViewWrapper{
                Settings {}
            },
        }
    } else {
        rsx! {
            ViewWrapper {
                justify_content: "space-evenly",
                "Something went wrong.",
                Button {
                    on_click: move |_event| {
                        window.close();
                    },
                    ExitIcon {}
                },
            }
        }
    }
}
