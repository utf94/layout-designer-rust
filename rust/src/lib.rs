use wasm_bindgen::prelude::*;

pub mod component;

mod drag_controller;
mod editor;
mod utils;

mod html_elements;

mod page;

// Wasm entry point
#[wasm_bindgen(start)]
pub fn start() -> ::std::result::Result<(), JsValue> {
    console_log::init_with_level(log::Level::Debug).unwrap();
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    // NOTE(poly): This does not require unsafe
    // but rust-analyzer in my IDE is complaining that it needs it
    // and I'm to lazy to file a bug for it, so I just added the redundant unsafe
    #[allow(unused_unsafe)]
    unsafe {
        html_elements::component::register();
    }

    Ok(())
}
