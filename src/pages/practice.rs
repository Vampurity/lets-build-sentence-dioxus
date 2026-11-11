use dioxus::prelude::*;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;

use crate::banks::{catalog, practice_set, COLLECTION_ID};
use crate::dom::{self, listener_slot};
use crate::english::EnglishEdit;
use crate::favorites::clamp_index;
use crate::nav::Nav;
use crate::storage::FavoriteStore;
use crate::ui::EnglishInput;
use crate::ui::{
    Button, ButtonKind, ChevronRightIcon, ClickTip, IconKind, Modal, NoData, Select, SelectOption,
    ENGLISH_INPUT_ID,
};

#[component]
pub fn PracticePage(bank_id: u32) -> Element {
    let nav = use_context::<Nav>();
    let favorites = use_context::<FavoriteStore>();
    let mut index = use_signal(|| 0usize);
    let mut help_open = use_signal(|| false);
    let mut answer_for = use_signal(|| None::<u32>);
    let listeners = use_hook(listener_slot);
    let ids = favorites.ids();
    let practice = practice_set(bank_id, &ids);
    let options = bank_options(favorites.count());

    use_effect(move || {
        if listeners.borrow().is_some() {
            return;
        }
        let closure = Closure::<dyn FnMut(web_sys::Event)>::new(move |event: web_sys::Event| {
            on_window_key(event, bank_id, favorites, index, help_open, answer_for);
        });
        if let Some(window) = dom::window() {
            *listeners.borrow_mut() = Some(dom::DomListener::listen(
                dom::event_target(&window),
                "keydown",
                true,
                closure,
            ));
        }
    });

    use_effect(move || {
        let help = help_open();
        let hidden = nav.show_settings();
        let _ = index();
        if help || hidden {
            return;
        }
        if let Some(input) = dom::input_by_id(ENGLISH_INPUT_ID) {
            let _ = input.unchecked_ref::<web_sys::HtmlElement>().focus();
        }
    });

    rsx! {
        main { id: "practice-page", class: "page",
            div { class: "stack",
                div { class: "row",
                    Button { onclick: move |_| nav.go("/"), "返回题库" }
                    Button { onclick: move |_| nav.open_settings(), "设置" }
                    Button {
                        onclick: move |_| help_open.set(true),
                        "帮助"
                    }
                    div { class: "grow" }
                    div { class: "select-wrap",
                        Select {
                            aria_label: "题库",
                            value: bank_id.to_string(),
                            options,
                            on_change: move |key: String| {
                                if let Ok(id) = key.parse::<u32>() {
                                    if id != bank_id {
                                        nav.go(&format!("/practice/{id}"));
                                    }
                                }
                            },
                        }
                    }
                }
                match practice {
                    None => rsx! {
                        NoData {
                            title: "题库不存在".to_string(),
                            extra: rsx! { Button { onclick: move |_| nav.go("/"), "返回题库" } },
                        }
                    },
                    Some(set) if set.sentences.is_empty() => rsx! {
                        NoData {
                            title: if set.is_collection { "还没有收藏的句子".to_string() } else { "这个题库里没有句子".to_string() },
                            description: if set.is_collection { "做题时点「收藏」，句子会放进这里。".to_string() } else { String::new() },
                            extra: rsx! { Button { onclick: move |_| nav.go("/"), "返回题库" } },
                        }
                    },
                    Some(set) => {
                        let total = set.sentences.len();
                        let current = index().min(total - 1);
                        let sentence = set.sentences[current].clone();
                        let answer_open = answer_for() == Some(sentence.id);
                        rsx! {
                            SentenceView {
                                key: "{sentence.id}",
                                index: current,
                                total,
                                sentence_id: sentence.id,
                                number: sentence.number,
                                chinese: sentence.chinese,
                                english: sentence.english,
                                favorite: sentence.favorite,
                                is_collection: set.is_collection,
                                answer_open,
                                on_answer: move |id: u32| {
                                    if answer_for() == Some(id) {
                                        answer_for.set(None);
                                    } else {
                                        answer_for.set(Some(id));
                                    }
                                },
                                on_index: move |next: usize| index.set(next),
                            }
                        }
                    }
                }
            }
        }
        Modal {
            open: help_open(),
            title: "操作说明".to_string(),
            width: 560,
            on_close: move |_| help_open.set(false),
            footer: rsx! {
                Button {
                    kind: ButtonKind::Primary,
                    onclick: move |_| help_open.set(false),
                    "知道了"
                }
            },
            ul { class: "help-list",
                li {
                    kbd { "⌘" }
                    " + "
                    kbd { "?" }
                    "：显示或隐藏英文答案。这个键在右 Shift 左边，正在输入时也可以"
                }
                li { kbd { "回车" } "：下一句" }
                li { "输入框里的空格用来分隔单词" }
                li { "点击中文句子同样可以查看答案" }
                li { "只能输入英文，第一个字母会自动大写" }
            }
        }
    }
}

#[component]
fn SentenceView(
    index: usize,
    total: usize,
    sentence_id: u32,
    number: u32,
    chinese: String,
    english: String,
    favorite: bool,
    is_collection: bool,
    answer_open: bool,
    on_answer: EventHandler<u32>,
    on_index: EventHandler<usize>,
) -> Element {
    let favorites = use_context::<FavoriteStore>();
    let mut draft = use_signal(String::new);
    let mut pending = use_signal(|| None::<(u32, u32)>);

    use_effect(move || {
        let _ = draft();
        if let Some((start, end)) = pending() {
            pending.set(None);
            if let Some(input) = dom::input_by_id(ENGLISH_INPUT_ID) {
                let focused = dom::document()
                    .and_then(|document| document.active_element())
                    .is_some_and(|element| input.is_same_node(Some(element.unchecked_ref())));
                if focused {
                    let _ = input.set_selection_range(start, end);
                }
            }
        }
    });

    rsx! {
        div { class: "practice-bar",
            Button {
                disabled: index == 0,
                icon: IconKind::ChevronLeft,
                onclick: move |_| {
                    if index > 0 {
                        on_index.call(index - 1);
                    }
                },
                "上一句"
            }
            div { class: "progress", "{number} / {total}" }
            Button {
                disabled: index + 1 >= total,
                onclick: move |_| {
                    if index + 1 < total {
                        on_index.call(index + 1);
                    }
                },
                "下一句"
                ChevronRightIcon {}
            }
        }
        p { class: "hint", "按 ⌘ + ?，或点击中文句子，查看英文答案" }
        div { class: "sentence-wrap",
            ClickTip { open: answer_open, text: english.clone(), class: "answer-tooltip".to_string(),
                Button {
                    kind: ButtonKind::Text,
                    class: "sentence-trigger".to_string(),
                    onclick: move |_| on_answer.call(sentence_id),
                    "{chinese}"
                }
            }
        }
        div { class: "practice-actions",
            Button {
                kind: if favorite { ButtonKind::Primary } else { ButtonKind::Default },
                aria_label: if favorite { "移出我的收藏".to_string() } else { "放入我的收藏".to_string() },
                aria_pressed: Some(favorite),
                icon: IconKind::Star,
                onclick: move |_| {
                    let on = favorites.toggle(sentence_id);
                    if is_collection && !on {
                        on_index.call(clamp_index(index, favorites.count()));
                    }
                },
                if favorite { "已收藏" } else { "收藏" }
            }
        }
        EnglishInput {
            value: draft(),
            on_change: move |edit: EnglishEdit| {
                let changed = edit.value != draft();
                draft.set(edit.value);
                if changed {
                    pending.set(Some((edit.caret_start, edit.caret_end)));
                }
            },
        }
    }
}

fn bank_options(favorite_count: usize) -> Vec<SelectOption> {
    let _ = favorite_count;
    let mut options = vec![SelectOption {
        key: COLLECTION_ID.to_string(),
        label: "我的收藏".to_string(),
    }];
    for bank in catalog().banks() {
        options.push(SelectOption {
            key: bank.id.to_string(),
            label: bank.name.clone(),
        });
    }
    options
}

fn on_window_key(
    event: web_sys::Event,
    bank_id: u32,
    favorites: FavoriteStore,
    mut index: Signal<usize>,
    help_open: Signal<bool>,
    mut answer_for: Signal<Option<u32>>,
) {
    let Ok(key_event) = event.dyn_into::<web_sys::KeyboardEvent>() else {
        return;
    };
    if help_open() || key_event.is_composing() {
        return;
    }
    let Some(page) = dom::element_by_id("practice-page") else {
        return;
    };
    if page.closest("[inert]").ok().flatten().is_some() {
        return;
    }
    let command_answer = key_event.code() == "Slash"
        && key_event.meta_key()
        && !key_event.alt_key()
        && !key_event.ctrl_key();
    if command_answer {
        key_event.prevent_default();
        key_event.stop_propagation();
        if key_event.repeat() {
            return;
        }
        let ids = favorites.ids();
        let Some(set) = practice_set(bank_id, &ids) else {
            return;
        };
        if set.sentences.is_empty() {
            return;
        }
        let current = index().min(set.sentences.len() - 1);
        let id = set.sentences[current].id;
        if answer_for() == Some(id) {
            answer_for.set(None);
        } else {
            answer_for.set(Some(id));
        }
        return;
    }
    if key_event.key() != "Enter"
        || key_event.meta_key()
        || key_event.ctrl_key()
        || key_event.alt_key()
        || key_event.shift_key()
        || key_event.repeat()
    {
        return;
    }
    let Some(target) = key_event
        .target()
        .and_then(|target| target.dyn_into::<web_sys::Element>().ok())
    else {
        return;
    };
    let on_blank = dom::body()
        .map(|body| body == target.clone().unchecked_into())
        .unwrap_or(false)
        || dom::document()
            .and_then(|document| document.document_element())
            .map(|root| root == target)
            .unwrap_or(false);
    let in_page = page.contains(Some(target.unchecked_ref()));
    if !in_page && !on_blank {
        return;
    }
    if target
        .closest("button, a, select, [role=\"listbox\"], [role=\"option\"], .select-wrap, [role=\"dialog\"]")
        .ok()
        .flatten()
        .is_some()
    {
        return;
    }
    let ids = favorites.ids();
    let Some(set) = practice_set(bank_id, &ids) else {
        return;
    };
    let total = set.sentences.len();
    let current = index();
    if current + 1 >= total {
        return;
    }
    key_event.prevent_default();
    key_event.stop_propagation();
    index.set(current + 1);
}

