use wasm_bindgen::JsCast;
use web_sys::{Document, HtmlElement};

use super::grid::Grids;
use crate::component::Component;

use super::drag_transform::DragTransform;

pub struct MoveController {
    document: Document,
    workspace: HtmlElement,

    pub component: Component,
    drag_state: Option<DragTransform>,
    grids: Grids,
}

impl MoveController {
    pub fn new(component: Component) -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let workspace = document.get_element_by_id("workspace").unwrap();
        let workspace: HtmlElement = workspace.dyn_into().unwrap();

        Self {
            document,
            workspace,

            component,
            drag_state: None,
            grids: Grids::new(),
        }
    }

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
                .set_position(component_rect.left() as i32, component_rect.top() as i32);

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

        self.drag_state = Some(DragTransform::start(
            self.component.clone(),
            event.client_x(),
            event.client_y(),
        ));
    }

    pub fn mouse_move(&mut self, event: web_sys::MouseEvent) {
        if let Some(drag_state) = self.drag_state.as_mut() {
            drag_state.drag(event.client_x(), event.client_y());

            {
                let component_rect = drag_state.component.element().get_bounding_client_rect();

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
                        self.grids.show(container);

                        self.grids.resize_placeholder(
                            container,
                            component_rect.width(),
                            component_rect.height(),
                        );
                        self.grids
                            .move_placeholder_to(container, component_x, component_y);
                    } else {
                        self.grids.hide();
                    }
                } else {
                    self.grids.hide();
                }
            }
        } else {
            self.drag_start(event);
        }
    }

    pub fn mouse_up(&mut self, _event: web_sys::MouseEvent) {
        self.document.set_onmousemove(None);
        self.document.set_onmouseup(None);

        let workspace_rect = self.workspace.get_bounding_client_rect();
        let elem_rect = self.component.element().get_bounding_client_rect();

        let remove = !(elem_rect.left() >= workspace_rect.left()
            && elem_rect.left() <= workspace_rect.left() + workspace_rect.width()
            && elem_rect.top() >= workspace_rect.top()
            && elem_rect.top() <= workspace_rect.top() + workspace_rect.height());

        if remove {
            self.component.remove();
        } else if let Some(drag_state) = self.drag_state.as_mut() {
            let component_rect = drag_state.component.element().get_bounding_client_rect();

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

            let mut offset = (0, 0);

            if let Some(container) = container {
                if container.class_list().contains("grid") {
                    let grid = self.grids.get_grid(container);

                    self.component
                        .set_grid_pos((grid.placeholder_pos.0, grid.placeholder_pos.1));
                    self.component
                        .set_grid_size((grid.placeholder_size.0, grid.placeholder_size.1));
                } else if container.class_list().contains("free") {
                    let rect = container.get_bounding_client_rect();
                    offset = (rect.left() as i32, rect.top() as i32);
                }

                self.component
                    .element()
                    .style()
                    .remove_property("position")
                    .unwrap();

                container.append_child(self.component.element()).unwrap();
            } else {
                self.workspace
                    .append_child(self.component.element())
                    .unwrap();
                self.component.remove();
            }

            drag_state.stop(offset);

            // drag_state::stop sets a absolute pos of a component, but we dont need it when we are in a grid/flex
            // TODO(poly): Find a a cleaner solution
            if let Some(container) = container {
                if container.class_list().contains("grid")
                    || container.class_list().contains("flex")
                {
                    self.component.unset_pos();
                }
            }

            crate::editor::with_editor_state(|editor| {
                if let Some(layou_elm) = container {
                    editor.insert_component_into_layout(layou_elm, self.component.index());
                }
            });
        }

        self.component.set_is_dragged(false);
        self.grids.hide();

        self.component
            .element()
            .style()
            .remove_property("pointer-events")
            .unwrap();
    }
}
