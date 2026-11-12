#![allow(non_snake_case)]

mod app;
mod banks;
mod english;
mod favorites;
mod host;
mod nav;
mod pages;
mod settings;
mod storage;
mod ui;

fn main() {
    use dioxus::prelude::*;

    dioxus::LaunchBuilder::new()
        .with_cfg(desktop! {
            dioxus::desktop::Config::new().with_window(
                dioxus::desktop::WindowBuilder::new()
                    .with_title("在线造句练习")
                    .with_inner_size(dioxus::desktop::LogicalSize::new(1360.0, 880.0))
                    .with_min_inner_size(dioxus::desktop::LogicalSize::new(1024.0, 700.0)),
            )
        })
        .launch(app::App);
}
