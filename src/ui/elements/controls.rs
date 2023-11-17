#![allow(non_snake_case)]
use crate::ui::components::button::Button;
use crate::ui::components::flexbox::Flexbox;
use crate::ui::components::wrapper::Wrapper;
use crate::ui::icons::menu_icon::MenuIcon;
use crate::ui::icons::next_icon::NextIcon;
use crate::ui::icons::pause_icon::PauseIcon;
use crate::ui::icons::play_icon::PlayIcon;

use crate::contexts::state::{TimerAction, TimerState};
use crate::ui::icons::reset_icon::ResetIcon;

use dioxus::prelude::*;

pub fn Controls<'a>(cx: Scope<'a>) -> Element {
    let timer_state = use_shared_state::<TimerState>(cx).unwrap();

    cx.render(rsx! {
        Wrapper {
            width: "240px",
            Flexbox {
                justify_content: "space-evenly",
                Button {
                    on_click: move |_event| {
                        timer_state.write().reduce(TimerAction::StartTime);
                    },
                    PlayIcon {},
                },
                Button {
                    on_click: move |_event| {
                        timer_state.write().reduce(TimerAction::PauseTime);
                    },
                    PauseIcon {}
                },
                Button {
                    on_click: move |_event| {
                        timer_state.write().reduce(TimerAction::ResetTime);
                    },
                    ResetIcon {},
                },
                Button {
                    on_click: move |_event| {
                        timer_state.write().reduce(TimerAction::NextActivity);
                    },
                    NextIcon {},
                },
                Button {
                    on_click: move |_event| {
                        timer_state.write().reduce(TimerAction::OpenSettings);
                    },
                    MenuIcon {},
                },
            }
        }

    })
}
