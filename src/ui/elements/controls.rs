#![allow(non_snake_case)]
use crate::ui::components::button::Button;
use crate::ui::components::flexbox::Flexbox;
use crate::ui::components::wrapper::Wrapper;
use crate::ui::icons::next_icon::NextIcon;
use crate::ui::icons::pause_icon::PauseIcon;
use crate::ui::icons::play_icon::PlayIcon;

use crate::contexts::state::IsCounting;
use crate::ui::icons::reset_icon::ResetIcon;

use dioxus::prelude::*;

#[derive(PartialEq, Props)]
pub struct ControlsProps {
    idx: usize, // TODO: verify usage
}

pub fn Controls(cx: Scope<ControlsProps>) -> Element {
    let is_counting = use_shared_state::<IsCounting>(cx).unwrap();

    let counting_state = is_counting.read().0;

    cx.render(rsx! {
        Wrapper {
            width: "240px",
            Flexbox {
                justify_content: "space-evenly",
                Button {
                    on_click: move |_event| {
                        if !counting_state {
                            is_counting.write().0 = !counting_state;
                        }
                    },
                    PlayIcon {},
                },
                Button {
                    on_click: move |_event| {
                        is_counting.write().0 = false;
                    },
                    PauseIcon {}
                },
                Button {
                    on_click: move |_event| {
                       //
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
