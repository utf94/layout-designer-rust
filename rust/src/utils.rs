use wasm_bindgen::{prelude::Closure, JsCast};

pub fn new_listener<
    EVENT: wasm_bindgen::convert::FromWasmAbi + 'static,
    D: 'static,
    F: FnMut(&mut D, EVENT) + 'static,
>(
    mut data: D,
    mut cb: F,
) -> js_sys::Function {
    Closure::wrap(Box::new(move |e: EVENT| cb(&mut data, e)) as Box<dyn FnMut(EVENT)>)
        .into_js_value()
        .unchecked_into()
}
