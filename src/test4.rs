use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn test4(value: String) {
    dbg!(value.len());
}
