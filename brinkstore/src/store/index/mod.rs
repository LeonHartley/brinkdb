use std::collections::{HashMap, BTreeMap};
use crate::store::BrinkData;
use serde_json::Value;
use jsonpath::Selector;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct BrinkIndex {
    pub key: String,
    pub json_selector: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct BrinkIndexValue {
    pub key: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct BrinkIndexStore {
    indexes: HashMap<String, BrinkIndex>,
    values: BTreeMap<String, HashMap<String, BrinkIndexValue>>,
}

impl BrinkIndexStore {
    pub fn new() -> BrinkIndexStore {
        let mut store = BrinkIndexStore {
            indexes: HashMap::new(),
            values: BTreeMap::new(),
        };

        store.indexes.insert("name".into(), BrinkIndex {
            key: "name".into(),
            json_selector: "$.name".into(),
        });

        store.values.insert("name".into(), HashMap::new());

        store
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

                let value = match store.values.get_mut(&index.key) {
                    Some(mut map) => map,
                    None => {
                        store.values.insert(index.key.clone(), HashMap::new());
                        store.values.get_mut(&index.key).unwrap()
                    }
                };

                for &m in &matches {
                    value.insert(key.clone(), BrinkIndexValue {
                        key: index.key.clone(),
                        value: m.into(),
                    });
                }
                println!("{:?}, store: {:?}", matches, &store);
            }
        } else {
            println!("nope, value: {}", json);
        }
    }
}

