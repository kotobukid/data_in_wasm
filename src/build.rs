use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use serde_json;
use serde::{Deserialize};
use std::fs;

#[derive(Deserialize, Debug)]
struct Item {
    id: u32,
    name: String,
    description: String,
}

#[derive(Deserialize, Debug)]
struct Data {
    items: Vec<Item>,
}

fn main() {
    let data = fs::read_to_string("data.json")
        .expect("Should have been able to read the file");
    let d_items: Data = serde_json::from_str(&data).expect("?");

    let dest_path = PathBuf::from("src").join("data_included.rs");
    let mut f = File::create(&dest_path).unwrap();
    write!(f, "use lazy_static::lazy_static;
use item::Item;
use crate::item;
lazy_static! {{ pub static ref ITEMS: Vec<Item> = vec![").unwrap();
    // write!(f, "static DATA: &str = [").unwrap();
    for item in d_items.items {
        write!(f, "Item::new({}, \"{}\", \"{}\"),", item.id, item.name, item.description).unwrap();
    }
    write!(f, "];}}").unwrap();
}