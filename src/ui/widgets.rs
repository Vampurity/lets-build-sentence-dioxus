use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum IconKind {
    #[default]
    None,
    Star,
    ChevronLeft,
    Plus,
    Minus,
}

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum ButtonKind {
    #[default]
    Default,
    Primary,
    Text,
}

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum ButtonSize {
    #[default]
    Middle,
    Large,
}

#[component]
pub fn Button(
    #[props(default)] kind: ButtonKind,
    #[props(default)] size: ButtonSize,
    #[props(default)] disabled: bool,
    #[props(default)] aria_label: String,
    #[props(default)] aria_pressed: Option<bool>,
    #[props(default)] class: String,
    #[props(default)] icon: IconKind,
    #[props(default)] onclick: EventHandler<MouseEvent>,
    #[props(default = dioxus_core::VNode::empty())] children: Element,
) -> Element {
    let kind_class = match kind {
        ButtonKind::Default => "animal-btn-default-L7yE6",
        ButtonKind::Primary => "animal-btn-primary-9tMG9",
        ButtonKind::Text => "animal-btn-text-Tope4",
    };
    let size_class = match size {
        ButtonSize::Middle => "animal-btn-middle-zEXqv",
        ButtonSize::Large => "animal-btn-large-k9IvA",
    };
    let class_name = format!("animal-btn-QoKe1 {kind_class} {size_class} {class}");
    let pressed = aria_pressed.map(|value| value.to_string());
    rsx! {
        button {
            class: "{class_name}",
            r#type: "button",
            disabled,
            aria_label: if aria_label.is_empty() { None } else { Some(aria_label) },
            aria_pressed: pressed,
            onclick: move |event| onclick.call(event),
            if icon != IconKind::None {
                span { class: "animal-btn-icon-P5CS9", aria_hidden: "true",
                    match icon {
                        IconKind::None => rsx! {},
                        IconKind::Star => rsx! { crate::ui::icons::StarIcon {} },
                        IconKind::ChevronLeft => rsx! { crate::ui::icons::ChevronLeftIcon {} },
                        IconKind::Plus => rsx! { crate::ui::icons::PlusIcon {} },
                        IconKind::Minus => rsx! { crate::ui::icons::MinusIcon {} },
                    }
                }
            }
            {children}
        }
    }
}

#[component]
pub fn Card(
    #[props(default)] hoverable: bool,
    #[props(default)] class: String,
    #[props(default)] aria_label: String,
    #[props(default)] aria_pressed: Option<bool>,
    #[props(default)] on_activate: EventHandler<()>,
    children: Element,
) -> Element {
    let class_name = if hoverable {
        format!("animal-card-DJ515 animal-card-hoverable-RF-YV {class}")
    } else {
        format!("animal-card-DJ515 {class}")
    };
    let pressed = aria_pressed.map(|value| value.to_string());
    rsx! {
        div {
            class: "{class_name}",
            role: if hoverable { "button" } else { "group" },
            tabindex: if hoverable { "0" } else { "-1" },
            aria_label: if aria_label.is_empty() { None } else { Some(aria_label) },
            aria_pressed: pressed,
            onclick: move |_| {
                if hoverable {
                    on_activate.call(());
                }
            },
            onkeydown: move |event| {
                if !hoverable {
                    return;
                }
                let activate = match event.key() {
                    Key::Enter => true,
                    Key::Character(value) if value == " " => true,
                    _ => false,
                };
                if activate {
                    event.prevent_default();
                    on_activate.call(());
                }
            },
            {children}
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum TitleSize {
    Small,
    #[default]
    Middle,
    Large,
}

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum TitleColor {
    #[default]
    Default,
    AppTeal,
}

#[component]
pub fn Title(
    #[props(default)] size: TitleSize,
    #[props(default)] color: TitleColor,
    children: Element,
) -> Element {
    let font_size = match size {
        TitleSize::Small => 14,
        TitleSize::Middle => 20,
        TitleSize::Large => 28,
    };
    let color_class = match color {
        TitleColor::Default => "",
        TitleColor::AppTeal => "animal-color-app-teal-QdItW",
    };
    rsx! {
        span { class: "animal-title-3j08r",
            span {
                class: "animal-ribbon-S4CL0 {color_class}",
                style: "font-size: {font_size}px;",
                span { class: "animal-ribbonBack-cW93o animal-ribbonBackLeft-TZiON", aria_hidden: "true" }
                span { class: "animal-ribbonBack-cW93o animal-ribbonBackRight-AipiN", aria_hidden: "true" }
                span { class: "animal-ribbonFold-qTBSV animal-ribbonFoldLeft-gVuCs", aria_hidden: "true" }
                span { class: "animal-ribbonFold-qTBSV animal-ribbonFoldRight-94--n", aria_hidden: "true" }
                span { class: "animal-ribbonFront-qihIp", aria_hidden: "true" }
                span { class: "animal-ribbonText-FUXBT", {children} }
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TagColor {
    Pink,
    Teal,
    Green,
}

#[component]
pub fn Tag(color: TagColor, children: Element) -> Element {
    let color_class = match color {
        TagColor::Pink => "animal-color-app-pink-soft-2mFMQ",
        TagColor::Teal => "animal-color-app-teal-soft-WIR5z",
        TagColor::Green => "animal-color-app-green-soft-GjH-8",
    };
    rsx! {
        span {
            class: "animal-tag-xuymV animal-size-small-6UPtL animal-variant-soft-xTrNB {color_class}",
            {children}
        }
    }
}
