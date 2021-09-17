use std::cell::RefCell;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

mod workspace;
use web_sys::HtmlElement;
use workspace::Workspace;

mod parameters_panel;
use parameters_panel::ParametersPanel;

use crate::{
    component::ComponentSource,
    html_elements::component::{ComponentDescriptor, EditorComponentSource},
    utils,
};

/// The main state of the whole editor
pub struct EditorState {
    pub workspace: Workspace,
    parameters_panel: ParametersPanel,
}

impl EditorState {
    fn new() -> Self {
        let parameters_panel = ParametersPanel::new();
        Self {
            parameters_panel,
            workspace: Workspace::new(),
        }
    }

    /// Let the parameters pannel know that something in the workspace has changed, and it should update
    pub fn update_parameters_panel(&mut self) {
        self.parameters_panel
            .update_components_tree(&self.workspace);
    }

    fn on_click(&mut self, event: web_sys::MouseEvent) {
        if let Some(target) = event.target() {
            if let Ok(elm) = target.dyn_into::<HtmlElement>() {
                if self.workspace.contains(&elm) {
                    self.workspace.on_mouse_click(&elm, &event);
                }
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

/// Register click listenere for the editor
fn register_click_listener() {
    let on_click = utils::new_listener((), |_, event: web_sys::MouseEvent| {
        with_editor_state(|editor| editor.on_click(event))
    });

    let document = web_sys::window().unwrap().document().unwrap();
    document
        .get_element_by_id("editor")
        .unwrap()
        .add_event_listener_with_callback("click", &on_click)
        .unwrap();
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

        register_click_listener();
        Self {}
    }

    /// Register a new kind of component
    pub fn register_component(&mut self, desc: JsValue) {
        let descriptor = ComponentDescriptor::new(desc);
        let source = EditorComponentSource::new(descriptor);
        ComponentSource::new(source);
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
