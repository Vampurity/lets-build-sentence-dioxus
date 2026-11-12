use dioxus::prelude::*;
use serde::Deserialize;

use crate::banks::FAVORITES_KEY;
use crate::english::allowed_extra_literal;
use crate::settings::SETTINGS_KEY;

pub const PRACTICE_PAGE_ID: &str = "practice-page";
pub const ENGLISH_INPUT_ID: &str = "english-sentence";
pub const BANK_SELECT_ID: &str = "bank-select";
pub const BANK_SELECT_TRIGGER_ID: &str = "bank-select-trigger";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Shortcut {
    Meta,
    Ctrl,
    None,
}

impl Shortcut {
    pub fn answer_key(self) -> Option<&'static str> {
        match self {
            Self::Meta => Some("⌘"),
            Self::Ctrl => Some("Ctrl"),
            Self::None => None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Boot {
    pub path: String,
    pub settings: Option<String>,
    pub favorites: Option<String>,
    pub shortcut: Shortcut,
}

impl Default for Boot {
    fn default() -> Self {
        Self {
            path: "/".to_string(),
            settings: None,
            favorites: None,
            shortcut: Shortcut::Ctrl,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PracticeAction {
    ToggleAnswer,
    Next,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EnglishEvent {
    Before {
        value: String,
        data: String,
        start: u32,
        end: u32,
    },
    Input {
        value: String,
        start: u32,
        end: u32,
    },
}

#[derive(Clone, Copy)]
pub struct HostBus {
    popstate: Signal<Vec<String>>,
    practice: Signal<Vec<PracticeAction>>,
    escape: Signal<u32>,
    english: Signal<Vec<EnglishEvent>>,
    outside_select: Signal<u32>,
}

impl HostBus {
    pub fn provide() -> Self {
        let bus = Self {
            popstate: use_signal(Vec::new),
            practice: use_signal(Vec::new),
            escape: use_signal(|| 0u32),
            english: use_signal(Vec::new),
            outside_select: use_signal(|| 0u32),
        };
        use_context_provider(|| bus);
        bus
    }

    pub fn drain_popstate(self) -> Vec<String> {
        drain_vec(self.popstate)
    }

    pub fn drain_practice(self) -> Vec<PracticeAction> {
        drain_vec(self.practice)
    }

    pub fn drain_english(self) -> Vec<EnglishEvent> {
        drain_vec(self.english)
    }

    pub fn escape_tick(self) -> u32 {
        (self.escape)()
    }

    pub fn outside_tick(self) -> u32 {
        (self.outside_select)()
    }

    fn push(self, event: HostEvent) {
        match event {
            HostEvent::Ready => {}
            HostEvent::Popstate { path } => {
                let mut queue = self.popstate;
                queue.write().push(path);
            }
            HostEvent::ToggleAnswer => {
                let mut queue = self.practice;
                queue.write().push(PracticeAction::ToggleAnswer);
            }
            HostEvent::Next => {
                let mut queue = self.practice;
                queue.write().push(PracticeAction::Next);
            }
            HostEvent::Escape => {
                let mut tick = self.escape;
                let next = tick() + 1;
                tick.set(next);
            }
            HostEvent::BeforeInput {
                value,
                data,
                start,
                end,
            } => {
                let mut queue = self.english;
                queue.write().push(EnglishEvent::Before {
                    value,
                    data,
                    start,
                    end,
                });
            }
            HostEvent::Input { value, start, end } => {
                let mut queue = self.english;
                queue.write().push(EnglishEvent::Input { value, start, end });
            }
            HostEvent::OutsideSelect => {
                let mut tick = self.outside_select;
                let next = tick() + 1;
                tick.set(next);
            }
        }
    }
}

fn drain_vec<T: Clone + 'static>(queue: Signal<Vec<T>>) -> Vec<T> {
    let pending = queue.read().clone();
    if pending.is_empty() {
        return pending;
    }
    let mut queue = queue;
    queue.write().clear();
    pending
}

#[derive(Deserialize)]
struct BootJson {
    path: String,
    settings: Option<String>,
    favorites: Option<String>,
    modifier: String,
}

#[derive(Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
enum HostEvent {
    Ready,
    Popstate {
        path: String,
    },
    ToggleAnswer,
    Next,
    Escape,
    BeforeInput {
        value: String,
        data: String,
        start: u32,
        end: u32,
    },
    Input {
        value: String,
        start: u32,
        end: u32,
    },
    OutsideSelect,
}

#[derive(Clone, Copy, Debug, Deserialize)]
pub struct AnchorRect {
    pub top: f64,
    pub right: f64,
    pub bottom: f64,
    pub width: f64,
    pub height: f64,
}

pub async fn boot() -> Boot {
    let Ok(value) = document::eval(&boot_script()).await else {
        return Boot::default();
    };
    let Ok(parsed) = serde_json::from_value::<BootJson>(value) else {
        return Boot::default();
    };
    Boot {
        path: if parsed.path.is_empty() {
            "/".to_string()
        } else {
            parsed.path
        },
        settings: parsed.settings,
        favorites: parsed.favorites,
        shortcut: match parsed.modifier.as_str() {
            "meta" => Shortcut::Meta,
            "none" => Shortcut::None,
            _ => Shortcut::Ctrl,
        },
    }
}

pub async fn run_session(bus: HostBus, mut on_ready: impl FnMut()) {
    let mut eval = document::eval(&session_script());
    let mut announced = false;
    loop {
        match eval.recv::<HostEvent>().await {
            Ok(HostEvent::Ready) => {
                if !announced {
                    announced = true;
                    on_ready();
                }
            }
            Ok(event) => bus.push(event),
            Err(document::EvalError::Serialization(_)) => {}
            Err(_) => {
                if !announced {
                    on_ready();
                }
                return;
            }
        }
    }
}

pub fn history_move(to: &str, replace: bool) {
    let Some(to) = js_string(to) else {
        return;
    };
    let method = if replace { "replaceState" } else { "pushState" };
    dioxus::core::spawn_forever(async move {
        let _ = document::eval(&format!("history.{method}(null, \"\", {to});")).await;
    });
}

pub fn storage_set(key: &str, value: &str) {
    let Some(key) = js_string(key) else {
        return;
    };
    let Some(value) = js_string(value) else {
        return;
    };
    dioxus::core::spawn_forever(async move {
        let _ = document::eval(&format!("localStorage.setItem({key}, {value});")).await;
    });
}

pub fn focus(id: &str) {
    let Some(id) = js_string(id) else {
        return;
    };
    dioxus::core::spawn_forever(async move {
        let _ = document::eval(&format!("document.getElementById({id})?.focus();")).await;
    });
}

pub fn restore_caret(id: &str, start: u32, end: u32) {
    let Some(id) = js_string(id) else {
        return;
    };
    dioxus::core::spawn_forever(async move {
        let _ = document::eval(&format!(
            r#"const input = document.getElementById({id});
               if (input && document.activeElement === input) {{
                   input.setSelectionRange({start}, {end});
               }}"#
        ))
        .await;
    });
}

pub async fn anchor_rect(id: &str) -> Option<AnchorRect> {
    let id = js_string(id)?;
    let value = document::eval(&format!(
        r#"const el = document.getElementById({id});
           if (!el) return null;
           const rect = el.getBoundingClientRect();
           return {{ top: rect.top, right: rect.right, bottom: rect.bottom, width: window.innerWidth, height: window.innerHeight }};"#
    ))
    .await
    .ok()?;
    serde_json::from_value(value).ok()
}

pub fn set_overflow_hidden(hidden: bool) {
    let Some(value) = js_string(if hidden { "hidden" } else { "" }) else {
        return;
    };
    dioxus::core::spawn_forever(async move {
        let _ = document::eval(&format!(
            "if (document.body) document.body.style.overflow = {value};"
        ))
        .await;
    });
}

pub async fn sleep_ms(ms: u32) {
    let _ = document::eval(&format!(
        "await new Promise((resolve) => setTimeout(resolve, {ms}));"
    ))
    .await;
}

fn js_string(value: &str) -> Option<String> {
    serde_json::to_string(value).ok()
}

fn boot_script() -> String {
    let settings = serde_json::to_string(SETTINGS_KEY).unwrap_or_else(|_| "\"\"".to_string());
    let favorites = serde_json::to_string(FAVORITES_KEY).unwrap_or_else(|_| "\"\"".to_string());
    r#"
        const phone = window.matchMedia("(pointer: coarse)").matches
            && !window.matchMedia("(hover: hover)").matches;
        const apple = /Mac|iPhone|iPad/i.test(navigator.platform || "")
            || /Mac OS X/.test(navigator.userAgent || "");
        const path = location.pathname && location.pathname.length ? location.pathname : "/";
        return {
            path,
            settings: localStorage.getItem(__SETTINGS_KEY__),
            favorites: localStorage.getItem(__FAVORITES_KEY__),
            modifier: phone ? "none" : apple ? "meta" : "ctrl",
        };
    "#
    .replace("__SETTINGS_KEY__", &settings)
    .replace("__FAVORITES_KEY__", &favorites)
}

fn session_script() -> String {
    let extra = allowed_extra_literal();
    let input_id = serde_json::to_string(ENGLISH_INPUT_ID).unwrap_or_else(|_| "\"\"".to_string());
    let select_id = serde_json::to_string(BANK_SELECT_ID).unwrap_or_else(|_| "\"\"".to_string());
    let practice_id =
        serde_json::to_string(PRACTICE_PAGE_ID).unwrap_or_else(|_| "\"\"".to_string());
    r#"
        try {
            if (!window.__sentenceHostInstalled) {
                window.__sentenceHostInstalled = true;
                const extra = __EXTRA__;
                let suppressEnglishInput = false;
                const allowedChar = (ch) => {
                    if (ch >= "0" && ch <= "9") return true;
                    if (ch >= "A" && ch <= "Z") return true;
                    if (ch >= "a" && ch <= "z") return true;
                    return extra.includes(ch);
                };
                const allowedInsert = (data) => {
                    const normalized = data.replace(/\r\n/g, " ").replace(/[\n\r]/g, " ");
                    for (const ch of normalized) {
                        if (!allowedChar(ch)) return false;
                    }
                    return true;
                };
                window.addEventListener("popstate", () => {
                    const path = location.pathname && location.pathname.length ? location.pathname : "/";
                    dioxus.send({ kind: "popstate", path });
                });
                window.addEventListener("keydown", (event) => {
                    if (event.isComposing) return;
                    if (document.querySelector("[role='dialog']")) {
                        if (event.key === "Escape") {
                            event.preventDefault();
                            dioxus.send({ kind: "escape" });
                        }
                        return;
                    }
                    const page = document.getElementById(__PRACTICE_ID__);
                    if (!page || page.closest("[inert]")) return;
                    const answer = event.code === "Slash"
                        && (event.metaKey || event.ctrlKey)
                        && !event.altKey
                        && !(event.metaKey && event.ctrlKey);
                    if (answer) {
                        event.preventDefault();
                        event.stopPropagation();
                        if (!event.repeat) dioxus.send({ kind: "toggle-answer" });
                        return;
                    }
                    if (event.key !== "Enter" || event.metaKey || event.ctrlKey || event.altKey || event.shiftKey || event.repeat) return;
                    const target = event.target;
                    if (!target) return;
                    const onBlank = target === document.body || target === document.documentElement;
                    if (!page.contains(target) && !onBlank) return;
                    if (target.closest && target.closest("button, a, select, [role='listbox'], [role='option'], .select-wrap, [role='dialog']")) return;
                    if (page.getAttribute("data-can-next") !== "true") return;
                    event.preventDefault();
                    event.stopPropagation();
                    dioxus.send({ kind: "next" });
                }, true);
                document.addEventListener("beforeinput", (event) => {
                    const input = event.target;
                    if (!input || input.id !== __INPUT_ID__ || event.data == null) return;
                    if (allowedInsert(String(event.data))) return;
                    event.preventDefault();
                    suppressEnglishInput = true;
                    dioxus.send({
                        kind: "before-input",
                        value: input.value ?? "",
                        data: String(event.data),
                        start: input.selectionStart ?? 0,
                        end: input.selectionEnd ?? 0,
                    });
                    queueMicrotask(() => {
                        suppressEnglishInput = false;
                    });
                }, true);
                document.addEventListener("input", (event) => {
                    if (suppressEnglishInput) return;
                    const input = event.target;
                    if (!input || input.id !== __INPUT_ID__) return;
                    const value = input.value ?? "";
                    dioxus.send({
                        kind: "input",
                        value,
                        start: input.selectionStart ?? value.length,
                        end: input.selectionEnd ?? value.length,
                    });
                });
                document.addEventListener("mousedown", (event) => {
                    const root = document.getElementById(__SELECT_ID__);
                    if (!root || root.contains(event.target)) return;
                    dioxus.send({ kind: "outside-select" });
                });
            }
        } catch (error) {}
        dioxus.send({ kind: "ready" });
        while (true) {
            await dioxus.recv();
        }
    "#
    .replace("__EXTRA__", &extra)
    .replace("__INPUT_ID__", &input_id)
    .replace("__SELECT_ID__", &select_id)
    .replace("__PRACTICE_ID__", &practice_id)
}
