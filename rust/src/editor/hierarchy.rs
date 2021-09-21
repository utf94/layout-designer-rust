#![allow(unused)]
/// Include relevent crates and modules
use gloo_events::EventListener;
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlElement};
use crate::page::Page;

/// Hierarchy Struct to represent inspector in editor
pub struct Hierarchy {
    /// Root html element of the Hierarchy
    html_element: HtmlElement,
}

/// Methods for Hierarchy Struct
impl Hierarchy {
    /// Create new instance of the hierarchy
    pub fn new() -> Self {
        let document = web_sys::window().unwrap().document().unwrap();
        let html_element = document.get_element_by_id("inspector-tree").unwrap();
        let html_element: HtmlElement = html_element.dyn_into().unwrap();
        Self { 
            html_element,
        }
    }

    /// Determines whether the tree contains a given html element
    pub fn contains(&self, elm: &Element) -> bool {
        self.html_element.contains(Some(elm))
    }
}

/// Partial equivalence relation for Hierarchy Struct
impl PartialEq<Element> for Hierarchy {
    fn eq(&self, html_element: &Element) -> bool {
        let root: &Element = self.html_element.as_ref();
        root == html_element
    }
}
