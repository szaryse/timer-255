#![allow(non_snake_case)]

use dioxus::prelude::*;

use crate::ui::components::button::Button;
use crate::ui::components::flexbox::Flexbox;
use crate::ui::icons::chevron_left::ChevronLeftIcon;
use crate::ui::icons::chevron_right::ChevronRightIcon;
use crate::ui::icons::next_icon::NextIcon;
use crate::ui::icons::pause_icon::PauseIcon;
use crate::ui::icons::play_icon::PlayIcon;
use crate::contexts::state::{TimerAction, TimerState};
use crate::ui::icons::reset_icon::ResetIcon;


pub fn Controls() -> Element {
    // let timer_state = use_shared_state::<TimerState>(cx).unwrap();

    rsx! {
        Flexbox {
            justify_content: "space-between",
            Flexbox {
                width: "40px",
                Button {
                    on_click: move |_event| {
                        // timer_state.write().reduce(TimerAction::GoBackToTimer);
                    },
                    ChevronLeftIcon {},
                },
            }
            Flexbox {
                padding: "0 8px",
                justify_content: "space-evenly",
                Button {
                    on_click: move |_event| {
                        // timer_state.write().reduce(TimerAction::StartTime);
                    },
                    PlayIcon {},
                },
                Button {
                    on_click: move |_event| {
                        // timer_state.write().reduce(TimerAction::PauseTime);
                    },
                    PauseIcon {}
                },
                Button {
                    on_click: move |_event| {
                        // timer_state.write().reduce(TimerAction::ResetTime);
                    },
                    ResetIcon {},
                },
                Button {
                    on_click: move |_event| {
                        // timer_state.write().reduce(TimerAction::NextActivity);
                    },
                    NextIcon {},
                },
            },
            Flexbox {
                width: "40px",
                Button {
                    on_click: move |_event| {
                        // timer_state.write().reduce(TimerAction::GoToSettings);
                    },
                    ChevronRightIcon {},
                },
            }
        }
    }
}
