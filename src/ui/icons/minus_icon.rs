use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct MinusIconProps {
    #[props(default = "40".to_string())]
    size: String,
}

pub fn MinusIcon(props: MinusIconProps) -> Element {
    let contents = r"
    M280-453h400v-60H280v60ZM480-80q-82 0-155-31.5t-127.5-86Q143-252 
    111.5-325T80-480q0-83 31.5-156t86-127Q252-817 325-848.5T480-880q83 0 
    156 31.5T763-763q54 54 85.5 127T880-480q0 82-31.5 155T763-197.5q-54 
    54.5-127 86T480-80Zm0-60q142 0 241-99.5T820-480q0-142-99-241t-241-99q-141 
    0-240.5 99T140-480q0 141 99.5 240.5T480-140Zm0-340Z";

    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            height: props.size.clone(),
            width: props.size,
            view_box: "0 -960 960 960",
            path {
                d: "{contents}",
                fill: "#008000",
            }
        }
    }
}
