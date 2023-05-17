use wasm_bindgen::prelude::*;
use js_sys::{Object, Reflect};


#[wasm_bindgen]
#[derive(Clone)]
pub struct Item {
    id: i32,
    name: String,
    description: String,
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
    pub fn name(&self) -> String {
        self.name.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn description(&self) -> String {
        self.description.clone()
    }
}

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref ITEMS: Vec<Item> = vec![
        Item::new(1, "taro", "hello world"),
        Item::new(2, "jiro", "hello world"),
        Item::new(3, "ginji", "hello world"),
    ];
}

#[wasm_bindgen]
pub fn filter(word: &str) -> js_sys::Array {
    let items: Vec<JsValue> = ITEMS.iter()
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