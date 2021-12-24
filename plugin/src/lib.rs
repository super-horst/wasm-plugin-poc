#![recursion_limit = "512"]
use wasm_bindgen::prelude::*;

use log::*;

#[wasm_bindgen(start)]
pub fn main() {
    info!("Plugin started");
}
