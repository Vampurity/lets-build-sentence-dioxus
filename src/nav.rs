use dioxus::prelude::*;
use wasm_bindgen::JsValue;

use crate::banks::{catalog, COLLECTION_ID};

#[derive(Clone, Copy)]
pub struct Nav {
    path: Signal<String>,
    behind: Signal<String>,
}

impl Nav {
    pub fn new(path: Signal<String>, behind: Signal<String>) -> Self {
        Self { path, behind }
    }

    pub fn show_settings(self) -> bool {
        self.path.read().as_str() == "/settings"
    }

    pub fn page_path(self) -> String {
        if self.show_settings() {
            self.behind.read().clone()
        } else {
            self.path.read().clone()
        }
    }

    pub fn go(self, to: &str) {
        self.visit(to, false);
    }

    pub fn open_settings(mut self) {
        if !self.show_settings() {
            self.behind.set(self.path.read().clone());
        }
        self.visit("/settings", false);
    }

    pub fn close_settings(self) {
        let back = self.behind.read().clone();
        let back = if back.is_empty() || back == "/settings" {
            "/".to_string()
        } else {
            back
        };
        self.visit(&back, true);
    }

    pub fn sync_from_browser(mut self) {
        let next = current_pathname();
        if next != "/settings" {
            self.behind.set(next.clone());
        }
        self.path.set(next);
    }

    fn visit(mut self, to: &str, replace: bool) {
        if current_pathname() != to {
            if let Some(window) = web_sys::window() {
                if let Ok(history) = window.history() {
                    let result = if replace {
                        history.replace_state_with_url(&JsValue::NULL, "", Some(to))
                    } else {
                        history.push_state_with_url(&JsValue::NULL, "", Some(to))
                    };
                    let _ = result;
                }
            }
        }
        self.path.set(to.to_string());
    }
}

pub fn current_pathname() -> String {
    web_sys::window()
        .and_then(|window| window.location().pathname().ok())
        .filter(|path| !path.is_empty())
        .unwrap_or_else(|| "/".to_string())
}

pub fn initial_behind() -> String {
    let path = current_pathname();
    if path == "/settings" {
        "/".to_string()
    } else {
        path
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Page {
    Library,
    Practice(u32),
    Missing,
}

pub fn parse_page(path: &str) -> Page {
    let path = path.split('?').next().unwrap_or(path);
    let path = path.trim_end_matches('/');
    if path.is_empty() || path == "/" {
        return Page::Library;
    }
    if let Some(rest) = path.strip_prefix("/practice/") {
        if let Ok(id) = rest.parse::<u32>() {
            if id == COLLECTION_ID || catalog().bank(id).is_some() {
                return Page::Practice(id);
            }
        }
    }
    Page::Missing
}
