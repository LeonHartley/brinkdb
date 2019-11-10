use std::collections::{HashMap, LinkedList};
use crate::store::{BrinkStore, BrinkData};
use crate::store::block::BrinkBlock;
use std::sync::{RwLock, Arc};
use tokio::io::Error;
use std::hash::Hasher;
use std::collections::hash_map::DefaultHasher;
use std::time::Instant;
use crate::store::index::BrinkIndex;

pub struct BrinkStoreContext {
    stores: HashMap<String, BrinkStore>,
    blocks: HashMap<i32, BrinkBlock>,
    default_block: Option<i32>,
}

impl BrinkStoreContext {
    pub fn new() -> BrinkStoreContext {
        BrinkStoreContext {
            stores: HashMap::new(),
            blocks: HashMap::new(),
            default_block: None,
        }
    }

    pub async fn put(&mut self, store: String, key: String, value: Vec<u8>) -> Result<(), Error> {
        let watch = Instant::now();

        let mut store = self.stores.get_mut(&store).unwrap();
        let mut default_block = self.blocks.get_mut(&self.default_block.unwrap()).unwrap();

        let res = store.put(key, value, &mut default_block).await;
//        println!("set value took {} ms", watch.elapsed().as_millis());

        res
    }

    pub async fn get(&mut self, store: String, key: String) -> Result<Option<BrinkData>, Error> {
        let watch = Instant::now();

        let store = self.stores.get(&store).unwrap();
        let mut default_block = self.blocks.get_mut(&self.default_block.unwrap()).unwrap();

        let result = match store.get(key, &mut default_block).await? {
            Some(data) => Ok(Some(data)),
            None => Ok(None)
        };

//        println!("get value took {} ms", watch.elapsed().as_millis());
        result
    }

    pub async fn del(&mut self, store: String, key: String) -> Result<(), Error> {
        let watch = Instant::now();

        let mut store = self.stores.get_mut(&store).unwrap();
        let mut default_block = self.blocks.get_mut(&self.default_block.unwrap()).unwrap();

        store.del(key).await
    }

    pub fn add_store(&mut self, store: BrinkStore) {
        self.stores.insert(store.name.clone(), store);
    }

    pub fn get_store_mut(&mut self, store: &String) -> Option<&mut BrinkStore> {
        self.stores.get_mut(store)
    }

    pub fn get_store(&self, store: &String) -> Option<&BrinkStore> {
        self.stores.get(store)
    }

    pub fn add_block(&mut self, block: BrinkBlock) {
        self.blocks.insert(block.id, block);
    }

    pub fn set_default_block(&mut self, block: i32) {
        self.default_block = Some(block)
    }

    pub fn index_metadata(&self, store: &String) -> Option<Vec<BrinkIndex>> {
        match self.get_store(store) {
            Some(store) => Some(store.indexes.indexes
                .values()
                .map(|f| f.clone())
                .collect()),

            None => None
        }
    }
}
