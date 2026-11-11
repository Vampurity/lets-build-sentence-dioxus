use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::EventTarget;

pub struct DomListener {
    target: EventTarget,
    event: &'static str,
    capture: bool,
    closure: Closure<dyn FnMut(web_sys::Event)>,
}

impl DomListener {
    pub fn listen(
        target: EventTarget,
        event: &'static str,
        capture: bool,
        closure: Closure<dyn FnMut(web_sys::Event)>,
    ) -> Self {
        let _ = target.add_event_listener_with_callback_and_bool(
            event,
            closure.as_ref().unchecked_ref(),
            capture,
        );
        Self {
            target,
            event,
            capture,
            closure,
        }
    }
}

impl Drop for DomListener {
    fn drop(&mut self) {
        let _ = self
            .target
            .remove_event_listener_with_callback_and_bool(
                self.event,
                self.closure.as_ref().unchecked_ref(),
                self.capture,
            );
    }
}

pub type ListenerSlot = Rc<RefCell<Option<DomListener>>>;

pub fn listener_slot() -> ListenerSlot {
    Rc::new(RefCell::new(None))
}

pub fn window() -> Option<web_sys::Window> {
    web_sys::window()
}

pub fn document() -> Option<web_sys::Document> {
    window()?.document()
}

pub fn body() -> Option<web_sys::HtmlElement> {
    document()?.body()
}

pub fn element_by_id(id: &str) -> Option<web_sys::Element> {
    document()?.get_element_by_id(id)
}

pub fn input_by_id(id: &str) -> Option<web_sys::HtmlInputElement> {
    element_by_id(id)?.dyn_into().ok()
}

pub fn event_target(target: &web_sys::Window) -> EventTarget {
    target.clone().unchecked_into()
}

pub fn set_overflow(hidden: bool) {
    if let Some(body) = body() {
        let _ = body
            .style()
            .set_property("overflow", if hidden { "hidden" } else { "" });
    }
}
