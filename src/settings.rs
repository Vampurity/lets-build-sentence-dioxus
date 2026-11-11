use serde::{Deserialize, Serialize};
use serde_json::Value;

pub const FONT_MIN: u32 = 12;
pub const FONT_MAX: u32 = 72;
pub const FONT_STEP: u32 = 2;
pub const SETTINGS_KEY: &str = "lets-build-sentence.settings";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum BackgroundKind {
    Default,
    Grid,
    Sprinkles,
    SweetCorner,
    CoffeeBreak,
    DotsDarkGreen,
    DotsPink,
    DotsPurple,
    DotsBlue,
    DotsYellow,
    DotsOrange,
    DotsTeal,
    DotsGreen,
    DotsRed,
    DotsLimeGreen,
    DotsYellowGreen,
    DotsBrown,
    DotsWarmPeachPink,
    DotsGray,
}

impl BackgroundKind {
    pub const ALL: [BackgroundKind; 19] = [
        Self::Default,
        Self::Grid,
        Self::Sprinkles,
        Self::SweetCorner,
        Self::CoffeeBreak,
        Self::DotsDarkGreen,
        Self::DotsPink,
        Self::DotsPurple,
        Self::DotsBlue,
        Self::DotsYellow,
        Self::DotsOrange,
        Self::DotsTeal,
        Self::DotsGreen,
        Self::DotsRed,
        Self::DotsLimeGreen,
        Self::DotsYellowGreen,
        Self::DotsBrown,
        Self::DotsWarmPeachPink,
        Self::DotsGray,
    ];

    pub fn label(self) -> &'static str {
        match self {
            Self::Default => "奶油波点",
            Self::Grid => "网格",
            Self::Sprinkles => "彩色糖针",
            Self::SweetCorner => "甜点角落",
            Self::CoffeeBreak => "咖啡时光",
            Self::DotsDarkGreen => "深绿波点",
            Self::DotsPink => "粉色波点",
            Self::DotsPurple => "紫色波点",
            Self::DotsBlue => "蓝色波点",
            Self::DotsYellow => "黄色波点",
            Self::DotsOrange => "橙色波点",
            Self::DotsTeal => "青色波点",
            Self::DotsGreen => "绿色波点",
            Self::DotsRed => "红色波点",
            Self::DotsLimeGreen => "青柠波点",
            Self::DotsYellowGreen => "黄绿波点",
            Self::DotsBrown => "棕色波点",
            Self::DotsWarmPeachPink => "蜜桃波点",
            Self::DotsGray => "灰色波点",
        }
    }

    pub fn css_class(self) -> &'static str {
        match self {
            Self::Default => "animal-bg-default-5auWm",
            Self::Grid => "animal-bg-grid-pQnHh",
            Self::Sprinkles => "animal-bg-sprinkles-IgYnr",
            Self::SweetCorner => "animal-bg-sweet-corner-dilg5",
            Self::CoffeeBreak => "animal-bg-coffee-break-D3-Ac",
            Self::DotsDarkGreen => "animal-bg-dots-dark-green-jt1MF",
            Self::DotsPink => "animal-bg-dots-pink-YD1yu",
            Self::DotsPurple => "animal-bg-dots-purple-SpcFE",
            Self::DotsBlue => "animal-bg-dots-blue-OFDDu",
            Self::DotsYellow => "animal-bg-dots-yellow-IDtej",
            Self::DotsOrange => "animal-bg-dots-orange-GuHLH",
            Self::DotsTeal => "animal-bg-dots-teal-YybV2",
            Self::DotsGreen => "animal-bg-dots-green-LXTsl",
            Self::DotsRed => "animal-bg-dots-red-Vq6Za",
            Self::DotsLimeGreen => "animal-bg-dots-lime-green-gTs-T",
            Self::DotsYellowGreen => "animal-bg-dots-yellow-green-K00B2",
            Self::DotsBrown => "animal-bg-dots-brown-0eRe7",
            Self::DotsWarmPeachPink => "animal-bg-dots-warm-peach-pink-OcXqR",
            Self::DotsGray => "animal-bg-dots-gray-Do1kL",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppSettings {
    pub background: BackgroundKind,
    pub chinese_size: u32,
    pub english_size: u32,
    pub answer_size: u32,
}

pub const DEFAULT_SETTINGS: AppSettings = AppSettings {
    background: BackgroundKind::Default,
    chinese_size: 42,
    english_size: 22,
    answer_size: 12,
};

fn read_size(value: Option<&Value>, fallback: u32) -> u32 {
    let Some(number) = value.and_then(Value::as_f64) else {
        return fallback;
    };
    if !number.is_finite() {
        return fallback;
    }
    let rounded = number.round() as i64;
    rounded.clamp(FONT_MIN as i64, FONT_MAX as i64) as u32
}

pub fn normalize_settings(value: &Value) -> AppSettings {
    let object = value.as_object();
    let background = object
        .and_then(|data| data.get("background"))
        .and_then(|item| serde_json::from_value::<BackgroundKind>(item.clone()).ok())
        .unwrap_or(DEFAULT_SETTINGS.background);
    AppSettings {
        background,
        chinese_size: read_size(
            object.and_then(|data| data.get("chineseSize")),
            DEFAULT_SETTINGS.chinese_size,
        ),
        english_size: read_size(
            object.and_then(|data| data.get("englishSize")),
            DEFAULT_SETTINGS.english_size,
        ),
        answer_size: read_size(
            object.and_then(|data| data.get("answerSize")),
            DEFAULT_SETTINGS.answer_size,
        ),
    }
}

pub fn step_size(value: u32, delta: i32) -> u32 {
    let next = value as i32 + delta;
    next.clamp(FONT_MIN as i32, FONT_MAX as i32) as u32
}

pub fn font_style(settings: AppSettings) -> String {
    format!(
        "--sentence-zh-size: {}px; --sentence-en-size: {}px; --sentence-answer-size: {}px;",
        settings.chinese_size, settings.english_size, settings.answer_size
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keeps_only_known_backgrounds_and_clamps_font_sizes() {
        let parsed = normalize_settings(&serde_json::json!({
            "background": "forest-grove",
            "chineseSize": 400,
            "englishSize": 3,
            "answerSize": 28.4
        }));
        assert_eq!(parsed.background, DEFAULT_SETTINGS.background);
        assert_eq!(parsed.chinese_size, 72);
        assert_eq!(parsed.english_size, 12);
        assert_eq!(parsed.answer_size, 28);

        let kept = normalize_settings(&serde_json::json!({
            "background": "coffee-break",
            "chineseSize": 36,
            "englishSize": 20,
            "answerSize": 18
        }));
        assert_eq!(
            kept,
            AppSettings {
                background: BackgroundKind::CoffeeBreak,
                chinese_size: 36,
                english_size: 20,
                answer_size: 18,
            }
        );
    }

    #[test]
    fn serializes_the_existing_storage_shape() {
        let raw = serde_json::to_value(DEFAULT_SETTINGS).unwrap();
        assert_eq!(raw["background"], "default");
        assert_eq!(raw["chineseSize"], 42);
        assert_eq!(raw["englishSize"], 22);
        assert_eq!(raw["answerSize"], 12);
        assert_eq!(BackgroundKind::ALL.len(), 19);
    }
}
