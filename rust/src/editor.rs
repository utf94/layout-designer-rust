use std::cell::RefCell;

use generational_arena::Index;
use wasm_bindgen::prelude::*;

mod workspace;
use web_sys::HtmlElement;
use workspace::Workspace;

mod parameters_panel;
use parameters_panel::ParametersPanel;

use crate::{
    component::{Component, ComponentSource},
    elements::component::EditorComponentSource,
};

pub struct EditorState {
    workspace: Workspace,
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

    pub fn insert_component(&mut self, value: Component) -> generational_arena::Index {
        let id = self.workspace.insert_component(value);

        self.parameters_panel
            .update_components_tree(&self.workspace);

        id
    }

    #[allow(unused)]
    pub fn remove_component(&mut self, i: generational_arena::Index) -> Option<Component> {
        let out = self.workspace.remove_component(i);

        self.parameters_panel
            .update_components_tree(&self.workspace);

        out
    }

    pub fn insert_component_into_layout(&mut self, layou_elm: &HtmlElement, id: Index) {
        self.workspace.insert_component_into_layout(layou_elm, id);
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
