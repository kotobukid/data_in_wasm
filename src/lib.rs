mod data_included;
mod item;

use wasm_bindgen::prelude::*;
use js_sys::{Object, Reflect};

#[wasm_bindgen]
pub fn filter(word: &str) -> js_sys::Array {
    let items: Vec<JsValue> = data_included::ITEMS.iter()
        .filter(|i| i.name.contains(word) || i.description.contains(word))
        .map(|i| {
            let obj = Object::new();
            Reflect::set(&obj, &"id".into(), &JsValue::from(i.id)).unwrap();
            Reflect::set(&obj, &"name".into(), &JsValue::from(i.name.clone())).unwrap();
            Reflect::set(&obj, &"description".into(), &JsValue::from(i.description.clone())).unwrap();
            JsValue::from(obj)
        })
        .collect();

    items.into_iter().collect::<js_sys::Array>()
}