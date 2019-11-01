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
    pub version: i32,
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

    pub fn put(&mut self, key: String, data: String) -> BrinkData {
        let mut data = BrinkData::new(key.clone(), 1, data);
        let mut entry = match self.keys.get(&key) {
            Some(e) => e,
            None => BrinkDataKey::new(key.clone())
        };

        data.version = entry.latest_version() + 1;

        data
    }
}

impl BrinkData {
    pub fn new(key: String, version: i32, blob: String) -> BrinkData {
        BrinkData { key, version, blob }
    }
}

impl BrinkDataKey {
    pub fn new(key: String) -> BrinkDataKey {
        let versions = LinkedList::new();
        BrinkDataKey {
            key,
            versions,
        }
    }

    pub fn put(&mut self, data_ref: BrinkDataRef) {
        self.versions.push_front(data_ref);
    }

    pub fn latest_version(&self) -> i32 {
        match self.versions.front() {
            Some(v) => v.version,
            None => 0
        }
    }
}
