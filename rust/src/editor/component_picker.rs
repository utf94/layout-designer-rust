use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlElement};

use crate::component::ComponentSource;

pub struct ComponentPicker {
    /// Root html element of the Workspace
    html_element: HtmlElement,

    sources: Vec<ComponentSource>,
}

impl ComponentPicker {
    /// Initialize the workspace
    pub fn new() -> Self {
        let document = web_sys::window().unwrap().document().unwrap();

        let html_element = document.get_element_by_id("picker").unwrap();
        let html_element: HtmlElement = html_element.dyn_into().unwrap();

        Self {
            html_element,
            sources: Vec::new(),
        }
    }

    pub fn sources(&self) -> &[ComponentSource] {
        &self.sources
    }

    pub fn insert_source(&mut self, src: ComponentSource) {
        self.sources.push(src);
    }

    /// Determines whether the workspace contains a given html element
    pub fn contains(&self, elm: &Element) -> bool {
        self.html_element.contains(Some(elm))
    }
}

impl PartialEq<Element> for ComponentPicker {
    fn eq(&self, html_element: &Element) -> bool {
        let root: &Element = self.html_element.as_ref();
        root == html_element
    }
}
