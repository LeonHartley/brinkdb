use std::collections::{LinkedList, HashMap};
use tokio::sync::Mutex;

pub mod ops;
pub mod handler;
pub mod builder;

#[cfg(test)]
pub mod test;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct BrinkIndexStore {}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct BrinkDataKey {
    pub key: String,
    pub versions: LinkedList<BrinkDataRef>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct BrinkStore {
    pub name: String,
    pub keys: HashMap<String, BrinkDataKey>,
    pub indexes: BrinkIndexStore,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct BrinkDataFileRef {
    pub block: i32,
    pub index: i32,
    pub length: usize,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct BrinkDataRef {
    pub version: i32,
    pub file: Option<BrinkDataFileRef>,
    pub state: BrinkDataState,
    pub timestamp: i64,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum BrinkDataState {
    Created,
    Updated,
    Deleted,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct BrinkData {
    pub key: String,
    pub timestamp: i64,
    pub blob: Vec<u8>,
}

impl BrinkDataKey {
    pub fn new(key: String) -> BrinkDataKey {
        let versions = LinkedList::new();
        BrinkDataKey { key, versions }
    }

    pub fn put(&mut self, data_ref: BrinkDataRef) {
        self.versions.push_front(data_ref);
    }

    pub fn latest_version(&self) -> Option<&BrinkDataRef> {
        self.versions.front()
    }
}

impl BrinkStore {
    pub fn get(&self, key: String) -> Option<BrinkDataRef> {
        match self.keys.get(&key) {
            Some(key) => match key.latest_version() {
                Some(latest) => Some(latest.clone()),
                None => None
            },
            None => None
        }
    }
}
