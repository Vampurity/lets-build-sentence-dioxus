use dioxus::prelude::*;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;

use crate::dom::{self, listener_slot};
use crate::ui::icons::{CloseIcon, NoDataIcon};

const MODAL_CLIP: &str = "M0.501,0.005 L0.501,0.005 L0.523,0.005 L0.549,0.006 C0.704,0.01,0.796,0.017,0.825,0.027 L0.827,0.028 C0.872,0.045,0.939,0.044,0.978,0.17 C1,0.254,1,0.365,0.99,0.505 L0.988,0.513 C0.979,0.558,0.971,0.598,0.965,0.633 C0.956,0.689,0.979,0.77,0.964,0.865 C0.953,0.928,0.921,0.966,0.869,0.979 C0.821,0.986,0.773,0.992,0.726,0.995 L0.712,0.996 L0.694,0.997 C0.648,1,0.586,1,0.507,1 L0.501,1 L0.464,1 C0.385,1,0.325,0.998,0.283,0.995 C0.234,0.992,0.184,0.987,0.133,0.979 C0.081,0.966,0.05,0.928,0.039,0.865 C0.023,0.77,0.047,0.689,0.037,0.633 C0.031,0.595,0.023,0.552,0.013,0.505 C-0.006,0.365,-0.002,0.254,0.024,0.17 C0.064,0.045,0.13,0.045,0.174,0.028 L0.175,0.028 C0.204,0.017,0.303,0.009,0.474,0.005 L0.501,0.005";

#[component]
pub fn Modal(
    open: bool,
    title: String,
    #[props(default = 520)] width: u32,
    on_close: EventHandler<()>,
    footer: Element,
    children: Element,
) -> Element {
    let mut open_state = use_signal(|| open);
    if open_state() != open {
        open_state.set(open);
    }
    let listeners = use_hook(listener_slot);
    use_effect(move || {
        dom::set_overflow(open_state());
        listeners.borrow_mut().take();
        if !open_state() {
            return;
        }
        let closure = Closure::<dyn FnMut(web_sys::Event)>::new(move |event: web_sys::Event| {
            let Ok(key_event) = event.dyn_into::<web_sys::KeyboardEvent>() else {
                return;
            };
            if key_event.key() == "Escape" {
                key_event.prevent_default();
                on_close.call(());
            }
        });
        if let Some(window) = dom::window() {
            *listeners.borrow_mut() = Some(dom::DomListener::listen(
                dom::event_target(&window),
                "keydown",
                false,
                closure,
            ));
        }
    });

    if !open {
        return rsx! {};
    }

    rsx! {
        div {
            class: "animal-mask-hAWeP",
            onclick: move |_| on_close.call(()),
            div {
                class: "animal-modal-d0zyM animal-modalGame-PfHbW",
                style: "width: {width}px;",
                role: "dialog",
                aria_modal: "true",
                aria_labelledby: "help-title",
                tabindex: "-1",
                onclick: move |event| event.stop_propagation(),
                svg {
                    style: "position: absolute; width: 0; height: 0;",
                    defs {
                        clipPath {
                            id: "animal-modal-clip",
                            clip_path_units: "objectBoundingBox",
                            path { d: MODAL_CLIP }
                        }
                    }
                }
                div { class: "animal-modalClipped-7IuAw animal-gameClipped-WhWVl",
                    div { class: "animal-header-udOSi",
                        div { id: "help-title", class: "animal-title-s6pv3", "{title}" }
                    }
                    div { class: "animal-body--aiyx", {children} }
                    div { class: "animal-footer-9Ib-a", {footer} }
                }
            }
        }
    }
}

#[component]
pub fn ClickTip(open: bool, text: String, class: String, children: Element) -> Element {
    let tooltip_class = if open {
        "animal-tooltip-UY7BU animal-bottom-1EwHs animal-bordered-FDN3c animal-visible-Gdi6E"
    } else {
        "animal-tooltip-UY7BU animal-bottom-1EwHs animal-bordered-FDN3c"
    };
    rsx! {
        div { class: "animal-tooltipWrapper-PL7BH {class}",
            {children}
            div {
                class: "{tooltip_class}",
                role: "tooltip",
                aria_hidden: (!open).to_string(),
                div { class: "animal-content-GvqM2", "{text}" }
            }
        }
    }
}

#[component]
pub fn NoData(
    title: String,
    #[props(default)] description: String,
    #[props(default = dioxus_core::VNode::empty())] extra: Element,
) -> Element {
    rsx! {
        div { class: "animal-no-data-to37x",
            div { class: "animal-artwork-chPRV", aria_hidden: "true", NoDataIcon {} }
            div { class: "animal-title-93lb0", "{title}" }
            if !description.is_empty() {
                div { class: "animal-description-bLIJu", "{description}" }
            }
            div { class: "animal-actions-IoVAn", {extra} }
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ToastKind {
    Success,
    Warning,
}

#[derive(Clone, PartialEq, Eq)]
struct ToastItem {
    id: u32,
    kind: ToastKind,
    message: String,
}

#[derive(Clone, Copy)]
pub struct Toasts(Signal<Vec<ToastItem>>);

impl Toasts {
    pub fn provide() -> Self {
        let items = use_signal(|| Vec::<ToastItem>::new());
        let toasts = Self(items);
        use_context_provider(|| toasts);
        toasts
    }

    pub fn success(self, message: impl Into<String>) {
        self.push(ToastKind::Success, message);
    }

    pub fn warning(self, message: impl Into<String>) {
        self.push(ToastKind::Warning, message);
    }

fn push(self, kind: ToastKind, message: impl Into<String>) {
        let message = message.into();
        let id = {
            let mut items = self.0;
            let mut items = items.write();
            let id = items.last().map(|item| item.id + 1).unwrap_or(1);
            items.push(ToastItem { id, kind, message });
            id
        };
        let mut items = self.0;
        let closure = wasm_bindgen::closure::Closure::once(move || {
            items.write().retain(|item| item.id != id);
        });
        if let Some(window) = dom::window() {
            let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                closure.as_ref().unchecked_ref(),
                4000,
            );
            closure.forget();
        }
    }
}

#[component]
pub fn ToastRegion() -> Element {
    let toasts = use_context::<Toasts>();
    let items = toasts.0.read().clone();
    rsx! {
        div { class: "animal-notificationRoot-RORJk",
            div { class: "animal-position-top-Hkkt-",
                for item in items {
                    ToastCard { key: "{item.id}", id: item.id, kind: item.kind, message: item.message }
                }
            }
        }
    }
}

#[component]
fn ToastCard(id: u32, kind: ToastKind, message: String) -> Element {
    let mut toasts = use_context::<Toasts>();
    let kind_class = match kind {
        ToastKind::Success => "animal-type-success-YK9DN",
        ToastKind::Warning => "animal-type-warning-jCRuT",
    };
    rsx! {
        div { class: "animal-notification-IdTYg {kind_class} animal-placement-top--vluo",
            div { class: "animal-iconWrap-H2lSM", aria_hidden: "true",
                match kind {
                    ToastKind::Success => rsx! { SuccessMark {} },
                    ToastKind::Warning => rsx! { WarningMark {} },
                }
            }
            div { class: "animal-body-yQbOe",
                div { class: "animal-title-4-Xls", "{message}" }
            }
            button {
                class: "animal-close-pi6S6",
                r#type: "button",
                aria_label: "关闭提示",
                onclick: move |_| {
                    toasts.0.write().retain(|item| item.id != id);
                },
                CloseIcon { size: 16 }
            }
        }
    }
}

#[component]
fn SuccessMark() -> Element {
    rsx! {
        svg { view_box: "0 0 24 24", width: "1em", height: "1em",
            path { d: "M5 12.5l4.5 4.5L19 7.5", fill: "none", stroke: "currentColor", stroke_width: "3", stroke_linecap: "round", stroke_linejoin: "round" }
        }
    }
}

#[component]
fn WarningMark() -> Element {
    rsx! {
        svg { view_box: "0 0 24 24", width: "1em", height: "1em",
            path { d: "M12 4l9.5 16.5h-19z", fill: "none", stroke: "currentColor", stroke_width: "2.5", stroke_linejoin: "round" }
            path { d: "M12 10v4M12 16.5v.01", fill: "none", stroke: "currentColor", stroke_width: "2.5", stroke_linecap: "round" }
        }
    }
}
