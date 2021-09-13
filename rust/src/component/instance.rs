use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

use crate::{elements::component::EditorComponent, utils};

/// Instance of a component
#[derive(Clone)]
pub struct Component {
    element: EditorComponent,
}

impl Component {
    pub fn new(element: EditorComponent) -> Self {
        let document = web_sys::window().unwrap().document().unwrap();

        document.body().unwrap().append_child(&element).unwrap();

        let component = Self { element };

        {
            let editor = crate::editor::get_editor_state();

            let id = editor.insert_component(component.clone());
            let id = id.into_raw_parts();

            component
                .element()
                .set_id(&format!("component-{}-{}", id.0, id.1));
        }

        component
    }

    pub fn wrap(element: EditorComponent) -> Self {
        Self { element }
    }

    pub fn element(&self) -> &EditorComponent {
        &self.element
    }

    pub fn parent(&self) -> Option<HtmlElement> {
        self.element
            .parent_element()
            .and_then(|parent| parent.dyn_into().ok())
    }

    pub fn set_is_dragged(&self, is: bool) {
        if is {
            self.element.class_list().add_1("dragged").unwrap();
        } else {
            self.element.class_list().remove_1("dragged").unwrap();
        }
    }

    pub fn set_is_selected(&self, is: bool) {
        if is {
            self.element.class_list().add_1("selected").unwrap();
        } else {
            self.element.class_list().remove_1("selected").unwrap();
        }
    }

    pub fn set_position(&self, x: i32, y: i32) {
        self.element
            .style()
            .set_property("left", &format!("{}px", x))
            .unwrap();
        self.element
            .style()
            .set_property("top", &format!("{}px", y))
            .unwrap();
    }

    pub fn size(&self) -> (f64, f64) {
        let bbox = self.element.get_bounding_client_rect();
        (bbox.width(), bbox.height())
    }

    pub fn set_size(&self, w: f64, h: f64) {
        self.element
            .style()
            .set_property("width", &format!("{}px", w))
            .unwrap();
        self.element
            .style()
            .set_property("height", &format!("{}px", h))
            .unwrap();
    }

    pub fn unset_pos(&self) {
        self.element.style().remove_property("top").unwrap();
        self.element.style().remove_property("left").unwrap();
        self.element.style().remove_property("position").unwrap();
    }

    pub fn remove(&self) {
        self.element.class_list().add_1("death-animation").unwrap();

        let onanimationend =
            utils::new_listener(self.clone(), |component, event: web_sys::AnimationEvent| {
                if event.animation_name() == "death-animation" {
                    component.element().remove();

                    let editor = crate::editor::get_editor_state();
                    editor.update();
                }
            });

        self.element
            .add_event_listener_with_callback("animationend", &onanimationend)
            .unwrap();
    }
}
