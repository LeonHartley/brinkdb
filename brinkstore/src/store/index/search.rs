use crate::store::index::{BrinkIndexStore, BrinkIndexValue};
use std::time::Instant;
use std::error::Error;

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
        let values: Vec<BrinkIndexValue> = self.keys.iter()
            .filter_map(|key| {
                let index = store.values.get(&key.key);
                if let Some(index) = index {
                    if let Some(keys) = index.get(&key.value) {
                        Some(keys.clone())
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .flat_map(|m| m)
            .collect();

        println!("search {:?}, taken {} ms", values, watch.elapsed().as_millis());
        BrinkIndexSearchResult::None
    }
}
