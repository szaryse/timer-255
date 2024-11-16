// #![windows_subsystem = "windows"]
// INFO: uncomment in the release build

use dioxus::prelude::*;
use dioxus::desktop::{
    tao::dpi::{LogicalSize, PhysicalPosition},
    Config, WindowBuilder,
};
use crate::app::App;
use crate::ui::global_styles::global_styles;

pub mod ui;
pub mod app;

fn main() {
    let window = WindowBuilder::new()
        .with_transparent(true)
        .with_inner_size(LogicalSize::new(320, 0))
        // INFO: set to false in the release build
        .with_always_on_top(true)
        .with_title("Timer 255 0.2")
        .with_resizable(true)
        // INFO: position only for development
        .with_position(PhysicalPosition::new(2870, 60));

    LaunchBuilder::desktop()
        .with_cfg(Config::new()
            .with_window(window)
            .with_custom_head(global_styles())
        )
        .launch(App);
}
