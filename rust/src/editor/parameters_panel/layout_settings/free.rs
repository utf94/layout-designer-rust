use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

pub fn settings() -> HtmlElement {
    let document = web_sys::window().unwrap().document().unwrap();

    let root = document.create_element("section").unwrap();
    let root: HtmlElement = root.dyn_into().unwrap();
    root.append_child(&super::title("Free")).unwrap();
    root
}
