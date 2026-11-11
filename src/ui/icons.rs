use dioxus::prelude::*;

fn svg(size: u32, children: Element) -> Element {
    rsx! {
        svg {
            width: "{size}",
            height: "{size}",
            view_box: "0 0 48 48",
            xmlns: "http://www.w3.org/2000/svg",
            fill: "none",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            {children}
        }
    }
}

#[component]
pub fn StarIcon(#[props(default = 18)] size: u32) -> Element {
    svg(
        size,
        rsx! {
            path { d: "M24 6 L29 18 L42 19 L32 28 L35 41 L24 34 L13 41 L16 28 L6 19 L19 18 Z", fill: "#E9C46A" }
            circle { cx: "20", cy: "22", r: "1.3", fill: "#2A2A2A" }
            circle { cx: "28", cy: "22", r: "1.3", fill: "#2A2A2A" }
        },
    )
}

#[component]
pub fn ChevronLeftIcon(#[props(default = 18)] size: u32) -> Element {
    svg(
        size,
        rsx! {
            path { d: "M29 12 L 17 24 L 29 36", fill: "none", stroke: "#2A2A2A", stroke_width: "3.5" }
        },
    )
}

#[component]
pub fn ChevronRightIcon(#[props(default = 18)] size: u32) -> Element {
    svg(
        size,
        rsx! {
            path { d: "M19 12 L 31 24 L 19 36", fill: "none", stroke: "#2A2A2A", stroke_width: "3.5" }
        },
    )
}

#[component]
pub fn PlusIcon(#[props(default = 18)] size: u32) -> Element {
    svg(
        size,
        rsx! {
            circle { cx: "24", cy: "24", r: "17", fill: "#E9C46A" }
            path { d: "M24 15 L 24 33", stroke: "#2A2A2A", stroke_width: "4.5" }
            path { d: "M15 24 L 33 24", stroke: "#2A2A2A", stroke_width: "4.5" }
        },
    )
}

#[component]
pub fn MinusIcon(#[props(default = 18)] size: u32) -> Element {
    svg(
        size,
        rsx! {
            circle { cx: "24", cy: "24", r: "17", fill: "#E9C46A" }
            path { d: "M15 24 L 33 24", stroke: "#2A2A2A", stroke_width: "4.5" }
        },
    )
}

#[component]
pub fn CloseIcon(#[props(default = 16)] size: u32) -> Element {
    svg(
        size,
        rsx! {
            circle { cx: "24", cy: "24", r: "17", fill: "#FAEDCD" }
            path { d: "M17 17 L 31 31", stroke: "#E76F51", stroke_width: "4.5" }
            path { d: "M31 17 L 17 31", stroke: "#E76F51", stroke_width: "4.5" }
        },
    )
}

#[component]
pub fn NoDataIcon() -> Element {
    rsx! {
        svg {
            class: "animal-scene-icon-BR3MR",
            view_box: "0 0 48 48",
            xmlns: "http://www.w3.org/2000/svg",
            fill: "none",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            path { d: "M8 13 L8 39 L40 39 L40 13 Z", fill: "#8B5E3C" }
            path { d: "M11 13 L37 13 L33 22 L15 22 Z", fill: "#264653", stroke: "#2A2A2A", stroke_width: "3", stroke_linejoin: "round" }
            path { d: "M8 13 L8 39 L40 39 L40 13", stroke: "#2A2A2A", stroke_width: "3.5", fill: "none" }
            rect { x: "14", y: "26", width: "20", height: "10", rx: "2", fill: "#FAEDCD" }
            circle { cx: "20", cy: "31", r: "1.6", fill: "#2A2A2A" }
            circle { cx: "28", cy: "31", r: "1.6", fill: "#2A2A2A" }
        }
    }
}
