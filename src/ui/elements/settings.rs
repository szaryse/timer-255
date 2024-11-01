use dioxus::prelude::*;
use dioxus::desktop::use_window;

use crate::contexts::state::{Activity, TimerAction, TimerState};
use crate::ui::components::{button::Button, flexbox::Flexbox, text::Text};
use crate::ui::elements::time_setter::TimeSetter;
use crate::ui::icons::chevron_left::ChevronLeftIcon;
use crate::ui::icons::exit::ExitIcon;

pub fn Settings() -> Element {
    // let timer_state = use_shared_state::<TimerState>(cx).unwrap();
    let window = use_window();

    rsx! {
        Flexbox {
            width: "100%",
            justify_content: "space-between",
            Flexbox {
                width: "40px",
                Button {
                    on_click: move |_event| {
                        // timer_state.write().reduce(TimerAction::GoBackToControls);
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
                    Text {
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
                    Text {
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
    }
}
