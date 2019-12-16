use crate::store::index::{BrinkIndex, BrinkIndexStore, BrinkIndexValue};
use jsonpath::Selector;
use serde_json::Value;
use std::collections::BTreeMap;

pub trait BrinkIndexParser {
    fn parse(key: &String, json: String, version: i32, store: &mut BrinkIndexStore);
}

impl BrinkIndexParser for BrinkIndex {
    fn parse(key: &String, json: String, version: i32, store: &mut BrinkIndexStore) {
        if let Ok(value) = serde_json::from_str::<Value>(&json) {
            for index in store.indexes.values() {
                let selector = Selector::new(&index.json_selector).unwrap();
                let matches: Vec<&str> =
                    selector.find(&value).map(|t| t.as_str().unwrap()).collect();

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
                        version,
                    };

                    if values.contains_key(&index_value.value) {
                        values
                            .get_mut(&index_value.value)
                            .unwrap()
                            .push(index_value);
                    } else {
                        values.insert(index_value.value.clone(), vec![index_value]);
                    }
                }
            }
        }
    }
}
