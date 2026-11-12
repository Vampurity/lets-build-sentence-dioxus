use dioxus::prelude::*;

use crate::banks::{catalog, practice_set, COLLECTION_ID};
use crate::english::EnglishEdit;
use crate::favorites::clamp_index;
use crate::host::{self, HostBus, PracticeAction, Shortcut, PRACTICE_PAGE_ID};
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
    let shortcut = use_context::<Signal<Shortcut>>()();
    let bus = use_context::<HostBus>();
    let mut index = use_signal(|| 0usize);
    let mut help_open = use_signal(|| false);
    let mut answer_for = use_signal(|| None::<u32>);
    let ids = favorites.ids();
    let practice = practice_set(bank_id, &ids);
    let can_next = practice
        .as_ref()
        .is_some_and(|set| index() + 1 < set.sentences.len());
    let options = bank_options(favorites.count());

    use_effect(move || {
        for action in bus.drain_practice() {
            apply_practice_action(action, bank_id, favorites, index, answer_for);
        }
    });

    use_effect(move || {
        let help = help_open();
        let hidden = nav.show_settings();
        let _ = index();
        if help || hidden {
            return;
        }
        host::focus(ENGLISH_INPUT_ID);
    });

    rsx! {
        main {
            id: "{PRACTICE_PAGE_ID}",
            class: "page",
            "data-can-next": if can_next { "true" } else { "false" },
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
                if let Some(key) = shortcut.answer_key() {
                    li {
                        kbd { "{key}" }
                        " + "
                        kbd { "?" }
                        "{answer_shortcut_help(shortcut)}"
                    }
                }
                if shortcut != Shortcut::None {
                    li { kbd { "回车" } "：下一句" }
                }
                li { "输入框里的空格用来分隔单词" }
                li { "点击中文句子同样可以查看答案" }
                li { "只能输入英文，第一个字母会自动大写" }
            }
        }
    }
}

fn answer_shortcut_help(mode: Shortcut) -> &'static str {
    match mode {
        Shortcut::Meta => "：显示或隐藏英文答案。这个键在右 Shift 左边，正在输入时也可以",
        Shortcut::Ctrl => "：显示或隐藏英文答案。正在输入时也可以",
        Shortcut::None => "",
    }
}

fn answer_hint(mode: Shortcut) -> &'static str {
    match mode {
        Shortcut::Meta => "按 ⌘ + ?，或点击中文句子，查看英文答案",
        Shortcut::Ctrl => "按 Ctrl + ?，或点击中文句子，查看英文答案",
        Shortcut::None => "点击中文句子，查看英文答案",
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
    let shortcut = use_context::<Signal<Shortcut>>()();
    let mut draft = use_signal(String::new);
    let mut pending = use_signal(|| None::<(u32, u32)>);

    use_effect(move || {
        let _ = draft();
        if let Some((start, end)) = pending() {
            pending.set(None);
            host::restore_caret(ENGLISH_INPUT_ID, start, end);
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
        p { class: "hint", "{answer_hint(shortcut)}" }
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

fn apply_practice_action(
    action: PracticeAction,
    bank_id: u32,
    favorites: FavoriteStore,
    mut index: Signal<usize>,
    mut answer_for: Signal<Option<u32>>,
) {
    let ids = favorites.ids();
    let Some(set) = practice_set(bank_id, &ids) else {
        return;
    };
    if set.sentences.is_empty() {
        return;
    }
    match action {
        PracticeAction::ToggleAnswer => {
            let current = index().min(set.sentences.len() - 1);
            let id = set.sentences[current].id;
            if answer_for() == Some(id) {
                answer_for.set(None);
            } else {
                answer_for.set(Some(id));
            }
        }
        PracticeAction::Next => {
            let total = set.sentences.len();
            let current = index();
            if current + 1 < total {
                index.set(current + 1);
            }
        }
    }
}
