use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

use crate::page::layout::{Layout, LayoutKind};

mod flex;
mod free;
mod grid;

fn title(title: &str) -> HtmlElement {
    let document = web_sys::window().unwrap().document().unwrap();

    let root = document.create_element("div").unwrap();
    let root: HtmlElement = root.dyn_into().unwrap();
    root.class_list().add_1("title").unwrap();
    root.set_inner_text(title);
    root
}

pub struct LayoutSettings {
    pub layout: Layout,
    pub root: HtmlElement,
}

impl LayoutSettings {
    pub fn new(layout: Layout) -> Self {
        let root = match &*layout.kind() {
            LayoutKind::Free { .. } => free::settings(),
            LayoutKind::Flex { .. } => flex::settings(),
            LayoutKind::Grid { .. } => grid::settings(),
        };

        Self { layout, root }
    }
}
