use crate::contexts::state::{Activity, TimerAction, TimerState};
use crate::ui::components::{button::Button, flexbox::Flexbox, label::Label};
use crate::ui::elements::time_setter::TimeSetter;
use crate::ui::icons::chevron_left::ChevronLeftIcon;
use crate::ui::icons::exit::ExitIcon;
use dioxus::prelude::*;

pub fn Settings<'a>(cx: Scope<'a>) -> Element {
    let timer_state = use_shared_state::<TimerState>(cx).unwrap();
    let window = dioxus_desktop::use_window(cx);

    cx.render(rsx! {
        Flexbox {
            width: "100%",
            justify_content: "space-between",
            Flexbox {
                width: "40px",
                Button {
                    on_click: move |_event| {
                        timer_state.write().reduce(TimerAction::GoBackToControls);
                    },
                    ChevronLeftIcon {},
                },
            },
            Flexbox {
                direction: "column",
                justify_content: "space-between",
                Flexbox {
                    height: "26px",
                    justify_content: "space-between",
                    Label {
                        font_size: "20px",
                        text: "Session Length"
                    },
                    TimeSetter {
                        activity_type: Activity::Session
                    }
                },
                Flexbox {
                    height: "26px",
                    justify_content: "space-between",
                    Label {
                        font_size: "20px",
                        text: "Break Length"
                    },
                    TimeSetter {
                        activity_type: Activity::Break
                    }
                },

            },
            Flexbox {
                width: "40px",
                Button {
                    on_click: move |_event| {
                        window.close();
                    },
                    ExitIcon {}
                },
            }
        }
    })
}
