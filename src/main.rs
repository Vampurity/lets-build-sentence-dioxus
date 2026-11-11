#![allow(non_snake_case)]

mod app;
mod banks;
mod dom;
mod english;
mod favorites;
mod nav;
mod pages;
mod settings;
mod storage;
mod ui;

fn main() {
    dioxus::launch(app::App);
}
