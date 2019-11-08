use std::collections::{HashMap, LinkedList};
use serde::{Serialize, Deserialize};
use tokio::fs::{read, write};
use tokio::io::Error;
use crate::store::block::BrinkBlock;
use std::borrow::BorrowMut;

pub mod block;
pub mod loader;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct BrinkDataKey {
    pub key: String,
    pub versions: LinkedList<BrinkDataRef>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct BrinkStore {
    pub name: String,
    pub keys: HashMap<String, BrinkDataKey>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct BrinkDataRef {
    pub store_id: i32,
    pub version: i32,
    pub index: i32,
    pub length: usize,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct BrinkData {
    pub key: String,
    pub version: i32,
    pub blob: Vec<u8>,
}

impl BrinkStore {
    pub async fn put_key(&mut self, key: String, value: String, block: &mut BrinkBlock) -> Result<(), Error> {
        let mut data = BrinkData::new(key.clone(), 1, value);
        let mut entry = match self.keys.get_mut(&key) {
            Some(mut e) => e,
            None => {
                let data_key = BrinkDataKey::new(key.clone());
                self.keys.insert(key.clone(), data_key);
                self.keys.get_mut(&key).unwrap()
            }
        };

        data.version = entry.latest_version() + 1;
        let bytes = bincode::serialize(&data).unwrap();
        let length = bytes.len();
        let index = block.write_value(bytes).await?;

        entry.versions.push_front(BrinkDataRef {
            store_id: 1,
            version: data.version,
            index,
            length,
        });

        Ok(())
    }
}

impl BrinkData {
    pub fn new(key: String, version: i32, blob: String) -> BrinkData {
        let blob = blob
            .as_bytes()
            .to_vec();

        BrinkData { key, version, blob }
    }
}

impl BrinkDataKey {
    pub fn new(key: String) -> BrinkDataKey {
        let versions = LinkedList::new();
        BrinkDataKey { key, versions }
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
