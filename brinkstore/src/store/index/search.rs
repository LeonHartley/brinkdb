use crate::store::index::{BrinkIndexStore, BrinkIndexValue};
use std::time::Instant;
use std::error::Error;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct BrinkIndexSearchKey {
    key: String,
    value: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct BrinkIndexSearch {
    keys: Vec<BrinkIndexSearchKey>
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum BrinkIndexSearchResult {
    Ok(Vec<BrinkIndexValue>),
    None,
}

impl BrinkIndexSearchKey {
    pub fn new(key: String, value: String) -> BrinkIndexSearchKey {
        BrinkIndexSearchKey { key, value }
    }
}

impl BrinkIndexSearch {
    pub fn new(keys: Vec<BrinkIndexSearchKey>) -> BrinkIndexSearch {
        BrinkIndexSearch { keys }
    }

    pub fn search(&self, store: &BrinkIndexStore) -> BrinkIndexSearchResult {
        let watch = Instant::now();
        let mut results: HashMap<&String, i32> = HashMap::new();
        let keys: Vec<String> = self.keys.iter()
            .filter_map(|key| {
                let index = store.values.get(&key.key);
                if let Some(index) = index {
                    if let Some(keys) = index.get(&key.value) {
                        Some(keys
                            .iter()
                            .map(|f| f.key.clone())
                            .collect())
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .flat_map(|m: Vec<String>| m)
            .collect();

        for key in &keys {
            if let Some(count) = results.get(&key.clone()) {
                results.insert(key, count + 1);
            } else {
                results.insert(key, 1);
            }
        }

        let v: Vec<String> = results.into_iter().filter_map(|(key, val)| {
            if val as usize == self.keys.len() {
                Some(key.clone())
            } else {
                None
            }
        }).collect();

        println!("search {:?}, taken {} ms", v, watch.elapsed().as_millis());
        BrinkIndexSearchResult::None
    }
}
