use gloo_events::EventListener;
use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, HtmlInputElement};

use crate::page::layout::{Layout, LayoutKind};

struct HeightInput {
    root: HtmlElement,
    input: HtmlInputElement,
    click_listener: Option<EventListener>,
}

impl HeightInput {
    fn new(layout: &Layout) -> Self {
        let document = web_sys::window().unwrap().document().unwrap();

        let root = document.create_element("div").unwrap();
        let root: HtmlElement = root.dyn_into().unwrap();
        root.class_list().add_1("size-input").unwrap();

        let span = document.create_element("span").unwrap();
        let span: HtmlElement = span.dyn_into().unwrap();

        span.set_inner_text("H");
        root.append_child(&span).unwrap();

        let input = document.create_element("input").unwrap();
        let input: HtmlInputElement = input.dyn_into().unwrap();

        input.set_attribute("type", "range").unwrap();
        input.set_attribute("step", "1").unwrap();
        input.set_attribute("min", "1").unwrap();
        input.set_attribute("max", "10").unwrap();

        if let LayoutKind::Grid { grid_data, .. } = &*layout.kind() {
            let h = grid_data.height();
            input.set_value(&h.to_string());
        } else {
            input.set_value("3");
        }

        root.append_child(&input).unwrap();

        Self {
            root,
            input,
            click_listener: None,
        }
    }

    fn connect<F: FnMut(u32) + 'static>(&mut self, mut cb: F) {
        let input = self.input.clone();
        let listener = EventListener::new(&self.root, "input", move |_| {
            let value = input.value();

            let value: Option<u32> = value.parse().ok();
            //
            if let Some(value) = value {
                cb(value);
            }
        });

        self.click_listener = Some(listener);
    }
}

pub struct GridSettings {
    pub root: HtmlElement,
    height_input: HeightInput,
}

impl GridSettings {
    pub fn new(layout: &Layout) -> Self {
        let document = web_sys::window().unwrap().document().unwrap();

        let root = document.create_element("section").unwrap();
        let root: HtmlElement = root.dyn_into().unwrap();
        root.append_child(&super::title("Grid")).unwrap();

        let height_input = HeightInput::new(layout);
        root.append_child(&height_input.root).unwrap();

        Self { root, height_input }
    }

    pub fn connect_height<F: FnMut(u32) + 'static>(&mut self, cb: F) {
        self.height_input.connect(cb);
    }
}
