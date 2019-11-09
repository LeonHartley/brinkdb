use std::collections::{HashMap, BTreeMap};
use crate::store::BrinkData;
use serde_json::Value;
use jsonpath::Selector;

pub mod search;

#[cfg(test)]
pub mod test;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct BrinkIndex {
    pub key: String,
    pub json_selector: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct BrinkIndexValue {
    pub key: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct BrinkIndexStore {
    pub indexes: HashMap<String, BrinkIndex>,
    pub values: HashMap<String, BTreeMap<String, Vec<BrinkIndexValue>>>,
}

impl BrinkIndexStore {
    pub fn new() -> BrinkIndexStore {
        BrinkIndexStore { indexes: HashMap::new(), values: HashMap::new() }
    }

    pub fn add(&mut self, index: BrinkIndex) {
        self.indexes.insert(index.key.clone(), index);
    }
}

impl BrinkIndex {
    pub fn new(key: String, json_selector: String) -> BrinkIndex {
        BrinkIndex { key, json_selector }
    }

    pub fn parse(key: &String, json: String, store: &mut BrinkIndexStore) {
        if let Ok(value) = serde_json::from_str::<Value>(&json) {
            for index in store.indexes.values() {
                let selector = Selector::new(&index.json_selector).unwrap();
                let matches: Vec<&str> = selector.find(&value)
                    .map(|t| t.as_str().unwrap())
                    .collect();

                let values = match store.values.get_mut(&index.key) {
                    Some(mut map) => map,
                    None => {
                        store.values.insert(index.key.clone(), BTreeMap::new());

                        store.values.get_mut(&index.key).unwrap()
                    }
                };

                for &m in &matches {
                    let value = m.into();
                    let index_value = BrinkIndexValue {
                        key: key.clone(),
                        value,
                    };

                    if values.contains_key(&index_value.value) {
                        values.get_mut(&index_value.value).unwrap().push(index_value);
                    } else {
                        values.insert(index_value.value.clone(), vec![index_value]);
                    }
                };
            }
        }
    }
}
