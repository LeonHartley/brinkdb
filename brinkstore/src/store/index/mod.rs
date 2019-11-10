use std::collections::{HashMap, BTreeMap};
use crate::store::BrinkData;
use serde_json::Value;
use jsonpath::Selector;

pub mod search;
pub mod parser;

#[cfg(test)]
pub mod test;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct BrinkIndex {
    pub key: String,
    pub json_selector: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct BrinkIndexValue {
    pub key: String,
    pub value: String,
    pub version: i32,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct BrinkIndexStore {
    pub indexes: HashMap<String, BrinkIndex>,
    pub values: HashMap<String, BTreeMap<String, Vec<BrinkIndexValue>>>,
}

impl BrinkIndexStore {
    pub fn new() -> BrinkIndexStore { BrinkIndexStore { indexes: HashMap::new(), values: HashMap::new() } }

    pub fn add(&mut self, index: BrinkIndex) {
        self.indexes.insert(index.key.clone(), index);
    }
}

impl BrinkIndex {
    pub fn new(key: String, json_selector: String) -> BrinkIndex {
        BrinkIndex { key, json_selector }
    }
}
