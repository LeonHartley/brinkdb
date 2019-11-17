pub mod block;
pub mod loader;
pub mod index;
pub mod util;

pub mod ctx;

#[cfg(test)]
pub mod test;

#[macro_use]
extern crate serde;
extern crate bincode;
extern crate tokio;
extern crate uuid;
extern crate serde_json;
extern crate jsonpath;
extern crate chrono;

use std::io::Error;
use std::collections::{HashMap, LinkedList};
use serde::{Serialize, Deserialize};
use crate::block::BrinkBlock;
use crate::index::{BrinkIndex, BrinkIndexStore};
use crate::util::IsJson;
use crate::index::parser::BrinkIndexParser;
use chrono::prelude::*;

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
pub enum BrinkDataState {
    Created,
    Updated,
    Deleted,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct BrinkDataRef {
    pub version: i32,
    pub index: i32,
    pub length: usize,
    pub state: BrinkDataState,
    pub timestamp: i64,
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
        let entry = match self.keys.get_mut(&key) {
            Some(e) => e,
            None => {
                let data_key = BrinkDataKey::new(key.clone());
                self.keys.insert(key.clone(), data_key);
                self.keys.get_mut(&key).unwrap()
            }
        };

        data.version = match entry.latest_version() {
            Some(latest) => latest.version + 1,
            None => data.version
        };

        let bytes = bincode::serialize(&data).unwrap();
        let length = bytes.len();
        let index = block.write_value(bytes).await?;

        if data.blob.is_json() {
            BrinkIndex::parse(&key, String::from_utf8(data.blob.clone()).unwrap(), data.version, &mut self.indexes);
        }

        let state = if data.version == 1 {
            BrinkDataState::Created
        } else {
            BrinkDataState::Updated
        };

        let timestamp = Utc::now().timestamp();
        entry.put(BrinkDataRef {
            version: data.version,
            state,
            index,
            length,
            timestamp,
        });

        Ok(())
    }

    pub async fn get(&self, key: String, block: &mut BrinkBlock) -> Result<Option<BrinkData>, Error> {
        let version = match self.keys.get(&key) {
            Some(e) => match e.latest_version() {
                Some(v) => v,
                None => return Ok(None)
            },
            None => return Ok(None)
        };

        if version.state == BrinkDataState::Deleted {
            return Ok(None);
        }

        let res = block.read(version.index, version.length as u64).await?;
        let x = bincode::deserialize(&res[..]).unwrap();

        Ok(Some(x))
    }

    pub async fn del(&mut self, key: String) -> Result<(), Error> {
        if let Some(entry) = self.keys.get_mut(&key) {
            if let Some(latest) = entry.latest_version() {
                if latest.state == BrinkDataState::Deleted {
                    return Ok(());
                }

                let mut new = latest.clone();
                new.state = BrinkDataState::Deleted;
                new.version = new.version + 1;
                new.timestamp = Utc::now().timestamp();

                entry.put(new);
            }
        }

        Ok(())
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