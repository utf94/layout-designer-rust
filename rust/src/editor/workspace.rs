use generational_arena::{Arena, Index};
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlElement};

use crate::{
    component::Component,
    page::{layout::Layout, Page},
};

/// Workspace is an area in the middle of the editor.
///
/// All of the pages are placed inside of it.
pub struct Workspace {
    /// Root html element of the Workspace
    html_element: HtmlElement,

    /// List of all components known to the editor
    components: Arena<Component>,

    /// List of all pages in the workspace
    pages: Vec<Page>,
}

impl Workspace {
    /// Initialize the workspace
    pub fn new() -> Self {
        let document = web_sys::window().unwrap().document().unwrap();

        let html_element = document.get_element_by_id("workspace").unwrap();
        let html_element: HtmlElement = html_element.dyn_into().unwrap();

        let mut pages = Vec::new();

        // Add a debug page
        {
            let mut page = Page::new("Home", 765);

            page.append_to(&html_element);

            // Add some debug layouts
            page.insert_layout(Layout::new_flex(765, 76));
            page.insert_layout(Layout::new_grid(765, 225, 76));
            page.insert_layout(Layout::new_free(765, 255));

            pages.push(page);
        }

        Self {
            html_element,
            components: Arena::new(),
            pages,
        }
    }

    /// Determines whether the workspace contains a given html element
    pub fn contains(&self, elm: &Element) -> bool {
        self.html_element.contains(Some(elm))
    }

    /// Called by editor to notify the workspace about click events
    pub fn on_mouse_click(&mut self, target: &HtmlElement, event: &web_sys::MouseEvent) {
        for page in self.pages.iter_mut() {
            if page.contains(target) {
                page.on_mouse_click(target, event);

                page.set_is_selected(page == target);
            } else {
                page.set_is_selected(false);
            }
        }
    }

    /// Get a page by html element
    #[allow(unused)]
    pub fn get_page(&self, elm: &HtmlElement) -> Option<&Page> {
        self.pages.iter().find(|page| page == &elm)
    }

    /// Get a mutable page by html element
    #[allow(unused)]
    pub fn get_page_mut(&mut self, elm: &HtmlElement) -> Option<&mut Page> {
        self.pages.iter_mut().find(|page| page == &elm)
    }

    /// Get unmutable ref to the components arena
    ///
    /// Used mostly to iterate over all components.
    ///
    /// ### Example:
    /// ```rs
    /// for (id,component) in workspace.components().iter(){
    /// }
    /// ```
    pub fn components(&self) -> &Arena<Component> {
        &self.components
    }

    /// Add new component into workspace
    pub fn insert_component(&mut self, value: Component) -> generational_arena::Index {
        self.components.insert(value)
    }

    /// Remove the component from the workspace
    ///
    /// Tracking it after this is done is not posible
    pub fn remove_component(&mut self, i: generational_arena::Index) -> Option<Component> {
        self.components.remove(i)
    }

    /// Insert component into a layout
    pub fn insert_component_into_layout(&mut self, layou_elm: &HtmlElement, id: Index) {
        // Try to find a page that contains suplied layout
        let page = self
            .pages
            .iter_mut()
            .find(|page| page.layouts().iter().any(|layout| layout == layou_elm));

        // Get component from id
        // This should always succeed, as long as component is alive
        let component = self.components.get_mut(id);

        // If both page and component were found:
        if let (Some(page), Some(component)) = (page, component) {
            page.insert_component_into_layout(layou_elm, component);
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
