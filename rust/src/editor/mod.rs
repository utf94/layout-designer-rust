use std::cell::RefCell;

use gloo_events::EventListener;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

pub mod workspace;
use web_sys::HtmlElement;
use workspace::Workspace;

mod parameters_panel;
use parameters_panel::ParametersPanel;

mod component_picker;
use component_picker::ComponentPicker;

mod inspector_tree;

use crate::drag_controller::move_controller::{MouseUpResult, MoveController};
use crate::drag_controller::resize_controller::ResizeController;
use crate::page::layout::Layout;
use crate::page::Page;
use crate::{
    component::ComponentSource,
    html_elements::component::{ComponentDescriptor, EditorComponentSource},
};

/// The main state of the whole editor
pub struct EditorState {
    component_picker: ComponentPicker,
    pub workspace: Workspace,
    parameters_panel: ParametersPanel,

    drag_state: DragState,
}

impl EditorState {
    fn new() -> Self {
        let parameters_panel = ParametersPanel::new();

        let mut workspace = Workspace::new();
        // Add a debug page
        {
            let mut page = Page::new("Home", 765);

            // Add some debug layouts
            page.insert_layout(Layout::new_flex(765, 76));
            page.insert_layout(Layout::new_grid(765, 225, 76));
            page.insert_layout(Layout::new_free(765, 255));

            workspace.insert_page(page);
        }

        Self {
            component_picker: ComponentPicker::new(),
            workspace,
            parameters_panel,

            drag_state: DragState::None,
        }
    }

    /// Let the parameters pannel know that something in the workspace has changed, and it should update
    pub fn update_parameters_panel(&mut self) {
        self.parameters_panel
            .update_components_tree(&self.workspace);
    }

    fn on_mouse_event(&mut self, kind: MouseEventKind, event: &web_sys::MouseEvent) {
        if let Some(target) = event.target() {
            if let Ok(target) = target.dyn_into::<HtmlElement>() {
                self.handle_mouse_event(kind, event, &target);
            }
        }
    }

    /// Called by editor to notify the workspace about mouse events
    fn handle_mouse_event(
        &mut self,
        kind: MouseEventKind,
        event: &web_sys::MouseEvent,
        target: &HtmlElement,
    ) {
        match kind {
            MouseEventKind::Click => {}
            MouseEventKind::MouseDown => {
                if self.drag_state.is_none() {
                    // If clicked element is in a workspace
                    if self.workspace.contains(target) {
                        // Finda a page that it belongs to
                        let page = self
                            .workspace
                            .pages()
                            .iter()
                            .find(|page| page.contains(target));

                        if let Some(page) = page {
                            if let Some(component) = page.find_component_by_element(target) {
                                if event.button() == 0 {
                                    self.drag_state =
                                        DragState::Move(MoveController::new(component.clone()));
                                } else if event.button() == 2 {
                                    self.drag_state =
                                        DragState::Resize(ResizeController::new(component.clone()));
                                }
                            }
                        }
                    } else if self.component_picker.contains(target) {
                        let source = self
                            .component_picker
                            .sources()
                            .iter()
                            .find(|source| source.contains(target));

                        if let Some(source) = source {
                            let component = source.new_instance();
                            let id = self.workspace.insert_component(component);

                            let component = self.workspace.components_mut().get_mut(id).unwrap();
                            component.set_id(id);

                            self.drag_state =
                                DragState::Move(MoveController::new(component.clone()));
                        }
                    }
                }
            }
            MouseEventKind::MouseMove => {
                match &mut self.drag_state {
                    DragState::Move(s) => s.mouse_move(&mut self.workspace, event),
                    DragState::Resize(s) => s.mouse_move(event),
                    _ => {}
                };
            }
            MouseEventKind::MouseUp => {
                match self.drag_state.take() {
                    DragState::Move(drag) => {
                        let res = drag.mouse_up(event);

                        match res {
                            MouseUpResult::MovedToLayout {
                                mut component,
                                layout,
                                ..
                            } => {
                                if let Some(layout) = component.layout() {
                                    // Finda a page that it belongs to
                                    let page = self
                                        .workspace
                                        .pages_mut()
                                        .iter_mut()
                                        .find(|page| page.contains(&layout));

                                    if let Some(page) = page {
                                        let layout =
                                            page.layouts_mut().iter_mut().find(|l| &**l == &layout);

                                        if let Some(layout) = layout {
                                            layout.remove_component(&mut component);
                                        }
                                    }
                                }

                                self.workspace
                                    .insert_component_into_layout(&layout, component.index());
                            }
                            MouseUpResult::Removed { component } => {
                                component.remove();
                                self.workspace.remove_component(component.index());
                            }
                            MouseUpResult::NotStarted { component } => {
                                if !self.workspace.contains(component.element()) {
                                    component.remove();
                                }
                            }
                        }

                        self.update_parameters_panel();
                    }
                    DragState::Resize(drag) => {
                        drag.mouse_up(event);
                    }
                    _ => {}
                };
            }
        }
    }
}

thread_local! {
    /// A global variable that stores the state of the editor
    ///
    /// The variable is thread local, in order to avoid the need for Mutex.
    /// With thread local var the RefCell is enought to check mutability rules.
    ///
    /// NOTE(poly): Not sure if we want to have global state in the long run,
    /// but in some parts of the codebase we are in callback hell, so it's easier to just acces a global,
    /// instead of trying to refcount it across every callback
    static EDITOR_STATE: RefCell<EditorState> = RefCell::new(EditorState::new());
}

/// Helper function used to acces the global editor state
pub fn with_editor_state<F, R>(f: F) -> R
where
    F: FnOnce(&mut EditorState) -> R,
{
    EDITOR_STATE.with(|s| f(&mut s.borrow_mut()))
}

/// Editor struct used as an API surface bettwen JS and Rust
#[wasm_bindgen]
pub struct Editor {}

impl Default for Editor {
    fn default() -> Self {
        Self::new()
    }
}

#[wasm_bindgen]
impl Editor {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        // First Call to thread local `with` method, will cause it to initialize
        with_editor_state(|_| {});

        register_editor_listeners();
        Self {}
    }

    /// Register a new kind of component
    pub fn register_component(&mut self, desc: JsValue) {
        let descriptor = ComponentDescriptor::new(desc);
        let source = EditorComponentSource::new(descriptor);

        let src = ComponentSource::new(source);

        with_editor_state(|editor| {
            editor.component_picker.insert_source(src);
        });
    }

    /// Resize one of pages in workspace
    pub fn resize_page(&mut self, page: &HtmlElement, width: usize) {
        with_editor_state(|editor| {
            if let Some(page) = editor.workspace.get_page_mut(page) {
                page.resize(width);
            }
        })
    }
}

#[derive(Clone, Copy)]
pub enum MouseEventKind {
    Click,

    MouseDown,
    MouseMove,
    MouseUp,
}

/// Register listeneres for the editor
fn register_editor_listeners() {
    let document = web_sys::window().unwrap().document().unwrap();

    let click = EventListener::new(&document, "click", |event| {
        let event = event.dyn_ref().unwrap();
        with_editor_state(|editor| editor.on_mouse_event(MouseEventKind::Click, event))
    });
    click.forget();

    let mouse_down = EventListener::new(&document, "mousedown", |event| {
        let event = event.dyn_ref().unwrap();
        with_editor_state(|editor| editor.on_mouse_event(MouseEventKind::MouseDown, event))
    });
    mouse_down.forget();

    let mouse_move = EventListener::new(&document, "mousemove", |event| {
        let event = event.dyn_ref().unwrap();
        with_editor_state(|editor| editor.on_mouse_event(MouseEventKind::MouseMove, event))
    });
    mouse_move.forget();

    let mouse_up = EventListener::new(&document, "mouseup", |event| {
        let event = event.dyn_ref().unwrap();
        with_editor_state(|editor| editor.on_mouse_event(MouseEventKind::MouseUp, event))
    });
    mouse_up.forget();
}

enum DragState {
    Move(MoveController),
    Resize(ResizeController),
    None,
}

impl Default for DragState {
    fn default() -> Self {
        Self::None
    }
}

impl DragState {
    fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }

    fn take(&mut self) -> Self {
        std::mem::take(self)
    }
}
