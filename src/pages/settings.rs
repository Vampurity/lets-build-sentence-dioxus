use dioxus::prelude::*;

use crate::nav::Nav;
use crate::settings::{step_size, BackgroundKind, FONT_MAX, FONT_MIN, FONT_STEP};
use crate::storage::SettingsStore;
use crate::ui::{
    Background, Button, ButtonKind, Card, ClickTip, IconKind, Tag, TagColor, TextInput, Title,
    TitleColor, TitleSize, Toasts,
};

#[component]
pub fn SettingsPage() -> Element {
    let nav = use_context::<Nav>();
    let settings_store = use_context::<SettingsStore>();
    let toasts = use_context::<Toasts>();
    let settings = settings_store.get();
    let mut preview = use_signal(|| "I like apples.".to_string());

    rsx! {
        main { class: "page",
            div { class: "stack",
                div { class: "row",
                    Button { onclick: move |_| nav.close_settings(), "返回" }
                    Title { size: TitleSize::Large, color: TitleColor::AppTeal, "设置" }
                }
                Card {
                    div { class: "stack",
                        Title { size: TitleSize::Small, "背景" }
                        p { class: "muted", "只能选择组件库自带的背景。背景和字体只保存在这台浏览器里。" }
                        div { class: "bg-grid",
                            for kind in BackgroundKind::ALL {
                                BackgroundTile {
                                    key: "{kind:?}",
                                    kind,
                                    selected: settings.background == kind,
                                    on_select: move |next| {
                                        let mut updated = settings_store.get();
                                        updated.background = next;
                                        settings_store.set(updated);
                                    },
                                }
                            }
                        }
                    }
                }
                Card {
                    div { class: "stack",
                        Title { size: TitleSize::Small, "字体大小" }
                        p { class: "muted", "分别调整中文句子、输入的英文，以及点击中文后弹出的答案。" }
                        SizeControl {
                            label: "中文句子".to_string(),
                            value: settings.chinese_size,
                            on_change: move |value| {
                                let mut updated = settings_store.get();
                                updated.chinese_size = value;
                                settings_store.set(updated);
                            },
                        }
                        SizeControl {
                            label: "英文句子".to_string(),
                            value: settings.english_size,
                            on_change: move |value| {
                                let mut updated = settings_store.get();
                                updated.english_size = value;
                                settings_store.set(updated);
                            },
                        }
                        SizeControl {
                            label: "答案提示".to_string(),
                            value: settings.answer_size,
                            on_change: move |value| {
                                let mut updated = settings_store.get();
                                updated.answer_size = value;
                                settings_store.set(updated);
                            },
                        }
                    }
                }
                Card {
                    div { class: "stack",
                        Title { size: TitleSize::Small, "预览" }
                        p { class: "hint", "点击中文句子，查看英文答案" }
                        div { class: "sentence-wrap", PreviewSentence {} }
                        TextInput {
                            aria_label: "英文句子预览".to_string(),
                            placeholder: "在这里写出英文句子".to_string(),
                            class: "english-input".to_string(),
                            value: preview(),
                            on_input: move |value| preview.set(value),
                        }
                    }
                }
                div {
                    Button {
                        onclick: move |_| {
                            settings_store.reset();
                            preview.set("I like apples.".to_string());
                            toasts.success("已恢复默认设置");
                        },
                        "恢复默认"
                    }
                }
            }
        }
    }
}

#[component]
fn PreviewSentence() -> Element {
    let mut open = use_signal(|| false);
    rsx! {
        ClickTip {
            open: open(),
            text: "I like apples.".to_string(),
            class: "answer-tooltip".to_string(),
            Button {
                kind: ButtonKind::Text,
                class: "sentence-trigger".to_string(),
                onclick: move |_| open.set(!open()),
                "我喜欢苹果。"
            }
        }
    }
}

#[component]
fn BackgroundTile(kind: BackgroundKind, selected: bool, on_select: EventHandler<BackgroundKind>) -> Element {
    let class_name = if selected { "bg-pick is-selected" } else { "bg-pick" };
    rsx! {
        div { class: "{class_name}",
            Card {
                hoverable: true,
                aria_label: kind.label().to_string(),
                aria_pressed: Some(selected),
                on_activate: move |_| on_select.call(kind),
                div { class: "stack",
                    div { class: "bg-swatch",
                        Background { kind, class: String::new(), style: "height: 100%;".to_string() }
                    }
                    div { class: "row",
                        span { class: "bg-name", "{kind.label()}" }
                        if selected {
                            Tag { color: TagColor::Green, "使用中" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn SizeControl(label: String, value: u32, on_change: EventHandler<u32>) -> Element {
    let decrease = label.clone();
    let increase = label.clone();
    rsx! {
        div { class: "size-row",
            span { class: "size-label", "{label}" }
            Button {
                aria_label: "减小{decrease}",
                disabled: value <= FONT_MIN,
                icon: IconKind::Minus,
                onclick: move |_| on_change.call(step_size(value, -(FONT_STEP as i32))),
            }
            span { class: "size-value", "{value} px" }
            Button {
                aria_label: "增大{increase}",
                disabled: value >= FONT_MAX,
                icon: IconKind::Plus,
                onclick: move |_| on_change.call(step_size(value, FONT_STEP as i32)),
            }
        }
    }
}
