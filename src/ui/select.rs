use dioxus::prelude::*;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;

use crate::dom::{self, listener_slot};

#[derive(Clone, PartialEq, Eq)]
pub struct SelectOption {
    pub key: String,
    pub label: String,
}

#[component]
pub fn Select(
    aria_label: String,
    value: String,
    options: Vec<SelectOption>,
    on_change: EventHandler<String>,
) -> Element {
    let mut open = use_signal(|| false);
    let mut active = use_signal(|| None::<String>);
    let mut menu_style = use_signal(String::new);
    let listeners = use_hook(listener_slot);
    let option_list = options.clone();

    use_effect(move || {
        listeners.borrow_mut().take();
        if !open() {
            return;
        }
        let closure = Closure::<dyn FnMut(web_sys::Event)>::new(move |event: web_sys::Event| {
            let Some(target) = event.target() else {
                return;
            };
            let Ok(node) = target.dyn_into::<web_sys::Node>() else {
                return;
            };
            let inside = dom::element_by_id("bank-select")
                .map(|root| root.contains(Some(&node)))
                .unwrap_or(false);
            if !inside {
                open.set(false);
            }
        });
        if let Some(document) = dom::document() {
            let target = document.unchecked_into::<web_sys::EventTarget>();
            *listeners.borrow_mut() =
                Some(dom::DomListener::listen(target, "mousedown", false, closure));
        }
    });

    let label = options
        .iter()
        .find(|option| option.key == value)
        .map(|option| option.label.clone())
        .unwrap_or_default();
    let keydown_value = value.clone();
    let click_value = value.clone();
    let row_value = value;
    let trigger_class = if open() {
        "animal-trigger-2-Ahh animal-open-867xo"
    } else {
        "animal-trigger-2-Ahh"
    };
    let menu = menu_style.read().clone();
    let choices = if open() { options.clone() } else { Vec::new() };

    rsx! {
        div {
            id: "bank-select",
            class: "animal-wrapper-pbOTq",
            onkeydown: move |event| {
                handle_key(event, open, active, &option_list, keydown_value.clone(), on_change);
            },
            div {
                id: "bank-select-trigger",
                class: "{trigger_class}",
                role: "combobox",
                tabindex: "0",
                aria_label: "{aria_label}",
                aria_expanded: open().to_string(),
                aria_haspopup: "listbox",
                onclick: move |_| {
                    let next = !open();
                    if next {
                        menu_style.set(menu_position());
                        active.set(Some(click_value.clone()));
                    }
                    open.set(next);
                },
                span { class: "animal-value-623RB", "{label}" }
                span { class: "animal-arrow-xNbxA", aria_hidden: "true",
                    svg {
                        width: "12",
                        height: "12",
                        view_box: "0 0 12 12",
                        fill: "none",
                        path {
                            d: "M3 4.5L6 7.5L9 4.5",
                            stroke: "currentColor",
                            stroke_width: "1.5",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                        }
                    }
                }
            }
            if open() {
                div {
                    class: "animal-dropdown-0l6jF",
                    style: "{menu}",
                    role: "listbox",
                    aria_label: "{aria_label}",
                    for option in choices {
                        OptionRow {
                            key: "{option.key}",
                            option_key: option.key.clone(),
                            label: option.label.clone(),
                            selected: option.key == row_value,
                            on_pick: move |key: String| {
                                on_change.call(key);
                                open.set(false);
                            },
                            on_hover: move |key: String| active.set(Some(key)),
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn OptionRow(
    option_key: String,
    label: String,
    selected: bool,
    on_pick: EventHandler<String>,
    on_hover: EventHandler<String>,
) -> Element {
    let class_name = if selected {
        "animal-option-GEkLI animal-active-jtSpt"
    } else {
        "animal-option-GEkLI"
    };
    let hover_key = option_key.clone();
    let pick_key = option_key;
    rsx! {
        div {
            class: "{class_name}",
            role: "option",
            aria_selected: selected.to_string(),
            onmouseenter: move |_| on_hover.call(hover_key.clone()),
            onclick: move |_| on_pick.call(pick_key.clone()),
            span { class: "animal-optionDot-FQtyw", aria_hidden: "true" }
            "{label}"
            if selected {
                div { class: "animal-pillBar-O9cTb" }
            }
        }
    }
}

fn handle_key(
    event: KeyboardEvent,
    mut open: Signal<bool>,
    mut active: Signal<Option<String>>,
    options: &[SelectOption],
    value: String,
    on_change: EventHandler<String>,
) {
    let key = event.key();
    if !open() {
        let opens = matches!(key, Key::Enter | Key::ArrowDown | Key::ArrowUp)
            || matches!(key, Key::Character(ref text) if text == " ");
        if opens {
            event.prevent_default();
            open.set(true);
            active.set(Some(value));
        }
        return;
    }
    match key {
        Key::Escape => {
            event.prevent_default();
            open.set(false);
        }
        Key::ArrowDown => {
            event.prevent_default();
            move_active(active, options, 1);
        }
        Key::ArrowUp => {
            event.prevent_default();
            move_active(active, options, -1);
        }
        Key::Enter => {
            event.prevent_default();
            if let Some(key) = active().or(Some(value)) {
                if options.iter().any(|option| option.key == key) {
                    on_change.call(key);
                    open.set(false);
                }
            }
        }
        Key::Character(text) if text == " " => {
            event.prevent_default();
            if let Some(key) = active() {
                on_change.call(key);
                open.set(false);
            }
        }
        _ => {}
    }
}

fn move_active(mut active: Signal<Option<String>>, options: &[SelectOption], delta: isize) {
    if options.is_empty() {
        return;
    }
    let current = active()
        .and_then(|key| options.iter().position(|option| option.key == key))
        .unwrap_or(0);
    let len = options.len() as isize;
    let next = (current as isize + delta).rem_euclid(len) as usize;
    active.set(Some(options[next].key.clone()));
}

fn menu_position() -> String {
    let Some(trigger) = dom::element_by_id("bank-select-trigger") else {
        return "position:absolute;top:calc(100% + 6px);left:0;min-width:220px;".to_string();
    };
    let rect = trigger.get_bounding_client_rect();
    let Some(window) = dom::window() else {
        return "position:absolute;top:calc(100% + 6px);left:0;min-width:220px;".to_string();
    };
    let width = window
        .inner_width()
        .ok()
        .and_then(|value| value.as_f64())
        .unwrap_or(1200.0);
    let height = window
        .inner_height()
        .ok()
        .and_then(|value| value.as_f64())
        .unwrap_or(800.0);
    let mut style = String::from("position:absolute;min-width:220px;");
    if rect.right() + 220.0 > width {
        style.push_str("right:100%;margin-right:6px;left:auto;");
    } else {
        style.push_str("left:100%;margin-left:6px;right:auto;");
    }
    let below = height - rect.bottom();
    let above = rect.top();
    if below < 220.0 && above > below {
        style.push_str("top:auto;bottom:100%;margin-bottom:6px;");
    } else if below < 220.0 || rect.top() < 220.0 {
        style.push_str("top:100%;margin-top:6px;bottom:auto;");
    } else {
        style.push_str("top:50%;bottom:auto;transform:translateY(-50%);");
    }
    style
}
