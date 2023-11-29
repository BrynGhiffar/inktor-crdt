use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct SVGDoc;

#[wasm_bindgen]
impl SVGDoc {
    pub fn new() -> Self {
        return SVGDoc { };
    }
    pub fn repr(&self) -> String {
        return String::from("cat");
    }
}
