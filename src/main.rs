#![allow(non_snake_case)]
use contexts::state::{create_initial_times, ActivityTime, Timer};
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

    use_shared_state_provider(cx, || create_initial_times());
    use_shared_state_provider(cx, || Timer {
        is_counting: false,
        idx: 1,
        show_set_time: true,
        is_pausing: false,
    });

    let timer_config = use_shared_state::<Timer>(cx).unwrap();
    let times = use_shared_state::<Vec<ActivityTime>>(cx).unwrap();

    let idx = timer_config.read().idx;
    let count = use_state(cx, || times.read()[idx].set_time * 60);
    let is_counting = timer_config.read().is_counting;
    let show_set_time = timer_config.read().show_set_time;

    if **count == 0 {
        let new_idx = match idx {
            0 => 1,
            1 => 0,
            _ => unreachable!(),
        };
        *count.make_mut() = times.read()[new_idx].set_time * 60;
        timer_config.write().idx = new_idx;
    }

    use_future(cx, &is_counting, move |_| {
        let mut count = count.clone();
        async move {
            loop {
                tokio::time::sleep(Duration::from_millis(1000)).await;
                if is_counting {
                    count -= 1;
                }
            }
        }
    });

    let current_text = times.read()[idx].activity_name.clone();

    cx.render(rsx! {
        div {
            style: app_style,
            Flexbox {
                padding: "16px",
                height: "100%",
                Controls {
                    idx: 1,
                    count: count
                },
                TimeLabel {
                    text: current_text,
                    count: match is_counting {
                        true => **count,
                        false => {
                            match show_set_time {
                                true => times.read()[idx].set_time * 60,
                                false => **count
                            }
                        }
                    }
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
