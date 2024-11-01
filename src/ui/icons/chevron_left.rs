use dioxus::prelude::*;

pub fn ChevronLeftIcon(cx: Scope) -> Element {
    let contents = r"m432-480 156 156q11 11 11 28t-11 28q-11 11-28 
    11t-28-11L348-452q-6-6-8.5-13t-2.5-15q0-8 2.5-15t8.5-13l184-184q11-11 
    28-11t28 11q11 11 11 28t-11 28L432-480Z";

    cx.render(rsx! {
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
    })
}
