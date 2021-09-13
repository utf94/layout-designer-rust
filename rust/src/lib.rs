use wasm_bindgen::prelude::*;

pub mod component;

mod editor;
mod grid;
mod drag_controler;
mod utils;

mod elements;

// Wasm entry point
#[wasm_bindgen(start)]
pub fn start() -> ::std::result::Result<(), JsValue> {
    console_log::init_with_level(log::Level::Debug).unwrap();
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    elements::component::register();

    Ok(())
}
