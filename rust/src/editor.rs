use std::{cell::RefCell, rc::Rc};

use wasm_bindgen::prelude::*;

mod workspace;
use workspace::Workspace;

mod parameters_panel;
use parameters_panel::ParametersPanel;

use crate::{
    component::{Component, ComponentSource},
    elements::component::EditorComponentSource,
};

#[derive(Clone, Copy)]
pub enum LayoutMode {
    Flex,
    Grid,
}

impl Default for LayoutMode {
    fn default() -> Self {
        Self::Grid
    }
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct EditorState {
    workspace: Workspace,
    parameters_panel: ParametersPanel,

    layout_mode: Rc<RefCell<LayoutMode>>,
}

impl EditorState {
    fn new() -> Self {
        let parameters_panel = ParametersPanel::new();
        Self {
            parameters_panel,
            workspace: Workspace::new(),
            layout_mode: Default::default(),
        }
    }

    pub fn insert_component(&self, value: Component) -> generational_arena::Index {
        let id = self.workspace.insert_component(value);

        self.parameters_panel
            .update_components_tree(&self.workspace);

        id
    }

    pub fn remove(&self, i: generational_arena::Index) -> Option<Component> {
        let out = self.workspace.remove_component(i);

        self.parameters_panel
            .update_components_tree(&self.workspace);

        out
    }

    pub fn update(&self) {
        self.workspace.update();

        self.parameters_panel
            .update_components_tree(&self.workspace);
    }

    pub fn layout_mode(&self) -> LayoutMode {
        *self.layout_mode.borrow()
    }

    pub fn set_layout_mode(&mut self, layout_mode: LayoutMode) {
        *self.layout_mode.borrow_mut() = layout_mode;
    }
}

thread_local! {
    static EDITOR_STATE: EditorState = EditorState::new();
}

pub fn get_editor_state() -> EditorState {
    EDITOR_STATE.with(|s| s.clone())
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
        Self {}
    }

    pub fn register_component(&mut self, desc: JsValue) {
        let descriptor = crate::elements::component::ComponentDescriptor::new(desc);
        let source = EditorComponentSource::new(descriptor);
        ComponentSource::new(source);
    }
}
