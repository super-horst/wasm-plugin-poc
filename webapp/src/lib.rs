#![recursion_limit = "512"]

use wasm_bindgen::prelude::*;
use log::*;


#[wasm_bindgen(start)]
pub fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    set_panic_hook();

    info!("Setup complete app is ready to launch");
}

fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function to get better error messages if we ever panic.
    console_error_panic_hook::set_once();
}
