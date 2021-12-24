#![recursion_limit = "512"]
use wasm_bindgen::prelude::*;

use log::*;
use wasm_bindgen_futures::{spawn_local, JsFuture};
use wasm_bindgen::JsValue;
use wasm_bindgen::JsCast;

use js_sys::WebAssembly::{Module, Instance};
use js_sys::Object;

use serde::{Deserialize, Serialize};

struct WasmResultObject {
    module: Module,
    instance: Instance,
}

#[wasm_bindgen(start)]
pub fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    set_panic_hook();

    info!("Setup complete, trying to load plugin");

    let empty_import = js_sys::Map::new();
    let imports = js_sys::Map::new();
    imports.set(&JsValue::from("wbg"), &empty_import);

    let window = web_sys::window().unwrap();
    let plugin_promise = window.fetch_with_str("plugin_bg.wasm");
    let wasm_promise = js_sys::WebAssembly::instantiate_streaming(&plugin_promise, &imports);

    spawn_local(async move {
        let wasm_result = JsFuture::from(wasm_promise).await.unwrap();

        info!("{:?}", wasm_result)
       //wasm_instance.instance.exports.__wbindgen_start();
        // continue with wasm_module
    });
}

fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function to get better error messages if we ever panic.
    console_error_panic_hook::set_once();
}
