use dioxus::prelude::*;
use serde_json::Value;

use crate::banks::FAVORITES_KEY;
use crate::favorites::sanitize;
use crate::settings::{normalize_settings, AppSettings, DEFAULT_SETTINGS, SETTINGS_KEY};

fn local_storage() -> Option<web_sys::Storage> {
    web_sys::window()?.local_storage().ok()?
}

fn storage_get(key: &str) -> Option<String> {
    local_storage()?.get_item(key).ok()?
}

fn storage_set(key: &str, value: &str) {
    if let Some(storage) = local_storage() {
        let _ = storage.set_item(key, value);
    }
}

pub fn load_settings() -> AppSettings {
    storage_get(SETTINGS_KEY)
        .and_then(|raw| serde_json::from_str::<Value>(&raw).ok())
        .map(|value| normalize_settings(&value))
        .unwrap_or(DEFAULT_SETTINGS)
}

pub fn save_settings(settings: &AppSettings) {
    if let Ok(raw) = serde_json::to_string(settings) {
        storage_set(SETTINGS_KEY, &raw);
    }
}

pub fn load_favorites() -> Vec<u32> {
    let Some(raw) = storage_get(FAVORITES_KEY) else {
        return Vec::new();
    };
    let Ok(ids) = serde_json::from_str::<Vec<u32>>(&raw) else {
        return Vec::new();
    };
    sanitize(&ids)
}

pub fn save_favorites(ids: &[u32]) {
    if let Ok(raw) = serde_json::to_string(ids) {
        storage_set(FAVORITES_KEY, &raw);
    }
}

#[derive(Clone, Copy)]
pub struct SettingsStore(pub Signal<AppSettings>);

impl SettingsStore {
    pub fn provide() -> Self {
        let settings = use_signal(load_settings);
        let store = Self(settings);
        use_context_provider(|| store);
        store
    }

    pub fn get(self) -> AppSettings {
        (self.0)()
    }

    pub fn set(mut self, settings: AppSettings) {
        self.0.set(settings);
        save_settings(&settings);
    }

    pub fn reset(self) {
        self.set(DEFAULT_SETTINGS);
    }
}

#[derive(Clone, Copy)]
pub struct FavoriteStore(pub Signal<Vec<u32>>);

impl FavoriteStore {
    pub fn provide() -> Self {
        let ids = use_signal(load_favorites);
        let store = Self(ids);
        use_context_provider(|| store);
        store
    }

    pub fn ids(self) -> Vec<u32> {
        self.0.read().clone()
    }

    pub fn count(self) -> usize {
        self.0.read().len()
    }

    pub fn toggle(mut self, sentence_id: u32) -> bool {
        let (next, on) = crate::favorites::toggle(&self.ids(), sentence_id);
        self.0.set(next.clone());
        save_favorites(&next);
        on
    }
}
