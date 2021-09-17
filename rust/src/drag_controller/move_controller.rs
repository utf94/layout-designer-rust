use wasm_bindgen::JsCast;
use web_sys::{Document, HtmlElement};

use super::grid::Grids;
use crate::component::Component;

mod css_transform;
use css_transform::CssMoveTransform;

pub struct MoveController {
    document: Document,

    component: Component,
    drag_state: Option<CssMoveTransform>,
    grids: Grids,
}

impl MoveController {
    /// Init the move controler for a component
    pub fn new(component: Component) -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        Self {
            document,

            component,
            drag_state: None,
            grids: Grids::new(),
        }
    }

    /// Start the component drag
    pub fn drag_start(&mut self, event: web_sys::MouseEvent) {
        self.component
            .element()
            .style()
            .set_property("pointer-events", "none")
            .unwrap();
        self.component.set_is_dragged(true);

        if self.component.parent().unwrap() != self.document.body().unwrap() {
            let component_rect = self.component.element().get_bounding_client_rect();
            self.component
                .set_position((component_rect.left() as i32, component_rect.top() as i32));

            self.document
                .body()
                .unwrap()
                .append_child(self.component.element())
                .unwrap();
        }

        self.component
            .element()
            .style()
            .set_property("position", "absolute")
            .unwrap();

        self.drag_state = Some(CssMoveTransform::start(
            self.component.clone(),
            event.client_x(),
            event.client_y(),
        ));
    }

    /// Called when mouse moves
    pub fn mouse_move(&mut self, event: web_sys::MouseEvent) {
        if let Some(drag_state) = self.drag_state.as_mut() {
            drag_state.drag(event.client_x(), event.client_y());

            {
                let component_rect = self.component.element().get_bounding_client_rect();

                let component_x = component_rect.left();
                let component_y = component_rect.top();
                let component_w = component_rect.width();
                let component_h = component_rect.height();

                let elements = self.document.elements_from_point(
                    (component_x + component_w / 2.0) as f32,
                    (component_y + component_h / 2.0) as f32,
                );

                let elements: Vec<_> = elements
                    .iter()
                    .filter_map(|elm| elm.dyn_into::<HtmlElement>().ok())
                    .filter(|elm| elm.class_list().contains("container"))
                    .collect();

                if let Some(container) = elements.first() {
                    if container.class_list().contains("grid") {
                        self.grids.resize_placeholder(
                            container,
                            component_rect.width(),
                            component_rect.height(),
                        );
                        self.grids
                            .move_placeholder_to(container, component_x, component_y);
                    }
                }
            }
        } else {
            self.drag_start(event);
        }
    }

    /// Called when mouse is up
    pub fn mouse_up(&mut self, _event: web_sys::MouseEvent) {
        self.document.set_onmousemove(None);
        self.document.set_onmouseup(None);

        if let Some(drag_state) = self.drag_state.as_mut() {
            let component_rect = self.component.element().get_bounding_client_rect();

            let component_x = component_rect.left();
            let component_y = component_rect.top();
            let component_w = component_rect.width();
            let component_h = component_rect.height();

            let elements = self.document.elements_from_point(
                (component_x + component_w / 2.0) as f32,
                (component_y + component_h / 2.0) as f32,
            );

            let elements: Vec<_> = elements
                .iter()
                .filter_map(|elm| elm.dyn_into::<HtmlElement>().ok())
                .filter(|elm| elm.class_list().contains("container"))
                .collect();

            let container = elements.first();

            if let Some(container) = container {
                let new_absolute_pos = drag_state.stop();

                if container.class_list().contains("grid") {
                    let grid = self.grids.get_grid(container);

                    self.component.unset_absolute_pos();
                    self.component
                        .set_grid_pos((grid.placeholder_pos.0, grid.placeholder_pos.1));
                    self.component
                        .set_grid_size((grid.placeholder_size.0, grid.placeholder_size.1));
                } else if container.class_list().contains("flex") {
                    self.component.unset_absolute_pos();
                } else if container.class_list().contains("free") {
                    let rect = container.get_bounding_client_rect();
                    let offset = (rect.left() as i32, rect.top() as i32);
                    let pos = (new_absolute_pos.0 - offset.0, new_absolute_pos.1 - offset.1);

                    self.component.set_position(pos);
                }

                // Move has ended so now the layout is responsible for positioning
                // So we remove the position property
                self.component
                    .element()
                    .style()
                    .remove_property("position")
                    .unwrap();

                container.append_child(self.component.element()).unwrap();
            } else {
                // We are outside of any layout, so we are removing the component
                self.component.remove();

                // Remove the component from the editor
                crate::editor::with_editor_state(|editor| {
                    editor.workspace.remove_component(self.component.index());
                    editor.update_parameters_panel();
                });
            }

            crate::editor::with_editor_state(|editor| {
                if let Some(layou_elm) = container {
                    editor
                        .workspace
                        .insert_component_into_layout(layou_elm, self.component.index());
                }
            });
        }

        self.component.set_is_dragged(false);

        self.component
            .element()
            .style()
            .remove_property("pointer-events")
            .unwrap();
    }
}
