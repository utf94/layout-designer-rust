use wasm_bindgen::JsCast;
use web_sys::{Document, HtmlElement};

use crate::{
    component::Component,
    editor::workspace::Workspace,
    page::layout::{Layout, LayoutKind},
};

mod css_transform;
use css_transform::CssMoveTransform;

pub enum DragMoveResult {
    MovedToLayout {
        component: Component,
        absolute_pos: (i32, i32),
        layout: HtmlElement,
    },
    Removed {
        component: Component,
    },
    NotStarted {
        component: Component,
    },
}

pub struct MoveController {
    document: Document,

    component: Component,

    drag_state: Option<CssMoveTransform>,
    layout: Option<Layout>,
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
            layout: None,
        }
    }

    /// Start the component drag
    pub fn drag_start(&mut self, event: &web_sys::MouseEvent) {
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
            self.component
                .set_size(component_rect.width(), component_rect.height());

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
    pub fn mouse_move(&mut self, workspace: &mut Workspace, event: &web_sys::MouseEvent) {
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
                    let component = &self.component;

                    // Finda a page that it belongs to
                    let page = workspace
                        .pages()
                        .iter()
                        .find(|page| page.contains(container));

                    if let Some(page) = page {
                        if let Some(layout) = page.find_layout_by_element(container) {
                            let (bbox_pos, _bbox_size) = layout.bounding_client_rect();

                            if let LayoutKind::Grid {
                                grid_data,
                                grid_background,
                                cell_size,
                                ..
                            } = &mut *layout.kind_mut()
                            {
                                let grid_w = grid_data.width();
                                let grid_h = grid_data.height();

                                let sub_x = component_x - bbox_pos.0;
                                let sub_y = component_y - bbox_pos.1;

                                let div_x = sub_x / *cell_size as f64;
                                let div_y = sub_y / *cell_size as f64;

                                let grid_x = div_x.floor() as usize + 1;
                                let grid_y = div_y.floor() as usize + 1;

                                let placeholder_size =
                                    self.component.grid_size().unwrap_or_else(|| {
                                        let width = component_rect.width();
                                        let height = component_rect.height();

                                        let w = width / *cell_size as f64;
                                        let h = height / *cell_size as f64;

                                        let w = w.ceil() as usize;
                                        let h = h.ceil() as usize;

                                        (w, h)
                                    });

                                let grid_x = grid_x.min(grid_w - placeholder_size.0 + 1).max(0);
                                let grid_y = grid_y.min(grid_h - placeholder_size.1 + 1).max(0);

                                grid_background.update_placeholder(
                                    workspace,
                                    grid_data,
                                    component,
                                    (grid_x, grid_y),
                                    placeholder_size,
                                );
                            }

                            if let Some(l) = self.layout.take() {
                                if l != layout {
                                    if let LayoutKind::Grid {
                                        grid_background, ..
                                    } = &mut *l.kind_mut()
                                    {
                                        grid_background.set_placeholder_visible(false);
                                    }
                                }
                            }

                            self.layout = Some(layout);
                        }
                    }
                }
            }
        } else {
            self.drag_start(event);
            self.mouse_move(workspace, event);
        }
    }

    /// Called when mouse is up
    pub fn mouse_up(
        mut self,
        _workspace: &mut Workspace,
        _event: &web_sys::MouseEvent,
    ) -> DragMoveResult {
        self.document.set_onmousemove(None);
        self.document.set_onmouseup(None);

        self.component.set_is_dragged(false);

        // TODO(poly): goddammit, there has to be a cleaner way to do all of this

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

            self.component
                .element()
                .style()
                .remove_property("pointer-events")
                .unwrap();

            let elements: Vec<_> = elements
                .iter()
                .filter_map(|elm| elm.dyn_into::<HtmlElement>().ok())
                .filter(|elm| elm.class_list().contains("container"))
                .collect();

            let container = elements.first();

            if let Some(layout) = self.layout.as_ref() {
                if let LayoutKind::Grid {
                    grid_background, ..
                } = &mut *layout.kind_mut()
                {
                    grid_background.set_placeholder_visible(false);
                }
            }

            if let Some(container) = container {
                let new_absolute_pos = if container.class_list().contains("grid") {
                    if let Some(layout) = self.layout.as_ref() {
                        if let LayoutKind::Grid {
                            grid_background, ..
                        } = &*layout.kind()
                        {
                            let placeholder_pos = grid_background.placeholder_pos();
                            let placeholder_size = grid_background.placeholder_size();

                            if !grid_background.is_placeholder_denied() {
                                self.component.unset_absolute_pos();
                                self.component.unset_size();

                                self.component.set_grid_pos(placeholder_pos);
                                self.component.set_grid_size(placeholder_size);
                            }
                        }
                    }

                    // The component does not have grid size, so this is initial drag and drop
                    // And component was droped into ocupied spot
                    if self.component.grid_pos().is_none() || self.component.grid_size().is_none() {
                        return DragMoveResult::Removed {
                            component: self.component,
                        };
                    }

                    drag_state.stop()
                } else if container.class_list().contains("flex") {
                    self.component.unset_absolute_pos();

                    drag_state.stop()
                } else if container.class_list().contains("free") {
                    let new_absolute_pos = drag_state.stop();

                    let rect = container.get_bounding_client_rect();
                    let offset = (rect.left() as i32, rect.top() as i32);
                    let pos = (new_absolute_pos.0 - offset.0, new_absolute_pos.1 - offset.1);

                    self.component.set_position(pos);

                    new_absolute_pos
                } else {
                    return DragMoveResult::Removed {
                        component: self.component,
                    };
                };

                // Move has ended so now the layout is responsible for positioning
                // So we remove the position property
                self.component
                    .element()
                    .style()
                    .remove_property("position")
                    .unwrap();

                container.append_child(self.component.element()).unwrap();

                DragMoveResult::MovedToLayout {
                    component: self.component,
                    layout: container.clone(),
                    absolute_pos: new_absolute_pos,
                }
            } else {
                DragMoveResult::Removed {
                    component: self.component,
                }
            }
        } else {
            self.component
                .element()
                .style()
                .remove_property("pointer-events")
                .unwrap();

            DragMoveResult::NotStarted {
                component: self.component,
            }
        }
    }
}
