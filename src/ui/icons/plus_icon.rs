use dioxus::prelude::*;

#[derive(PartialEq, Props)]
pub struct PlusIconProps<'a> {
    #[props(default = "40")]
    size: &'a str,
}

pub fn PlusIcon<'a>(cx: Scope<'a, PlusIconProps<'a>>) -> Element {
    let contents = r"
    M453-280h60v-166h167v-60H513v-174h-60v174H280v60h173v166Zm27.266 
    200q-82.734 0-155.5-31.5t-127.266-86q-54.5-54.5-86-127.341Q80-397.681 
    80-480.5q0-82.819 31.5-155.659Q143-709 197.5-763t127.341-85.5Q397.681-880 
    480.5-880q82.819 0 155.659 31.5Q709-817 763-763t85.5 127Q880-563 
    880-480.266q0 82.734-31.5 155.5T763-197.684q-54 54.316-127 86Q563-80 
    480.266-80Zm.234-60Q622-140 721-239.5t99-241Q820-622 721.188-721 622.375-820 
    480-820q-141 0-240.5 98.812Q140-622.375 140-480q0 141 99.5 240.5t241 
    99.5Zm-.5-340Z";

    cx.render(rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            height: cx.props.size,
            width: cx.props.size,
            view_box: "0 -960 960 960",
            path {
                d: "{contents}",
                fill: "#008000",
            }
        }
    })
}
