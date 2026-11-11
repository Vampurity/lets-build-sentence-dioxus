use dioxus::prelude::*;

use crate::banks::{catalog, COLLECTION_ID};
use crate::nav::Nav;
use crate::storage::FavoriteStore;
use crate::ui::{
    Button, ButtonKind, ButtonSize, Card, Tag, TagColor, Title, TitleColor, TitleSize,
    Toasts,
};

#[component]
pub fn LibraryPage() -> Element {
    let nav = use_context::<Nav>();
    let favorites = use_context::<FavoriteStore>();
    let toasts = use_context::<Toasts>();
    let mut selected = use_signal(|| None::<u32>);
    let favorite_count = favorites.count();
    let mut cards = vec![BankCard {
        id: COLLECTION_ID,
        name: "我的收藏".to_string(),
        count: favorite_count,
        collection: true,
    }];
    for bank in catalog().banks() {
        cards.push(BankCard {
            id: bank.id,
            name: bank.name.clone(),
            count: bank.sentences.len(),
            collection: false,
        });
    }

    rsx! {
        main { class: "page",
            div { class: "stack",
                div { class: "row",
                    Title { size: TitleSize::Large, color: TitleColor::AppTeal, "在线造句练习" }
                    div { class: "grow" }
                    Button { onclick: move |_| nav.open_settings(), "设置" }
                }
                div { class: "bank-grid",
                    for card in cards {
                        {
                            let id = card.id;
                            rsx! {
                                BankTile {
                                    key: "{id}",
                                    card,
                                    selected: selected() == Some(id),
                                    on_select: move |next| selected.set(Some(next)),
                                }
                            }
                        }
                    }
                }
                Button {
                    kind: ButtonKind::Primary,
                    size: ButtonSize::Large,
                    onclick: move |_| {
                        if let Some(id) = selected() {
                            nav.go(&format!("/practice/{id}"));
                        } else {
                            toasts.warning("请先选择题库");
                        }
                    },
                    "开始练习"
                }
            }
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
struct BankCard {
    id: u32,
    name: String,
    count: usize,
    collection: bool,
}

#[component]
fn BankTile(card: BankCard, selected: bool, on_select: EventHandler<u32>) -> Element {
    let class_name = if selected {
        "bank-pick is-selected"
    } else {
        "bank-pick"
    };
    let id = card.id;
    rsx! {
        div { class: "{class_name}",
            Card {
                hoverable: true,
                aria_label: "{card.name}",
                aria_pressed: Some(selected),
                on_activate: move |_| on_select.call(id),
                div { class: "stack",
                    p { class: "bank-name", "{card.name}" }
                    div { class: "row",
                        if card.collection {
                            Tag { color: TagColor::Pink, "收藏" }
                        } else {
                            Tag { color: TagColor::Teal, "内置" }
                        }
                        span { class: "muted", "{card.count} 句" }
                        if selected {
                            Tag { color: TagColor::Green, "已选中" }
                        }
                    }
                }
            }
        }
    }
}
