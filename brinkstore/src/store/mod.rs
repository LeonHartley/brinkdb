use std::collections::{HashMap, LinkedList};
use serde::{Serialize, Deserialize};
use tokio::fs::{read, write};
use tokio::io::Error;
use crate::store::block::BrinkBlock;
use std::borrow::BorrowMut;
use std::collections::hash_map::DefaultHasher;
use crypto::sha1::Sha1;
use crypto::digest::Digest;

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
    pub async fn put(&mut self, key: String, value: Vec<u8>, block: &mut BrinkBlock) -> Result<(), Error> {
        if let Some(data) = self.get(key.clone(), block).await? {
            if data.blob == value {
                return Ok(());
            }
        }

        let mut data = BrinkData::new(key.clone(), 1, value);
        let mut entry = match self.keys.get_mut(&key) {
            Some(mut e) => e,
            None => {
                let data_key = BrinkDataKey::new(key.clone());
                self.keys.insert(key.clone(), data_key);
                self.keys.get_mut(&key).unwrap()
            }
        };

        data.version = match entry.latest_version() {
            Some(latest) => {
                data.version + 1
            }
            None => data.version
        };

        let bytes = bincode::serialize(&data).unwrap();
        let length = bytes.len();
        let index = block.write_value(bytes).await?;

        entry.put(BrinkDataRef {
            store_id: 1,
            version: data.version,
            index,
            length,
        });

        Ok(())
    }

    pub async fn get(&mut self, key: String, block: &mut BrinkBlock) -> Result<Option<BrinkData>, Error> {
        let version = match self.keys.get_mut(&key) {
            Some(e) => match e.latest_version() {
                Some(v) => v,
                None => return Ok(None)
            },
            None => return Ok(None)
        };

        let res = block.read(version.index, version.length as u64).await?;
        let x = bincode::deserialize(&res[..]).unwrap();
        Ok(Some(x))
    }
}

impl BrinkData {
    pub fn new(key: String, version: i32, blob: Vec<u8>) -> BrinkData {
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

    pub fn latest_version(&self) -> Option<&BrinkDataRef> {
        self.versions.front()
    }
}
