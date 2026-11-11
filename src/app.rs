use dioxus::prelude::*;
use wasm_bindgen::closure::Closure;

use crate::dom::{self, listener_slot};
use crate::nav::{initial_behind, parse_page, current_pathname, Nav, Page};
use crate::storage::{FavoriteStore, SettingsStore};
use crate::ui::{shell_style, Background, Button, NoData, ToastRegion, Toasts};

const ZCOOL_LATIN: Asset = asset!("/assets/fonts/zcool-kuaile-latin-400-normal.woff2");
const ZCOOL_CJK: Asset = asset!("/assets/fonts/zcool-kuaile-chinese-simplified-400-normal.woff2");

#[component]
pub fn App() -> Element {
    let path = use_signal(current_pathname);
    let behind = use_signal(initial_behind);
    let nav = Nav::new(path, behind);
    use_context_provider(|| nav);
    let _settings = SettingsStore::provide();
    let _favorites = FavoriteStore::provide();
    let _toasts = Toasts::provide();
    let listeners = use_hook(listener_slot);

    use_effect(move || {
        if listeners.borrow().is_some() {
            return;
        }
        let closure = Closure::<dyn FnMut(web_sys::Event)>::new(move |_| {
            nav.sync_from_browser();
        });
        if let Some(window) = dom::window() {
            *listeners.borrow_mut() = Some(dom::DomListener::listen(
                dom::event_target(&window),
                "popstate",
                false,
                closure,
            ));
        }
    });

    let settings = use_context::<SettingsStore>().get();
    let show_settings = nav.show_settings();
    let page = parse_page(&nav.page_path());

    use_effect(move || {
        let hide = nav.show_settings();
        if let Some(layer) = dom::element_by_id("page-layer") {
            if hide {
                let _ = layer.set_attribute("inert", "");
            } else {
                let _ = layer.remove_attribute("inert");
            }
        }
    });

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
            div {
                id: "page-layer",
                hidden: show_settings,
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
