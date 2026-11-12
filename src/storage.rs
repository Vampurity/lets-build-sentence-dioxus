use dioxus::prelude::*;
use serde_json::Value;

use crate::banks::FAVORITES_KEY;
use crate::favorites::sanitize;
use crate::host;
use crate::settings::{normalize_settings, AppSettings, DEFAULT_SETTINGS, SETTINGS_KEY};

pub fn settings_from_storage(raw: Option<&str>) -> AppSettings {
    raw.and_then(|raw| serde_json::from_str::<Value>(raw).ok())
        .map(|value| normalize_settings(&value))
        .unwrap_or(DEFAULT_SETTINGS)
}

pub fn save_settings(settings: &AppSettings) {
    if let Ok(raw) = serde_json::to_string(settings) {
        host::storage_set(SETTINGS_KEY, &raw);
    }
}

pub fn favorites_from_storage(raw: Option<&str>) -> Vec<u32> {
    let Some(raw) = raw else {
        return Vec::new();
    };
    let Ok(ids) = serde_json::from_str::<Vec<u32>>(raw) else {
        return Vec::new();
    };
    sanitize(&ids)
}

pub fn save_favorites(ids: &[u32]) {
    if let Ok(raw) = serde_json::to_string(ids) {
        host::storage_set(FAVORITES_KEY, &raw);
    }
}

#[derive(Clone, Copy)]
pub struct SettingsStore(pub Signal<AppSettings>);

impl SettingsStore {
    pub fn provide() -> Self {
        let settings = use_signal(|| DEFAULT_SETTINGS);
        let store = Self(settings);
        use_context_provider(|| store);
        store
    }

    pub fn get(self) -> AppSettings {
        (self.0)()
    }

    pub fn replace(mut self, settings: AppSettings) {
        self.0.set(settings);
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
        let ids = use_signal(Vec::new);
        let store = Self(ids);
        use_context_provider(|| store);
        store
    }

    pub fn replace(mut self, ids: Vec<u32>) {
        self.0.set(ids);
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
