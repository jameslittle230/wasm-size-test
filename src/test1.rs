use regex::Regex;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn test1() {
    let regex = Regex::new(
        // sorry
        r#"(?P<key>[^"^\s]+)=(?P<val>"(?P<qval>[^"]+)"|[^"^\s]+)|(?:^|[^=])"(?P<quote>[^"\\]+(?:\\.[^"\\]*)*)""#,
    )
    .unwrap();

    dbg!(regex.captures(r#""a="b c""#).unwrap());
}
