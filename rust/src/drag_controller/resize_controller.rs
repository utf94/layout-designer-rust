use wasm_bindgen::JsCast;
use web_sys::{Document, HtmlElement};

use crate::component::Component;

use super::grid::{Grid, Grids};

struct ResizeState {
    pub component: Component,

    last_x: i32,
    last_y: i32,
}

impl ResizeState {
    /// Start the resize of a component
    pub fn start(component: Component, x: i32, y: i32) -> Self {
        let last_x = x;
        let last_y = y;

        Self {
            component,
            last_x,
            last_y,
        }
    }

    /// Called when mouse is being draged
    pub fn drag(&mut self, grid: Option<&Grid>, x: i32, y: i32) {
        let dx = self.last_x - x;
        let dy = self.last_y - y;

        let (w, h) = self.component.size();

        // Check if we are in a gird
        // If so resize cell by cell
        // Otherwise just resize freely
        if let Some(_grid) = grid {
            let (pos_x, pos_y) = self.component.grid_pos();
            let (size_x, size_y) = self.component.grid_size();

            if -dx > 76 {
                self.last_x = x;

                self.component.set_grid_size((size_x + 1, size_y));

                self.component
                    .element()
                    .style()
                    .set_property("grid-column", &format!("{}/span {}", pos_x, size_x + 1))
                    .unwrap();
            }

            if -dx < -76 && size_x > 1 {
                self.last_x = x;

                self.component.set_grid_size((size_x - 1, size_y));

                self.component
                    .element()
                    .style()
                    .set_property("grid-column", &format!("{}/span {}", pos_x, size_x - 1))
                    .unwrap();
            }

            if -dy > 76 {
                self.last_y = y;

                self.component.set_grid_size((size_x, size_y + 1));

                self.component
                    .element()
                    .style()
                    .set_property("grid-row", &format!("{}/span {}", pos_y, size_y + 1))
                    .unwrap();
            }

            if -dy < -76 && size_y > 1 {
                self.last_y = y;

                self.component.set_grid_size((size_x, size_y - 1));

                self.component
                    .element()
                    .style()
                    .set_property("grid-row", &format!("{}/span {}", pos_y, size_y - 1))
                    .unwrap();
            }
        } else {
            self.last_x = x;
            self.last_y = y;

            self.component.set_size(w - dx as f64, h - dy as f64);
        }
    }

    pub fn stop(&mut self) {}
}

pub struct ResizeController {
    document: Document,

    pub component: Component,
    drag_state: Option<ResizeState>,

    grids: Grids,
    grid_element: Option<HtmlElement>,
}

impl ResizeController {
    /// Init the move controler for a component
    pub fn new(component: Component) -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        Self {
            document,

            component,
            drag_state: None,
            grids: Grids::new(),
            grid_element: None,
        }
    }

    /// Start the component drag
    pub fn drag_start(&mut self, event: web_sys::MouseEvent) {
        self.component
            .element()
            .style()
            .set_property("pointer-events", "none")
            .unwrap();

        self.component.set_is_selected(true);

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
                    self.grid_element = Some(container.clone());
                }
            }
        }

        self.drag_state = Some(ResizeState::start(
            self.component.clone(),
            event.client_x(),
            event.client_y(),
        ));
    }

    /// Called when mouse moves
    pub fn mouse_move(&mut self, event: web_sys::MouseEvent) {
        if let Some(drag_state) = self.drag_state.as_mut() {
            if let Some(elm) = self.grid_element.as_ref() {
                drag_state.drag(
                    Some(self.grids.get_grid(elm)),
                    event.client_x(),
                    event.client_y(),
                );
            } else {
                drag_state.drag(None, event.client_x(), event.client_y());
            }
        } else {
            self.drag_start(event);
        }
    }

    /// Called when mouse moves
    pub fn mouse_up(&mut self, _event: web_sys::MouseEvent) {
        self.document.set_onmousemove(None);
        self.document.set_onmouseup(None);

        if let Some(drag_state) = self.drag_state.as_mut() {
            drag_state.stop();
        }

        self.component.set_is_selected(false);

        self.component
            .element()
            .style()
            .remove_property("pointer-events")
            .unwrap();
    }
}
