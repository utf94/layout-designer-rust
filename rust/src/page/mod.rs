use generational_arena::Index;
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlElement};

use crate::component::Component;

pub mod layout;
use layout::Layout;

/// The representation of a Paga
///
/// Page is an arrea where layouts are added
/// We can have multiple pages in the same workspace
pub struct Page {
    /// Root html element of a page
    ///
    /// Layouts are placed inside of it
    html_element: HtmlElement,

    /// The name of a page
    ///
    /// For example `Home`, `Contact`, `News`
    _name: String,

    /// List of layouts inside of a page,
    /// laid out one under the other
    layouts: Vec<Layout>,
}

impl Page {
    /// Create a new page, with given name
    pub fn new(name: &str) -> Self {
        let name = name.to_owned();

        let document = web_sys::window().unwrap().document().unwrap();

        let html_element = document.create_element("div").unwrap();
        let html_element: HtmlElement = html_element.dyn_into().unwrap();

        html_element.style().set_property("width", "765px").unwrap();

        html_element.class_list().add_1("page").unwrap();

        Self {
            html_element,

            _name: name,

            layouts: Vec::new(),
        }
    }

    /// Append the page to the element
    ///
    /// Used to append page to the workspace
    pub fn append_to(&self, parent: &Element) {
        parent.append_child(&self.html_element).unwrap();
    }

    /// Get unmutable list of all layouts in a page
    pub fn layouts(&self) -> &[Layout] {
        &self.layouts
    }

    /// Insert a layout into a page
    pub fn insert_layout(&mut self, layout: Layout) {
        layout.append_to(&self.html_element);

        self.layouts.push(layout);
    }

    /// Insert a component into a layout inside of this page
    pub fn insert_component_into_layout(
        &mut self,
        layou_elm: &HtmlElement,
        component: &mut Component,
    ) {
        let layout = self.layouts.iter_mut().find(|l| l == &layou_elm);

        if let Some(layout) = layout {
            layout.insert_component(component);
        }
    }
}
