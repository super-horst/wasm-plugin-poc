#![recursion_limit = "512"]
use wasm_bindgen::prelude::*;

use log::*;

#[wasm_bindgen(start)]
pub fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    info!("Plugin started");
}
