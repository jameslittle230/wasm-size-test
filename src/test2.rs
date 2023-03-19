use serde::Deserialize;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[derive(Deserialize)]
pub struct MetadataFilter {
    pub key: String,
    pub value: String,
}

#[derive(Deserialize)]
pub enum SearchTerm {
    Inexact(String),
    Exact(String),
    MetadataFilter(MetadataFilter),
}

#[wasm_bindgen]
pub fn test2(value: JsValue) {
    let terms: Vec<SearchTerm> = serde_wasm_bindgen::from_value(value).unwrap();
    dbg!(terms.len());
}
