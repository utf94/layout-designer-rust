use wasm_bindgen::JsCast;
use web_sys::{Document, HtmlElement};

use crate::{
    component::Component,
    editor::workspace::Workspace,
    page::layout::{Layout, LayoutKind},
};

pub enum DragResizeResult {
    Resized {
        layout: Layout,
        component: Component,
    },
    NotResized,
}

struct ResizeState {
    layout: Layout,
    component: Component,

    last_x: i32,
    last_y: i32,
}

impl ResizeState {
    /// Start the resize of a component
    pub fn start(layout: Layout, component: Component, x: i32, y: i32) -> Self {
        let last_x = x;
        let last_y = y;

        if let LayoutKind::Grid {
            grid_background, ..
        } = &mut *layout.kind_mut()
        {
            grid_background.set_placeholder_visible(true);
        }

        Self {
            layout,
            component,
            last_x,
            last_y,
        }
    }

    /// Called when mouse is being draged
    pub fn drag(&mut self, workspace: &mut Workspace, x: i32, y: i32) {
        let dx = -(self.last_x - x);
        let dy = -(self.last_y - y);

        let (w, h) = self.component.size();

        // Check if we are in a gird
        // If so resize cell by cell
        // Otherwise just resize freely
        if let LayoutKind::Grid {
            cell_size,
            grid_data,
            grid_background,
            ..
        } = &mut *self.layout.kind_mut()
        {
            let (pos_x, pos_y) = self.component.grid_pos().unwrap();
            let (size_x, size_y) = self.component.grid_size().unwrap();

            let cell_size = *cell_size as i32;

            let horizontal = dx.abs() > cell_size;
            let vertical = dy.abs() > cell_size;

            if horizontal {
                if dx > cell_size && pos_x + size_x < grid_data.width() + 1 {
                    self.last_x = x;

                    self.component.set_grid_size((size_x + 1, size_y));

                    self.component
                        .element()
                        .style()
                        .set_property("grid-column", &format!("{}/span {}", pos_x, size_x + 1))
                        .unwrap();
                } else if dx < -cell_size && size_x > 1 {
                    self.last_x = x;

                    self.component.set_grid_size((size_x - 1, size_y));

                    self.component
                        .element()
                        .style()
                        .set_property("grid-column", &format!("{}/span {}", pos_x, size_x - 1))
                        .unwrap();
                }
            }

            if vertical {
                if dy > cell_size && pos_y + size_y < grid_data.height() + 1 {
                    self.last_y = y;

                    self.component.set_grid_size((size_x, size_y + 1));

                    self.component
                        .element()
                        .style()
                        .set_property("grid-row", &format!("{}/span {}", pos_y, size_y + 1))
                        .unwrap();
                } else if dy < -cell_size && size_y > 1 {
                    self.last_y = y;

                    self.component.set_grid_size((size_x, size_y - 1));

                    self.component
                        .element()
                        .style()
                        .set_property("grid-row", &format!("{}/span {}", pos_y, size_y - 1))
                        .unwrap();
                }
            }

            if horizontal || vertical {
                grid_background.update_placeholder(
                    workspace,
                    grid_data,
                    &self.component,
                    self.component.grid_pos().unwrap(),
                    self.component.grid_size().unwrap(),
                );
            }
        } else {
            self.last_x = x;
            self.last_y = y;

            self.component.set_size(w + dx as f64, h + dy as f64);
        }
    }

    pub fn stop(&mut self) {
        if let LayoutKind::Grid {
            grid_background, ..
        } = &mut *self.layout.kind_mut()
        {
            grid_background.set_placeholder_visible(false);
        }
    }
}

pub struct ResizeController {
    document: Document,

    pub component: Component,
    drag_state: Option<ResizeState>,
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
        }
    }

    /// Start the component drag
    pub fn drag_start(&mut self, workspace: &mut Workspace, event: &web_sys::MouseEvent) {
        self.component
            .element()
            .style()
            .set_property("pointer-events", "none")
            .unwrap();

        self.component.set_is_selected(true);

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

        let layout = elements.first().and_then(|container| {
            // Finda a page that it belongs to
            let page = workspace
                .pages()
                .iter()
                .find(|page| page.contains(container));

            page.and_then(|page| page.find_layout_by_element(container))
        });

        self.drag_state = layout.map(|layout| {
            ResizeState::start(
                layout,
                self.component.clone(),
                event.client_x(),
                event.client_y(),
            )
        });
    }

    /// Called when mouse moves
    pub fn mouse_move(&mut self, workspace: &mut Workspace, event: &web_sys::MouseEvent) {
        if let Some(drag_state) = self.drag_state.as_mut() {
            drag_state.drag(workspace, event.client_x(), event.client_y());
        } else {
            self.drag_start(workspace, event);
        }
    }

    /// Called when mouse moves
    pub fn mouse_up(mut self, _event: &web_sys::MouseEvent) -> DragResizeResult {
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

        DragResizeResult::NotResized
    }
}
