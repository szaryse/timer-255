#![allow(non_snake_case)]
use crate::ui::components::button::Button;
use crate::ui::components::flexbox::Flexbox;
use crate::ui::components::label::Label;
use crate::ui::components::wrapper::Wrapper;

use crate::contexts::state::{Activity, TimerAction, TimerState};
use crate::ui::icons::minus_icon::MinusIcon;
use crate::ui::icons::plus_icon::PlusIcon;

use dioxus::prelude::*;

#[derive(PartialEq, Props)]
pub struct TimeSetterProps {
    activity_type: Activity,
}

pub fn TimeSetter(cx: Scope<TimeSetterProps>) -> Element {
    let timer_state = use_shared_state::<TimerState>(cx).unwrap();

    let value = timer_state
        .read()
        .select_time_value(cx.props.activity_type.clone());

    cx.render(rsx! {
        Flexbox{
            width: "96px",
            flex_grow: 0,
            Button {
                on_click: move |_event| {
                    match cx.props.activity_type {
                        Activity::Break => timer_state.write().reduce(TimerAction::DecreaseBreakTime),
                        Activity::Session => timer_state.write().reduce(TimerAction::DecreaseSessionTime)
                    }
                },
                Flexbox {
                    MinusIcon {
                        size: "24"
                    }
                }
            },
            Wrapper {
                width: "36px",
                Label {
                    font_size: "20px",
                    text: "{value}",
                },
            },
            Button {
                on_click: move |_event| {
                    match cx.props.activity_type {
                        Activity::Break => timer_state.write().reduce(TimerAction::IncreaseBreakTime),
                        Activity::Session => timer_state.write().reduce(TimerAction::IncreaseSessionTime)
                    }
                },
                Flexbox {
                    PlusIcon {
                        size: "24"
                    }
                }   
            }
        }
    })
}
