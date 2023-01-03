mod compute;

pub use compute::*;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(catch, js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, js_namespace = ["window", "__TAURI__", "tauri"], js_name = "invoke")]
    async fn invoke_args(cmd: &str, args: JsValue) -> Result<JsValue, JsValue>;
}
