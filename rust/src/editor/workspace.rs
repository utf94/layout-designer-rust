use std::{
    cell::{Ref, RefCell},
    rc::Rc,
};

use generational_arena::Arena;
use web_sys::HtmlElement;

use crate::component::Component;

struct InternalData {
    components: Arena<Component>,
}

#[derive(Clone)]
pub struct Workspace {
    data: Rc<RefCell<InternalData>>,
    body: HtmlElement,
}

impl Workspace {
    pub fn new() -> Self {
        let document = web_sys::window().unwrap().document().unwrap();
        let body = document.body().unwrap();

        Self {
            data: Rc::new(RefCell::new(InternalData {
                components: Arena::new(),
            })),
            body,
        }
    }

    pub fn components(&self) -> Ref<Arena<Component>> {
        Ref::map(self.data.borrow(), |data| &data.components)
    }

    pub fn insert_component(&self, value: Component) -> generational_arena::Index {
        let mut data = self.data.borrow_mut();
        data.components.insert(value)
    }

    pub fn remove_component(&self, i: generational_arena::Index) -> Option<Component> {
        let mut data = self.data.borrow_mut();
        data.components.remove(i)
    }

    pub fn update(&self) {
        let mut data = self.data.borrow_mut();
        data.components
            .retain(|_, c| self.body.contains(Some(c.element())));
    }
}
