#![allow(non_snake_case)]

use dioxus::prelude::*;
use crate::app::{Activity, ActivityTime};
use crate::ui::components::button::Button;
use crate::ui::components::flexbox::Flexbox;
use crate::ui::icons::chevron_left::ChevronLeftIcon;
use crate::ui::icons::chevron_right::ChevronRightIcon;
use crate::ui::icons::next_icon::NextIcon;
use crate::ui::icons::pause_icon::PauseIcon;
use crate::ui::icons::play_icon::PlayIcon;
use crate::ui::icons::reset_icon::ResetIcon;

#[derive(PartialEq, Props, Clone)]
pub struct ControlsProps {
    is_timer_open: Signal<bool>,
    is_controls_open: Signal<bool>,
    is_counting: Signal<bool>,
    is_settings_open: Signal<bool>,
    activity_type: Signal<Activity>,
    count: Signal<u32>,
    break_time: Signal<ActivityTime>,
    session_time: Signal<ActivityTime>,
    starting_time: Signal<ActivityTime>,
}


pub fn Controls(mut props: ControlsProps) -> Element {
    let activity_type = props.activity_type;
    let break_time = props.break_time;
    let session_time = props.session_time;
    let starting_time = props.starting_time;

    rsx! {
        Flexbox {
            justify_content: "space-between",
            height: "40px",
            Flexbox {
                width: "40px",
                Button {
                    height: "40px",
                    on_click: move |_event| {
                        props.is_controls_open.set(false);
                        props.is_timer_open.set(true);
                    },
                    ChevronLeftIcon {},
                },
            }
            Flexbox {
                padding: "0 8px",
                justify_content: "space-evenly",
                Button {
                    height: "40px",
                    on_click: move |_event| {
                        let is_counting = props.is_counting;

                        if is_counting() == false {
                            props.is_counting.set(true);
                        }
                    },
                    PlayIcon {},
                },
                Button {
                    height: "40px",
                    on_click: move |_event| {
                        props.is_counting.set(false);
                    },
                    PauseIcon {}
                },
                Button {
                    height: "40px",
                    on_click: move |_event| {
                        props.is_counting.set(false);
                        match activity_type() {
                            Activity::Session => {
                                props.count.set(session_time().set_time * 60);
                            }
                            Activity::Break => {
                                props.count.set(break_time().set_time * 60);
                            }
                            Activity::StartingIn => {
                                props.count.set(starting_time().set_time * 60);
                            }
                        }
                    },
                    ResetIcon {},
                },
                Button {
                    height: "40px",
                    on_click: move |_event| {
                        match activity_type() {
                            Activity::Session => {
                                props.activity_type.set(Activity::Break);
                                props.count.set(break_time().set_time * 60);
                            }
                            Activity::Break => {
                                props.activity_type.set(Activity::StartingIn);
                                props.count.set(starting_time().set_time * 60);
                            }
                            Activity::StartingIn => {
                                props.activity_type.set(Activity::Session);
                                props.count.set(session_time().set_time * 60);
                            }
                        }

                    },
                    NextIcon {},
                },
            },
            Flexbox {
                width: "40px",
                Button {
                    height: "40px",
                    on_click: move |_event| {
                        props.is_controls_open.set(false);
                        props.is_settings_open.set(true);
                    },
                    ChevronRightIcon {},
                },
            }
        }
    }
}
