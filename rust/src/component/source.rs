use wasm_bindgen::JsCast;

use web_sys::{Element, HtmlElement};

use crate::html_elements::component::EditorComponentSource;

use super::Component;

/// Source of new components
/// Located in a component picker
#[derive(Clone)]
pub struct ComponentSource {
    pub root: HtmlElement,
    pub source: EditorComponentSource,
}

impl ComponentSource {
    pub fn new(source: EditorComponentSource) -> Self {
        let document = web_sys::window().unwrap().document().unwrap();

        let root = document.create_element("section").unwrap();
        root.set_class_name("component-source");
        let root: HtmlElement = root.dyn_into().unwrap();

        root.append_child(&source).unwrap();

        let app = document.get_element_by_id("picker").unwrap();
        app.append_child(&root).unwrap();

        Self { root, source }
    }

    /// Determines whether the workspace contains a given html element
    pub fn contains(&self, elm: &Element) -> bool {
        self.root.contains(Some(elm))
    }

    pub fn new_instance(&self) -> Component {
        let component = self.source.instantiate_component();
        let component = Component::new(component);

        let rect = self.source.instance().get_bounding_client_rect();
        component.set_position((rect.left() as i32, rect.top() as i32));

        component
            .element()
            .style()
            .set_property("position", "absolute")
            .unwrap();

        component
            .element()
            .class_list()
            .add_1("spawn-animation")
            .unwrap();

        component
    }
}
