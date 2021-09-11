use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub enum ComponentPropertie {
    Color = "color",
    Text = "text",
    Number = "number",
    LayoutStyle = "layout_style",
    Boolean = "boolean",
}

#[wasm_bindgen(module = "/src/elements/component.js")]
extern "C" {
    pub fn register();
}

#[wasm_bindgen(module = "/src/elements/component.js")]
extern "C" {
    #[derive(Debug, Clone)]
    pub type ComponentParameter;

    #[wasm_bindgen(method, getter)]
    pub fn name(this: &ComponentParameter) -> String;

    #[wasm_bindgen(method, getter)]
    pub fn data_type(this: &ComponentParameter) -> ComponentPropertie;
}

#[wasm_bindgen(module = "/src/elements/component.js")]
extern "C" {
    #[derive(Debug, Clone)]
    pub type ComponentDescriptor;

    #[wasm_bindgen(constructor)]
    pub fn new(desc: JsValue) -> ComponentDescriptor;

    #[wasm_bindgen(method, getter)]
    pub fn tag_name(this: &ComponentDescriptor) -> String;

    #[wasm_bindgen(method, getter, js_name = "parameters")]
    fn parameters_array(this: &ComponentDescriptor) -> js_sys::Array;

}

impl ComponentDescriptor {
    pub fn parameters(&self) -> Vec<ComponentParameter> {
        self.parameters_array()
            .iter()
            .map(|val| val.dyn_into().unwrap())
            .collect()
    }
}

#[wasm_bindgen(module = "/src/elements/component.js")]
extern "C" {
    #[wasm_bindgen(extends = HtmlElement)]
    #[derive(Debug, Clone)]
    pub type EditorComponentSource;

    #[wasm_bindgen(constructor)]
    pub fn new(desc: ComponentDescriptor) -> EditorComponentSource;

    #[wasm_bindgen(method, getter)]
    pub fn instance(this: &EditorComponentSource) -> HtmlElement;

    #[wasm_bindgen(method, getter)]
    pub fn descriptor(this: &EditorComponentSource) -> ComponentDescriptor;

    #[wasm_bindgen(method)]
    pub fn instantiate_component(this: &EditorComponentSource) -> EditorComponent;
}

#[wasm_bindgen(module = "/src/elements/component.js")]
extern "C" {
    #[wasm_bindgen(extends = HtmlElement)]
    #[derive(Debug, Clone)]
    pub type EditorComponent;

    #[wasm_bindgen(constructor)]
    pub fn new(desc: ComponentDescriptor) -> EditorComponent;

    #[wasm_bindgen(method, getter)]
    pub fn instance(this: &EditorComponent) -> HtmlElement;

    #[wasm_bindgen(method, getter)]
    pub fn descriptor(this: &EditorComponent) -> ComponentDescriptor;
}
