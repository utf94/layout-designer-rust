use std::{cell::RefCell, rc::Rc};

use generational_arena::Index;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

use crate::{html_elements::component::EditorComponent, utils};

struct InnerData {
    grid_size: (u32, u32),
    grid_pos: (u32, u32),

    /// The index of a componetn
    ///
    /// Despite the fact that this is an `Option`, it is guaranteed to be initialized
    /// It is Option because we don't know the Index when Component is initialized, we get the id after initialization
    index: Option<Index>,
}

/// Instance of a component
#[derive(Clone)]
pub struct Component {
    element: EditorComponent,
    data: Rc<RefCell<InnerData>>,
}

impl Component {
    pub fn new(element: EditorComponent) -> Self {
        let document = web_sys::window().unwrap().document().unwrap();

        document.body().unwrap().append_child(&element).unwrap();

        let component = Self {
            element,
            data: Rc::new(RefCell::new(InnerData {
                grid_size: (1, 1),
                grid_pos: (1, 1),
                index: None,
            })),
        };

        let id = crate::editor::with_editor_state(|editor| {
            let id = editor.workspace.insert_component(component.clone());

            let (number, generation) = id.into_raw_parts();

            component
                .element()
                .set_id(&format!("component-{}-{}", number, generation));

            editor.update_parameters_panel();

            id
        });

        component.data.borrow_mut().index = Some(id);

        component
    }

    pub fn index(&self) -> Index {
        self.data.borrow().index.unwrap()
    }

    pub fn element(&self) -> &EditorComponent {
        &self.element
    }

    pub fn parent(&self) -> Option<HtmlElement> {
        self.element
            .parent_element()
            .and_then(|parent| parent.dyn_into().ok())
    }

    fn update_grid_css_properties(&self) {
        let data = self.data.borrow();
        self.element
            .style()
            .set_property(
                "grid-column",
                &format!("{}/span {}", data.grid_pos.0, data.grid_size.0),
            )
            .unwrap();

        self.element
            .style()
            .set_property(
                "grid-row",
                &format!("{}/span {}", data.grid_pos.1, data.grid_size.1),
            )
            .unwrap();
    }

    pub fn grid_pos(&self) -> (u32, u32) {
        self.data.borrow().grid_pos
    }

    pub fn set_grid_pos(&mut self, pos: (u32, u32)) {
        self.data.borrow_mut().grid_pos = pos;
        self.update_grid_css_properties();
    }

    pub fn grid_size(&self) -> (u32, u32) {
        self.data.borrow().grid_size
    }

    pub fn set_grid_size(&mut self, size: (u32, u32)) {
        self.data.borrow_mut().grid_size = size;
        self.update_grid_css_properties();
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

    pub fn set_position(&self, (x, y): (i32, i32)) {
        self.element
            .style()
            .set_property("left", &format!("{}px", x))
            .unwrap();
        self.element
            .style()
            .set_property("top", &format!("{}px", y))
            .unwrap();
    }

    /// Get size in px
    pub fn size(&self) -> (f64, f64) {
        let bbox = self.element.get_bounding_client_rect();
        (bbox.width(), bbox.height())
    }

    /// Set size in px
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

    /// Unsets absolute pos
    pub fn unset_absolute_pos(&self) {
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
                }
            });

        self.element
            .add_event_listener_with_callback("animationend", &onanimationend)
            .unwrap();
    }
}
