use crate::contexts::state::{Activity, TimerAction, TimerState};
use crate::ui::components::{button::Button, flexbox::Flexbox, label::Label};
use crate::ui::elements::time_setter::TimeSetter;
use crate::ui::icons::close::CloseIcon;
use dioxus::prelude::*;

pub fn Settings(cx: Scope) -> Element {
    let timer_state = use_shared_state::<TimerState>(cx).unwrap();

    cx.render(rsx! {
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
            Button {
                on_click: move |_event| {
                    timer_state.write().reduce(TimerAction::CloseSettings);
                },
                CloseIcon {},
            }
        }
    })
}
