#![recursion_limit = "512"]

use wasm_bindgen::prelude::*;

use log::*;
use wasm_bindgen_futures::{spawn_local, JsFuture};
use wasm_bindgen::JsValue;
use wasm_bindgen::JsCast;

use js_sys::{WebAssembly, Object, Function, Reflect};

#[wasm_bindgen]
pub fn main(imports: Object) {
    wasm_logger::init(wasm_logger::Config::default());
    set_panic_hook();

    info!("Setup complete, trying to load plugin with imports {:?}", imports);

    let window = web_sys::window().unwrap();
    let plugin_promise = window.fetch_with_str("plugin_bg.wasm");
    let wasm_promise = WebAssembly::instantiate_streaming(&plugin_promise, &imports);

    spawn_local(async move {
        let wasm_result = JsFuture::from(wasm_promise).await.unwrap();
        let wasm_instance: WebAssembly::Instance = Reflect::get(&wasm_result, &"instance".into()).unwrap().dyn_into().unwrap();

        let wasm_exports = wasm_instance.exports();

        let add = Reflect::get(wasm_exports.as_ref(), &"__wbindgen_start".into())
            .unwrap()
            .dyn_into::<Function>()
            .expect("wasm-bindgen start wasn't a function");

        add.call0(&JsValue::undefined()).unwrap();
        info!("Completed load of 'plugin'")
    });
}

fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function to get better error messages if we ever panic.
    console_error_panic_hook::set_once();
}
