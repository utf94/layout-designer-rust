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

pub mod hierarchy;
use hierarchy::Hierarchy;

use crate::drag_controller::move_controller::{DragMoveResult, MoveController};
use crate::drag_controller::resize_controller::DragResizeResult;
use crate::drag_controller::resize_controller::ResizeController;
use crate::page::layout::Layout;
use crate::page::Page;
use crate::{
    component::ComponentSource,
    html_elements::component::{ComponentDescriptor, EditorComponentSource},
};

#[derive(Clone, PartialEq)]
pub enum Selection {
    Layout(Layout),
    None,
}

impl Selection {
    pub fn set_is_selected(&mut self, is: bool) {
        match self {
            Self::Layout(layout) => layout.set_is_selected(is),
            Self::None => {}
        }
    }
}

/// The main state of the whole editor
pub struct EditorState {
    component_picker: ComponentPicker,
    pub workspace: Workspace,

    hierarchy: Hierarchy,

    parameters_panel: ParametersPanel,

    drag_state: DragState,

    /// Currently selected item
    selection: Selection,
}

impl EditorState {
    fn new() -> Self {
        let parameters_panel = ParametersPanel::new();

        let mut workspace = Workspace::new();
        // Add a debug page
        {
            let mut page = Page::new("Home", 908);

            // Add some debug layouts
            page.insert_layout(Layout::new_flex(908, 76), None);
            page.insert_layout(Layout::new_grid(908), None);
            page.insert_layout(Layout::new_free(908, 255), None);

            workspace.insert_page(page);
        }

        let hierarchy = Hierarchy::new();
        hierarchy.update(&workspace);

        Self {
            component_picker: ComponentPicker::new(),
            workspace,
            parameters_panel,

            hierarchy,

            drag_state: DragState::None,
            selection: Selection::None,
        }
    }

    /// Resize one of pages in workspace
    fn resize_page(&mut self, page: &HtmlElement, width: u32) {
        let document = web_sys::window().unwrap().document().unwrap();

        let gap = 4;

        let cell_size = width / 10;

        {
            let cell_size = format!("{}", cell_size);

            let pattern = document.get_element_by_id("grid-pattern").unwrap();
            pattern.set_attribute("width", &cell_size).unwrap();
            pattern.set_attribute("height", &cell_size).unwrap();
        }

        {
            let cell_size = format!("{}", cell_size - gap * 2);

            let rect = document.get_element_by_id("grid-pattern__rect").unwrap();
            rect.set_attribute("width", &cell_size).unwrap();
            rect.set_attribute("height", &cell_size).unwrap();
        }

        if let Some(page) = self.workspace.get_page_mut(page) {
            page.resize(width);
        }
    }

    /// Resize one of layouts in workspace
    fn resize_layout(&mut self, layout: &HtmlElement, height: u32) {
        // Finda a page that it belongs to
        let page = self
            .workspace
            .pages_mut()
            .iter_mut()
            .find(|page| page.contains(layout));

        if let Some(page) = page {
            let mut layouts = page.layouts_mut();
            let layout = layouts.iter_mut().find(|l| *l == layout);

            if let Some(layout) = layout {
                layout.resize(None, Some(height));
            }
        }
    }

    /// Let the parameters pannel know that something in the workspace has changed, and it should update
    fn update_tree(&mut self) {
        self.parameters_panel
            .update_debug_components_tree(&self.workspace);
        self.hierarchy.update(&self.workspace);
    }

    fn set_selection(&mut self, mut selection: Selection) {
        self.selection.set_is_selected(false);

        selection.set_is_selected(true);

        self.parameters_panel.set_selected(&selection);
        self.selection = selection;
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
            MouseEventKind::Click => {
                let add_btn = web_sys::window()
                    .unwrap()
                    .document()
                    .unwrap()
                    .get_element_by_id("add-page-btn")
                    .unwrap();

                if add_btn.contains(Some(target)) {
                    for page in self.workspace.pages() {
                        page.html_element
                            .style()
                            .set_property("display", "none")
                            .unwrap();
                    }
                    // Add a debug page
                    {
                        let page = Page::new("Home", 908);

                        self.workspace.insert_page(page);
                        self.update_tree();
                    }
                } else if self.workspace.contains(target) {
                    // Finda a page that it belongs to
                    let page = self
                        .workspace
                        .pages_mut()
                        .iter_mut()
                        .find(|page| page.contains(target));

                    if let Some(page) = page {
                        let layout = page.layouts().iter().find(|l| l == &target).cloned();

                        if let Some(layout) = layout {
                            if event.button() == 0 {
                                let selection = Selection::Layout(layout);
                                self.set_selection(selection)
                            }
                        } else {
                            let layout = page
                                .layouts()
                                .iter()
                                .position(|l| l.close_icon_element().contains(Some(target)));
                            if let Some(id) = layout {
                                if let Some(layout) = page.remove_layout(id) {
                                    for component in layout.components().iter() {
                                        component.remove();
                                        self.workspace.components_mut().remove(component.index());
                                    }

                                    layout.remove();
                                    self.update_tree();
                                }
                            }

                            self.set_selection(Selection::None);
                        }
                    } else {
                        self.set_selection(Selection::None);
                    }
                }
            }
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
                                        DragState::Move(MoveController::new(component));
                                } else if event.button() == 2 {
                                    self.drag_state =
                                        DragState::Resize(ResizeController::new(component));
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
                    DragState::Resize(s) => s.mouse_move(&mut self.workspace, event),
                    _ => {}
                };
            }
            MouseEventKind::MouseUp => {
                match self.drag_state.take() {
                    DragState::Move(drag) => {
                        let res = drag.mouse_up(&mut self.workspace, event);

                        match res {
                            DragMoveResult::MovedToLayout {
                                mut component,
                                mut layout,
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
                                        let mut layouts = page.layouts_mut();
                                        let layout = layouts.iter_mut().find(|l| **l == layout);

                                        if let Some(layout) = layout {
                                            layout.remove_component(&mut component);
                                        }
                                    }
                                }

                                layout.insert_component(component);
                            }
                            DragMoveResult::Removed { mut component } => {
                                component.remove();
                                self.workspace.remove_component(&mut component);
                            }
                            DragMoveResult::NotStarted { component } => {
                                if !self.workspace.contains(component.element()) {
                                    component.remove();
                                }
                            }
                        }

                        self.update_tree();
                    }
                    DragState::Resize(drag) => {
                        if let DragResizeResult::Resized {
                            mut layout,
                            component,
                        } = drag.mouse_up(event)
                        {
                            // Reinsert component with new size
                            layout.insert_component(component);
                        }
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
    pub fn resize_page(&mut self, page: &HtmlElement, width: u32) {
        with_editor_state(|editor| {
            editor.resize_page(page, width);
        })
    }

    /// Resize one of layouts in workspace
    pub fn resize_layout(&mut self, layout: &HtmlElement, height: u32) {
        with_editor_state(|editor| {
            editor.resize_layout(layout, height);
        })
    }

    pub fn add_layout_to_page(&mut self, page: &HtmlElement, id: usize, layout_kind: &str) {
        with_editor_state(|editor| {
            let page = editor.workspace.get_page_mut(page);
            if let Some(page) = page {
                match layout_kind {
                    "grid" => {
                        page.insert_layout(Layout::new_grid(page.width()), Some(id));
                    }
                    "flex" => {
                        page.insert_layout(Layout::new_flex(page.width(), 76), Some(id));
                    }
                    "free" => {
                        page.insert_layout(Layout::new_free(page.width(), 76), Some(id));
                    }
                    _ => {}
                }

                editor.update_tree();
            }
        });
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
