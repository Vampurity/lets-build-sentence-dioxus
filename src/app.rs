use dioxus::prelude::*;

use crate::host::{self, HostBus, Shortcut};
use crate::nav::{initial_behind, parse_page, Nav, Page};
use crate::storage::{favorites_from_storage, settings_from_storage, FavoriteStore, SettingsStore};
use crate::ui::{shell_style, Background, Button, NoData, ToastRegion, Toasts};

const ZCOOL_LATIN: Asset = asset!("/assets/fonts/zcool-kuaile-latin-400-normal.woff2");
const ZCOOL_CJK: Asset = asset!("/assets/fonts/zcool-kuaile-chinese-simplified-400-normal.woff2");

#[component]
pub fn App() -> Element {
    let mut path = use_signal(|| "/".to_string());
    let mut behind = use_signal(|| "/".to_string());
    let mut shortcut = use_signal(|| Shortcut::Ctrl);
    let mut ready = use_signal(|| false);
    let nav = Nav::new(path, behind);
    use_context_provider(|| nav);
    use_context_provider(|| shortcut);
    let settings_store = SettingsStore::provide();
    let favorites = FavoriteStore::provide();
    let _toasts = Toasts::provide();
    let bus = HostBus::provide();
    let started = use_hook(|| std::rc::Rc::new(std::cell::Cell::new(false)));

    use_effect(move || {
        if started.get() {
            return;
        }
        started.set(true);
        spawn(async move {
            let loaded = host::boot().await;
            path.set(loaded.path.clone());
            behind.set(initial_behind(&loaded.path));
            settings_store.replace(settings_from_storage(loaded.settings.as_deref()));
            favorites.replace(favorites_from_storage(loaded.favorites.as_deref()));
            shortcut.set(loaded.shortcut);
            host::run_session(bus, move || ready.set(true)).await;
        });
    });

    use_effect(move || {
        for next in bus.drain_popstate() {
            nav.sync_path(next);
        }
    });

    let settings = settings_store.get();
    let show_settings = nav.show_settings();
    let show_pages = ready();
    let page = parse_page(&nav.page_path());

    rsx! {
        document::Title { "在线造句练习" }
        document::Stylesheet { href: asset!("/assets/animal.css") }
        document::Stylesheet { href: asset!("/assets/app.css") }
        document::Style {
            r#"
            @font-face {{
                font-family: "ZCOOL KuaiLe";
                src: url("{ZCOOL_LATIN}") format("woff2");
                font-weight: 400;
                font-style: normal;
                font-display: swap;
                unicode-range: U+0000-00FF, U+0131, U+0152-0153, U+02BB-02BC, U+02C6, U+02DA, U+02DC, U+0304, U+0308, U+0329, U+2000-206F, U+20AC, U+2122, U+2191, U+2193, U+2212, U+2215, U+FEFF, U+FFFD;
            }}
            @font-face {{
                font-family: "ZCOOL KuaiLe";
                src: url("{ZCOOL_CJK}") format("woff2");
                font-weight: 400;
                font-style: normal;
                font-display: swap;
                unicode-range: U+3000-303F, U+3400-4DBF, U+4E00-9FFF, U+F900-FAFF, U+FF00-FFEF;
            }}
            "#
        }
        Background {
            kind: settings.background,
            class: "animal-cursor animal-cursor--force app-shell".to_string(),
            style: shell_style(settings),
            if show_pages {
                div {
                    id: "page-layer",
                    hidden: show_settings.then_some("true"),
                    inert: show_settings.then_some("true"),
                    match page {
                        Page::Library => rsx! { crate::pages::LibraryPage {} },
                        Page::Practice(bank_id) => rsx! { crate::pages::PracticePage { key: "{bank_id}", bank_id } },
                        Page::Missing => rsx! {
                            main { class: "page",
                                NoData {
                                    title: "这个页面不存在".to_string(),
                                    extra: rsx! { Button { onclick: move |_| nav.go("/"), "返回题库" } },
                                }
                            }
                        },
                    }
                }
                if show_settings {
                    crate::pages::SettingsPage {}
                }
                ToastRegion {}
            }
        }
    }
}
