use std::cell::Cell;
use std::rc::Rc;

use dioxus::prelude::*;

use crate::host::{self, AnchorRect, HostBus, BANK_SELECT_ID, BANK_SELECT_TRIGGER_ID};

#[derive(Clone, PartialEq, Eq)]
pub struct SelectOption {
    pub key: String,
    pub label: String,
}

const MENU_FALLBACK: &str = "position:absolute;top:calc(100% + 6px);left:0;min-width:220px;";

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
    let bus = use_context::<HostBus>();
    let seen_outside = use_hook(|| Rc::new(Cell::new(u32::MAX)));
    let option_list = options.clone();

    use_effect(move || {
        let tick = bus.outside_tick();
        if seen_outside.get() == u32::MAX {
            seen_outside.set(tick);
            return;
        }
        if tick == seen_outside.get() {
            return;
        }
        seen_outside.set(tick);
        if open() {
            open.set(false);
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
            id: "{BANK_SELECT_ID}",
            class: "animal-wrapper-pbOTq",
            onkeydown: move |event| {
                handle_key(
                    event,
                    open,
                    active,
                    menu_style,
                    &option_list,
                    keydown_value.clone(),
                    on_change,
                );
            },
            div {
                id: "{BANK_SELECT_TRIGGER_ID}",
                class: "{trigger_class}",
                role: "combobox",
                tabindex: "0",
                aria_label: "{aria_label}",
                aria_expanded: open().to_string(),
                aria_haspopup: "listbox",
                onclick: move |_| {
                    let click_value = click_value.clone();
                    async move {
                        let next = !open();
                        if next {
                            menu_style.set(position_style().await);
                            active.set(Some(click_value));
                        }
                        open.set(next);
                    }
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
    mut menu_style: Signal<String>,
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
            active.set(Some(value));
            dioxus::core::spawn_forever(async move {
                menu_style.set(position_style().await);
                open.set(true);
            });
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

async fn position_style() -> String {
    let Some(rect) = host::anchor_rect(BANK_SELECT_TRIGGER_ID).await else {
        return MENU_FALLBACK.to_string();
    };
    menu_style_for(rect)
}

fn menu_style_for(rect: AnchorRect) -> String {
    let mut style = String::from("position:absolute;min-width:220px;");
    if rect.right + 220.0 > rect.width {
        style.push_str("right:100%;margin-right:6px;left:auto;");
    } else {
        style.push_str("left:100%;margin-left:6px;right:auto;");
    }
    let below = rect.height - rect.bottom;
    let above = rect.top;
    if below < 220.0 && above > below {
        style.push_str("top:auto;bottom:100%;margin-bottom:6px;");
    } else if below < 220.0 || rect.top < 220.0 {
        style.push_str("top:100%;margin-top:6px;bottom:auto;");
    } else {
        style.push_str("top:50%;bottom:auto;transform:translateY(-50%);");
    }
    style
}
