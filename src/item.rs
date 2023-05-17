use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

#[wasm_bindgen]
#[derive(Clone)]
pub struct Item {
    pub id: i32,
    pub name: String,
    pub description: String,
}

#[wasm_bindgen]
impl Item {
    #[wasm_bindgen(constructor)]
    pub fn new(id: i32, name: &str, description: &str) -> Item {
        Item {
            id,
            name: name.to_string(),
            description: description.to_string(),
        }
    }

    #[wasm_bindgen(getter)]
    pub fn id(&self) -> i32 {
        self.id
    }

    #[wasm_bindgen(getter)]
    pub fn name(&self) -> JsValue {
        JsValue::from_str(&self.name)
    }

    #[wasm_bindgen(getter)]
    pub fn description(&self) -> JsValue {
        JsValue::from_str(&self.description)
    }
}