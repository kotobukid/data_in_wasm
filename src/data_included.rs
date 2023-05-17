use lazy_static::lazy_static;
use item::Item;
use crate::item;
lazy_static! { pub static ref ITEMS: Vec<Item> = vec![Item::new(1, "taro", "I am taro"),Item::new(2, "jiro", "I am jiro"),Item::new(3, "ginji", "I am ginji"),Item::new(1, "hanako", "I am hanako"),];}