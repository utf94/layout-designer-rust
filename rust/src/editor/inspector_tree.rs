#![allow(unused)]

use gloo_events::EventListener;
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlElement};

use crate::page::Page;

pub struct InspectorTree {
    /// Root html element of the Inspector
    html_element: HtmlElement,

    layouts: Vec<EventListener>,
}

impl InspectorTree {
    /// Initialize the workspace
    pub fn new() -> Self {
        let document = web_sys::window().unwrap().document().unwrap();

        let html_element = document.get_element_by_id("inspector-tree").unwrap();
        let html_element: HtmlElement = html_element.dyn_into().unwrap();

        Self {
            html_element,
            layouts: Default::default(),
        }
    }

    /// Determines whether the tree contains a given html element
    pub fn contains(&self, elm: &Element) -> bool {
        self.html_element.contains(Some(elm))
    }

    pub fn insert_page(&mut self, page: &Page) {
        //
    }
}

impl PartialEq<Element> for InspectorTree {
    fn eq(&self, html_element: &Element) -> bool {
        let root: &Element = self.html_element.as_ref();
        root == html_element
    }
}
