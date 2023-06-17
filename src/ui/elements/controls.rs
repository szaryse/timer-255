#![allow(non_snake_case)]
use crate::ui::components::button::Button;
use crate::ui::components::flexbox::Flexbox;
use crate::ui::components::wrapper::Wrapper;
use crate::ui::icons::next_icon::NextIcon;
use crate::ui::icons::pause_icon::PauseIcon;
use crate::ui::icons::play_icon::PlayIcon;

use crate::contexts::state::{ActivityTime, Timer};
use crate::ui::icons::reset_icon::ResetIcon;

use dioxus::prelude::*;

#[derive(PartialEq, Props)]
pub struct ControlsProps<'a> {
    idx: usize,
    count: &'a UseState<u32>,
}

pub fn Controls<'a>(cx: Scope<'a, ControlsProps<'a>>) -> Element {
    let timer_config = use_shared_state::<Timer>(cx).unwrap();
    let times = use_shared_state::<Vec<ActivityTime>>(cx).unwrap();

    let is_counting = timer_config.read().is_counting;

    cx.render(rsx! {
        Wrapper {
            width: "240px",
            Flexbox {
                justify_content: "space-evenly",
                Button {
                    on_click: move |_event| {
                        if !is_counting {
                            timer_config.write().is_counting = true;
                        }
                    },
                    PlayIcon {},
                },
                Button {
                    on_click: move |_event| {
                        timer_config.write().is_counting = false;
                    },
                    PauseIcon {}
                },
                Button {
                    on_click: move |_event| {
                        timer_config.write().is_counting = false;
                        *cx.props.count.make_mut() = times.read()[cx.props.idx].set_time * 60;
                    },
                    ResetIcon {},
                },
                Button {
                    on_click: move |_event| {
                       //
                    },
                    NextIcon {},
                },
            }
        }

    })
}
