use generational_arena::{Arena, Index};
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

use crate::{
    component::Component,
    page::{layout::Layout, Page},
};

pub struct Workspace {
    /// Root html element of a Workspace
    _html_element: HtmlElement,

    components: Arena<Component>,

    /// A debug page, should be replaced with Vec<Page> at some point
    page: Page,
}

impl Workspace {
    pub fn new() -> Self {
        let document = web_sys::window().unwrap().document().unwrap();

        let html_element = document.get_element_by_id("workspace").unwrap();
        let html_element: HtmlElement = html_element.dyn_into().unwrap();

        let mut page = Page::new("Home");

        page.append_to(&html_element);

        // Add some debug layouts
        {
            page.insert_layout(Layout::new_flex(765, 76));
            page.insert_layout(Layout::new_grid(765, 225));
            page.insert_layout(Layout::new_free(765, 255));
        }

        Self {
            _html_element: html_element,
            components: Arena::new(),
            page,
        }
    }

    pub fn components(&self) -> &Arena<Component> {
        &self.components
    }

    pub fn insert_component(&mut self, value: Component) -> generational_arena::Index {
        self.components.insert(value)
    }

    pub fn remove_component(&mut self, i: generational_arena::Index) -> Option<Component> {
        self.components.remove(i)
    }

    pub fn insert_component_into_layout(&mut self, layou_elm: &HtmlElement, id: Index) {
        if let Some(component) = self.components.get_mut(id) {
            self.page
                .insert_component_into_layout(layou_elm, (id, component));
        }
    }

    /// Remove all components that are no longer in the DOM tree
    ///
    /// Curently not used anywhere, but it may be usefull
    #[allow(unused)]
    fn update(&mut self) {
        let document = web_sys::window().unwrap().document().unwrap();
        let body = document.body().unwrap();

        self.components
            .retain(|_, c| body.contains(Some(c.element())));
    }
}
