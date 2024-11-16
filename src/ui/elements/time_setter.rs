#![allow(non_snake_case)]

use dioxus::prelude::*;
use crate::app::{Activity, ActivityTime};
use crate::ui::components::button::Button;
use crate::ui::components::flexbox::Flexbox;
use crate::ui::components::text::Text;
use crate::ui::components::wrapper::Wrapper;
use crate::ui::icons::minus_icon::MinusIcon;
use crate::ui::icons::plus_icon::PlusIcon;


#[derive(PartialEq, Props, Clone)]
pub struct TimeSetterProps {
    activity_type_label: Activity,
    activity_type: Signal<Activity>,
    break_time: Signal<ActivityTime>,
    session_time: Signal<ActivityTime>,
    starting_time: Signal<ActivityTime>,
    is_counting: bool,
    count: Signal<u32>,
}

pub fn TimeSetter(mut props: TimeSetterProps) -> Element {
    let break_time = props.break_time;
    let session_time = props.session_time;
    let starting_time = props.starting_time;
    let activity_type_label = props.activity_type_label;
    let mut activity_type = props.activity_type;

    let value = match activity_type_label {
        Activity::Break => break_time().set_time,
        Activity::Session => session_time().set_time,
        Activity::StartingIn => starting_time().set_time,
    };

    rsx! {
        Flexbox{
            width: "96px",
            flex_grow: 0,
            Button {
                on_click: move |_event| {
                    match activity_type_label {
                        Activity::Break => {
                             if (2..=60u32).contains(&break_time().set_time) {
                                 let mut break_time_copy = break_time();
                                 break_time_copy.set_time -= 1;
                                 props.break_time.set(break_time_copy);
                            }
                            if !props.is_counting {
                                props.count.set(break_time().set_time * 60);

                                if activity_type() != Activity::Break {
                                    props.activity_type.set(Activity::Break);
                                }
                            }
                        }
                        Activity::Session => {
                            if (2..=60u32).contains(&session_time().set_time) {
                                let mut session_time_copy = session_time();
                                session_time_copy.set_time -= 1;
                                props.session_time.set(session_time_copy);
                            }
                            if !props.is_counting {
                                props.count.set(session_time().set_time * 60);

                                if activity_type() != Activity::Session {
                                    props.activity_type.set(Activity::Session);
                                }
                            }
                        }
                        Activity::StartingIn => {
                            if (2..=60u32).contains(&starting_time().set_time) {
                                let mut starting_time_copy = starting_time();
                                starting_time_copy.set_time -= 1;
                                props.starting_time.set(starting_time_copy);
                            }
                            if !props.is_counting {
                                props.count.set(starting_time().set_time * 60);

                                if activity_type() != Activity::StartingIn {
                                    props.activity_type.set(Activity::StartingIn);
                                }
                            }
                        }
                    }
                },
                Flexbox {
                    MinusIcon {
                        size: "20"
                    }
                }
            },
            Wrapper {
                width: "36px",
                Text {
                    font_size: "20px",
                    text: "{value}",
                },
            },
            Button {
                on_click: move |_event| {
                    match activity_type_label {
                        Activity::Break => {
                            if (0..=59u32).contains(&break_time().set_time) {
                                let mut break_time_copy = break_time();
                                break_time_copy.set_time += 1;
                                props.break_time.set(break_time_copy);
                            }
                            if !props.is_counting {
                                props.count.set(break_time().set_time * 60);

                                if activity_type() == Activity::Session {
                                    props.activity_type.set(Activity::Break);
                                }
                            }
                        }
                        Activity::Session => {
                            if (0..=59u32).contains(&session_time().set_time) {
                                let mut session_time_copy = session_time();
                                session_time_copy.set_time += 1;
                                props.session_time.set(session_time_copy);
                            }
                            if !props.is_counting {
                                props.count.set(session_time().set_time * 60);

                                if activity_type() == Activity::Break {
                                    props.activity_type.set(Activity::Session);
                                }
                            }
                        }
                        Activity::StartingIn => {
                            if (0..=59u32).contains(&starting_time().set_time) {
                                let mut starting_time_copy = starting_time();
                                starting_time_copy.set_time += 1;
                                props.starting_time.set(starting_time_copy);
                            }
                            if !props.is_counting {
                                props.count.set(starting_time().set_time * 60);

                                if activity_type() != Activity::StartingIn {
                                    props.activity_type.set(Activity::StartingIn);
                                }
                            }
                        }
                    }
                },
                Flexbox {
                    PlusIcon {
                        size: "20"
                    }
                }   
            }
        }
    }
}
