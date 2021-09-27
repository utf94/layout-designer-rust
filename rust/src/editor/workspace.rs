use generational_arena::{Arena, Index};
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlElement};

use crate::{
    component::Component,
    page::{layout::Layout, Page},
};

pub enum Selection {
    Layout(Layout),
    None,
}

impl Selection {
    pub fn set_is_selected(&mut self, is: bool) {
        match self {
            Self::Layout(layout) => layout.set_is_selected(is),
            Self::None => {}
        }
    }
}

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

    /// Currently selected item
    selection: Selection,
}

impl Workspace {
    /// Initialize the workspace
    pub fn new() -> Self {
        let document = web_sys::window().unwrap().document().unwrap();

        let html_element = document.get_element_by_id("workspace").unwrap();
        let html_element: HtmlElement = html_element.dyn_into().unwrap();

        Self {
            html_element,
            components: Arena::new(),
            pages: Vec::new(),
            selection: Selection::None,
        }
    }

    pub fn set_selection(&mut self, mut selection: Selection) {
        self.selection.set_is_selected(false);

        selection.set_is_selected(true);
        self.selection = selection;
    }

    pub fn insert_page(&mut self, page: Page) {
        page.append_to(&self.html_element);
        self.pages.push(page);
    }

    /// Determines whether the workspace contains a given html element
    pub fn contains(&self, elm: &Element) -> bool {
        self.html_element.contains(Some(elm))
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

    pub fn pages(&self) -> &[Page] {
        &self.pages
    }

    pub fn pages_mut(&mut self) -> &mut [Page] {
        &mut self.pages
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

    /// Get mutable ref to the components arena
    pub fn components_mut(&mut self) -> &mut Arena<Component> {
        &mut self.components
    }

    /// Add new component into workspace
    pub fn insert_component(&mut self, value: Component) -> generational_arena::Index {
        self.components.insert(value)
    }

    /// Remove the component from the workspace
    ///
    /// Tracking it after this is done is not posible
    pub fn remove_component(&mut self, component: &mut Component) -> Option<Component> {
        if let Some(layout) = component.layout() {
            // Finda a page that it belongs to
            let page = self
                .pages_mut()
                .iter_mut()
                .find(|page| page.contains(&layout));

            if let Some(page) = page {
                let mut layouts = page.layouts_mut();
                let layout = layouts.iter_mut().find(|l| **l == layout);

                if let Some(layout) = layout {
                    layout.remove_component(component);
                }
            }
        }

        self.components.remove(component.index())
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
