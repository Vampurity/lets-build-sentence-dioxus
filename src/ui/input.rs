use dioxus::prelude::*;

use crate::english::{edit_from_insert, edit_from_raw, EnglishEdit};
use crate::host::{EnglishEvent, HostBus};

pub use crate::host::ENGLISH_INPUT_ID;

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

#[component]
pub fn EnglishInput(value: String, on_change: EventHandler<EnglishEdit>) -> Element {
    let bus = use_context::<HostBus>();

    use_effect(move || {
        for event in bus.drain_english() {
            match event {
                EnglishEvent::Before {
                    value,
                    data,
                    start,
                    end,
                } => {
                    let edit = edit_from_insert(&value, start as usize, end as usize, &data);
                    if edit.value != value {
                        on_change.call(edit);
                    }
                }
                EnglishEvent::Input { value, start, end } => {
                    on_change.call(edit_from_raw(&value, start as usize, end as usize));
                }
            }
        }
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
            }
        }
    }
}
