use std::cell::RefCell;

use wasm_bindgen::prelude::*;

mod workspace;
use workspace::Workspace;

mod parameters_panel;
use parameters_panel::ParametersPanel;

use crate::{component::ComponentSource, elements::component::EditorComponentSource};

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

    pub fn update_parameters_panel(&mut self) {
        self.parameters_panel
            .update_components_tree(&self.workspace);
    }
}

thread_local! {
    static EDITOR_STATE: RefCell<EditorState> = RefCell::new(EditorState::new());
}

pub fn with_editor_state<F, R>(f: F) -> R
where
    F: FnOnce(&mut EditorState) -> R,
{
    EDITOR_STATE.with(|s| f(&mut s.borrow_mut()))
}

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
        // Call to thread local `with` method, will cause it to initialize
        with_editor_state(|_| {});
        Self {}
    }

    pub fn register_component(&mut self, desc: JsValue) {
        let descriptor = crate::elements::component::ComponentDescriptor::new(desc);
        let source = EditorComponentSource::new(descriptor);
        ComponentSource::new(source);
    }
}
