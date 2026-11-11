use dioxus::prelude::*;
use dioxus::web::WebEventExt;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;

use crate::dom::{self, listener_slot};
use crate::english::{edit_from_insert, edit_from_raw, is_allowed_english_insert, EnglishEdit};

#[component]
pub fn TextInput(
    value: String,
    aria_label: String,
    placeholder: String,
    #[props(default)] class: String,
    on_input: EventHandler<String>,
) -> Element {
    rsx! {
        span { class: "animal-wrapper-IRrhj animal-wrapper-large-RrYM4 animal-wrapper-no-shadow-WP9Gk {class}",
            input {
                class: "animal-input-12WUn",
                r#type: "text",
                aria_label,
                placeholder,
                autocomplete: "off",
                value,
                oninput: move |event| on_input.call(event.value()),
            }
        }
    }
}

pub const ENGLISH_INPUT_ID: &str = "english-sentence";

#[component]
pub fn EnglishInput(value: String, on_change: EventHandler<EnglishEdit>) -> Element {
    let listeners = use_hook(listener_slot);
    use_effect(move || {
        if listeners.borrow().is_some() {
            return;
        }
        let Some(input) = dom::input_by_id(ENGLISH_INPUT_ID) else {
            return;
        };
        let closure = Closure::<dyn FnMut(web_sys::Event)>::new(move |event: web_sys::Event| {
            handle_before_input(event, on_change);
        });
        let target = input.unchecked_into::<web_sys::EventTarget>();
        *listeners.borrow_mut() = Some(dom::DomListener::listen(target, "beforeinput", false, closure));
    });

    rsx! {
        span { class: "animal-wrapper-IRrhj animal-wrapper-large-RrYM4 animal-wrapper-no-shadow-WP9Gk english-input",
            input {
                id: ENGLISH_INPUT_ID,
                class: "animal-input-12WUn",
                r#type: "text",
                aria_label: "英文句子",
                placeholder: "在这里写出英文句子",
                autocomplete: "off",
                value,
                oninput: move |event| {
                    let (start, end) = selection_from_form(&event);
                    on_change.call(edit_from_raw(&event.value(), start, end));
                },
            }
        }
    }
}

fn handle_before_input(event: web_sys::Event, on_change: EventHandler<EnglishEdit>) {
    let Ok(input_event) = event.dyn_into::<web_sys::InputEvent>() else {
        return;
    };
    let Some(data) = input_event.data() else {
        return;
    };
    if is_allowed_english_insert(&data) {
        return;
    }
    input_event.prevent_default();
    let Some(target) = input_event.target() else {
        return;
    };
    let Ok(input) = target.dyn_into::<web_sys::HtmlInputElement>() else {
        return;
    };
    let value = input.value();
    let (start, end) = selection_of(&input, value.encode_utf16().count());
    let edit = edit_from_insert(&value, start, end, &data);
    if edit.value != value {
        on_change.call(edit);
    }
}

fn selection_from_form(event: &FormEvent) -> (usize, usize) {
    let raw = event.data().as_web_event();
    let Some(target) = raw.target() else {
        let len = event.value().encode_utf16().count();
        return (len, len);
    };
    let Ok(input) = target.dyn_into::<web_sys::HtmlInputElement>() else {
        return (0, 0);
    };
    selection_of(&input, input.value().encode_utf16().count())
}

fn selection_of(input: &web_sys::HtmlInputElement, len: usize) -> (usize, usize) {
    let start = input
        .selection_start()
        .ok()
        .flatten()
        .unwrap_or(len as u32) as usize;
    let end = input
        .selection_end()
        .ok()
        .flatten()
        .unwrap_or(start as u32) as usize;
    (start.min(len), end.min(len))
}
