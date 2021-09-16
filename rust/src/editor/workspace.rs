use generational_arena::{Arena, Index};
use web_sys::HtmlElement;

use crate::{component::Component, layout::Layout};

pub struct Workspace {
    components: Arena<Component>,

    // Layouts shoud be inside of a page, but we don't have a page struct for now
    #[allow(unused)]
    layouts: Vec<Layout>,
}

impl Workspace {
    pub fn new() -> Self {
        let document = web_sys::window().unwrap().document().unwrap();

        let mut layouts = Vec::new();

        // Add some debug layouts
        {
            layouts.push(Layout::new_flex(765, 76));
            layouts.push(Layout::new_grid(765, 225));
            layouts.push(Layout::new_free(765, 255));

            let page = document.get_element_by_id("page").unwrap();
            for layout in layouts.iter() {
                layout.append_to(&page);
            }
        }

        Self {
            components: Arena::new(),
            layouts,
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
        let layout = self.layouts.iter_mut().find(|l| l == &layou_elm);

        if let (Some(layout), Some(component)) = (layout, self.components.get_mut(id)) {
            layout.insert_component((id, component));
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
