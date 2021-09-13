use web_sys::Document;

use crate::{component::Component, grid::Grids};

struct ResizeState {
    pub component: Component,

    last_x: i32,
    last_y: i32,
}

impl ResizeState {
    pub fn start(component: Component, x: i32, y: i32) -> Self {
        let last_x = x;
        let last_y = y;

        Self {
            component,
            last_x,
            last_y,
        }
    }

    pub fn drag(&mut self, x: i32, y: i32) {
        let dx = self.last_x - x;
        let dy = self.last_y - y;

        self.last_x = x;
        self.last_y = y;

        let (w, h) = self.component.size();

        self.component.set_size(w - dx as f64, h - dy as f64);
    }

    pub fn stop(&mut self) {}
}

pub struct ResizeController {
    document: Document,

    pub component: Component,
    drag_state: Option<ResizeState>,
    grids: Grids,
}

impl ResizeController {
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

    pub fn drag_start(&mut self, event: web_sys::MouseEvent) {
        self.component
            .element()
            .style()
            .set_property("pointer-events", "none")
            .unwrap();

        self.component.set_is_selected(true);

        self.drag_state = Some(ResizeState::start(
            self.component.clone(),
            event.client_x(),
            event.client_y(),
        ));
    }

    pub fn mouse_move(&mut self, event: web_sys::MouseEvent) {
        if let Some(drag_state) = self.drag_state.as_mut() {
            drag_state.drag(event.client_x(), event.client_y());
        } else {
            self.drag_start(event);
        }
    }

    pub fn mouse_up(&mut self, _event: web_sys::MouseEvent) {
        self.document.set_onmousemove(None);
        self.document.set_onmouseup(None);

        if let Some(drag_state) = self.drag_state.as_mut() {
            drag_state.stop();
        }

        self.component.set_is_selected(false);
        self.component.set_is_dragged(false);

        self.grids.hide();

        self.component
            .element()
            .style()
            .remove_property("pointer-events")
            .unwrap();
    }
}
