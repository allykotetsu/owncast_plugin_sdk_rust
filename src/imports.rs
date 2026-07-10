use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
extern "C" {
    // #[requires("chat.send")]
    #[wasm_bindgen(js_namespace = ["owncast", "chat"], js_name = send)]
    pub fn owncast_send_chat(textPtr: &str);
}