use crate::contexts::state::{Activity, TimerAction, TimerState};
use crate::ui::components::{button::Button, flexbox::Flexbox, label::Label};
use crate::ui::elements::time_setter::TimeSetter;
use crate::ui::icons::close::CloseIcon;
use crate::ui::icons::exit::ExitIcon;
use dioxus::prelude::*;

pub fn Settings<'a>(cx: Scope<'a>) -> Element {
    let timer_state = use_shared_state::<TimerState>(cx).unwrap();
    let window = dioxus_desktop::use_window(cx);

    cx.render(rsx! {
        Flexbox {
            direction: "column",
            width: "172px",
            Label {
                font_size: "20px",
                text: "Break Length"
            },
            TimeSetter {
                activity_type: Activity::Break
            }
        },
        Flexbox {
            direction: "column",
            width: "172px",
            Label {
                font_size: "20px",
                text: "Session Length"
            },
            TimeSetter {
                activity_type: Activity::Session
            }
        }
        Flexbox {
            width: "120px",
            justify_content: "space-evenly",
            Button {
                on_click: move |_event| {
                    window.close();
                },
                ExitIcon {}
            },
            Button {
                on_click: move |_event| {
                    timer_state.write().reduce(TimerAction::CloseSettings);
                },
                CloseIcon {},
            }
        }
    })
}
