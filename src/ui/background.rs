use dioxus::prelude::*;

use crate::settings::{font_style, AppSettings, BackgroundKind};

const SWEET_CORNER: Asset = asset!("/assets/backgrounds/sweet-corner.svg");
const COFFEE_BREAK: Asset = asset!("/assets/backgrounds/coffee-break.svg");

#[component]
pub fn Background(kind: BackgroundKind, class: String, style: String, children: Element) -> Element {
    let class_name = format!("animal-background-tX6OV {} {class}", kind.css_class());
    let mut inline = style;
    if let Some(url) = scene_url(kind) {
        inline.push_str(&format!("background-image: url('{url}');"));
    }
    rsx! {
        div { class: "{class_name}", style: "{inline}", {children} }
    }
}

pub fn shell_style(settings: AppSettings) -> String {
    font_style(settings)
}

fn scene_url(kind: BackgroundKind) -> Option<String> {
    match kind {
        BackgroundKind::SweetCorner => Some(SWEET_CORNER.to_string()),
        BackgroundKind::CoffeeBreak => Some(COFFEE_BREAK.to_string()),
        _ => None,
    }
}
