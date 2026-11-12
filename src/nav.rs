use dioxus::prelude::*;

use crate::banks::{catalog, COLLECTION_ID};
use crate::host;

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

    pub fn sync_path(mut self, next: String) {
        if next != "/settings" {
            self.behind.set(next.clone());
        }
        self.path.set(next);
    }

    fn visit(mut self, to: &str, replace: bool) {
        if self.path.read().as_str() == to {
            return;
        }
        host::history_move(to, replace);
        self.path.set(to.to_string());
    }
}

pub fn initial_behind(path: &str) -> String {
    if path == "/settings" {
        "/".to_string()
    } else {
        path.to_string()
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
