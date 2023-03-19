use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn test3(values: Vec<wasm_bindgen::JsValue>) {
    dbg!(values.len());
}
