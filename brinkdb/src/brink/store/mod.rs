use std::collections::{LinkedList, HashMap};

#[cfg(test)]
pub mod test;

pub struct BrinkStore {
    pub id: i32,
    pub name: String,
    pub namespaces: HashMap<i32, i32>,
    pub buffer: Vec<u8>,
    pub keys: HashMap<String, BrinkDataKey>,
}

pub struct BrinkDataKey {
    pub key: String,
    pub versions: LinkedList<BrinkDataRef>,
}

pub struct BrinkDataRef {
    pub store_id: i32,
    pub index: i32,
}

pub struct BrinkData {
    pub key: String,
    pub version: i32,
    pub blob: String,
}

impl BrinkStore {
    pub fn new(id: i32, name: String) -> BrinkStore {
        let buffer = vec!();
        let namespaces = HashMap::new();
        let keys = HashMap::new();

        BrinkStore { id, name, namespaces, buffer, keys }
    }
}

