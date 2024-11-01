use dioxus::prelude::*;

pub fn CloseIcon() -> Element {
    let contents = r"M480-424 284-228q-11 11-28 
    11t-28-11q-11-11-11-28t11-28l196-196-196-196q-11-11-11-28t11-28q11-11 
    28-11t28 11l196 196 196-196q11-11 28-11t28 11q11 11 11 28t-11 
    28L536-480l196 196q11 11 11 28t-11 28q-11 11-28 11t-28-11L480-424Z";

    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            height: "40",
            width: "40",
            view_box: "0 -960 960 960",
            path {
                d: "{contents}",
                fill: "#008000",
            }
        }
    }
}
