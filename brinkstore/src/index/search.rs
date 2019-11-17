use std::time::Instant;

use std::collections::HashMap;
use crate::index::{BrinkIndexStore, BrinkIndexValue};

#[derive(Serialize, Deserialize, Debug)]
pub struct BrinkIndexSearchKey {
    key: String,
    value: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BrinkIndexSearch {
    keys: Vec<BrinkIndexSearchKey>
}

#[derive(Serialize, Deserialize, Debug)]
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
        let mut results: HashMap<(&String, i32), i32> = HashMap::new();
        let keys: Vec<(&String, i32)> = self.keys.iter()
            .filter_map(|key| {
                let index = store.values.get(&key.key);
                if let Some(index) = index {
                    if let Some(keys) = index.get(&key.value) {
                        Some(keys
                            .iter()
                            .map(|f| (&f.key, f.version))
                            .collect())
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .flat_map(|m: Vec<(&String, i32)>| m)
            .collect();

        for key in keys {
            let count = if let Some(count) = results.get(&key) {
                *count + 1
            } else {
                1
            };

            results.insert(key,count);
        }

        let v: Vec<(String, i32)> = results.into_iter().filter_map(|((key, version), val)| {
            if val as usize == self.keys.len() {
                Some((key.clone(), version))
            } else {
                None
            }
        }).collect();

        println!("search {:?}, taken {} ms", v, watch.elapsed().as_millis());
        BrinkIndexSearchResult::None
    }
}
