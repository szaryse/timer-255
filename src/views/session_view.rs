#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus::desktop::use_window;
use crate::app::{Activity, ActivityTime, Views};
use crate::ui::components::{button::Button, flexbox::Flexbox, text::Text};
use crate::ui::components::wrapper::Wrapper;
use crate::ui::elements::time_setter::TimeSetter;
use crate::ui::icons::chevron_left::ChevronLeftIcon;
use crate::ui::icons::chevron_right::ChevronRightIcon;
use crate::ui::icons::exit::ExitIcon;
use crate::ui::icons::minus_icon::MinusIcon;
use crate::ui::icons::plus_icon::PlusIcon;

#[derive(PartialEq, Props, Clone)]
pub struct SessionViewProps {
    activity_type: Signal<Activity>,
    selected_view: Signal<Views>,
    starting_time: Signal<ActivityTime>,
    session_number: Signal<i32>,
    is_counting: bool,
    count: Signal<u32>,
}

pub fn SessionView(mut props: SessionViewProps) -> Element {
    let session_number = props.session_number;
    let starting_time = props.starting_time;
    let set_time = starting_time().set_time;
    let mut activity_type = props.activity_type;


    rsx! {
        Flexbox {
            width: "100%",
            height: "40px",
            justify_content: "space-between",
            Flexbox {
                width: "40px",
                Button {
                    width: "40px",
                    on_click: move |_event| {
                        props.selected_view.set(Views::Controls);
                    },
                    ChevronLeftIcon {},
                },
            },
            Flexbox {
                direction: "column",
                justify_content: "space-between",
                Flexbox {
                    height: "20px",
                    justify_content: "space-between",
                    Text {
                        font_size: "18px",
                        line_height: "20px",
                        text: "Session number"
                    },
                    Flexbox {
                        width: "96px",
                        flex_grow: 0,
                        Button {
                            on_click: move |_event| {
                                if (2..=16i32).contains(&session_number()) {
                                    props.session_number -= 1;
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
                                text: "{session_number()}",
                            },
                        },
                        Button {
                            on_click: move |_event| {
                                if (0..=15i32).contains(&session_number()) {
                                    props.session_number += 1;
                                }
                            },
                            Flexbox {
                                PlusIcon {
                                    size: "20"
                                }
                            }
                        }
                    }
                },
                Flexbox {
                    height: "20px",
                    justify_content: "space-between",
                    Text {
                        font_size: "18px",
                        line_height: "20px",
                        text: "Starting in"
                    },
                    Flexbox {
                        width: "96px",
                        flex_grow: 0,
                        Button {
                            on_click: move |_event| {
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
                                text: "{set_time}",
                            },
                        },
                        Button {
                            on_click: move |_event| {
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
                            },
                            Flexbox {
                                PlusIcon {
                                    size: "20"
                                }
                            }
                        }
                    }
                },
            },
            Flexbox {
                width: "40px",
                Button {
                    height: "40px",
                    on_click: move |_event| {
                        props.selected_view.set(Views::Settings);
                    },
                    ChevronRightIcon {},
                },
            }
        }
    }
}
