use crate::store::index::{BrinkIndexStore, BrinkIndexValue};

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
        let values: Vec<Vec<BrinkIndexValue>> = self.keys.iter()
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
            .collect();

        println!("{:?}", values);
        BrinkIndexSearchResult::None
    }
}
