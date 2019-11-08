use std::collections::{HashMap, LinkedList};
use crate::store::{BrinkStore, BrinkData};
use crate::store::block::BrinkBlock;
use std::sync::{RwLock, Arc};
use tokio::io::Error;
use std::hash::Hasher;
use std::collections::hash_map::DefaultHasher;

pub type BrinkContext = Arc<RwLock<BrinkStoreContext>>;

pub struct BrinkStoreContext {
    hasher: DefaultHasher,
    stores: HashMap<String, BrinkStore>,
    blocks: HashMap<i32, BrinkBlock>,
    default_block: Option<i32>,
}

impl BrinkStoreContext {
    pub fn new() -> BrinkStoreContext {
        BrinkStoreContext {
            hasher: DefaultHasher::new(),
            stores: HashMap::new(),
            blocks: HashMap::new(),
            default_block: None,
        }
    }

    pub async fn put(&mut self, store: String, key: String, value: Vec<u8>) -> Result<(), Error> {
        let mut store = self.stores.get_mut(&store).unwrap();
        let mut default_block = self.blocks.get_mut(&self.default_block.unwrap()).unwrap();

        store.put(key, value, &mut default_block).await
    }

    pub async fn get(&mut self, store: String, key: String) -> Result<Option<BrinkData>, Error> {
        let mut store = self.stores.get_mut(&store).unwrap();
        let mut default_block = self.blocks.get_mut(&self.default_block.unwrap()).unwrap();

        match store.get(key, &mut default_block).await? {
            Some(data) => Ok(Some(data)),
            None => Ok(None)
        }
    }

    pub fn hasher(&self) -> &DefaultHasher {
        &self.hasher
    }

    pub fn add_store(&mut self, store: BrinkStore) {
        self.stores.insert(store.name.clone(), store);
    }

    pub fn get_store_mut(&mut self, store: &String) -> Option<&mut BrinkStore> {
        self.stores.get_mut(store)
    }

    pub fn get_store(&mut self, store: &String) -> Option<&BrinkStore> {
        self.stores.get(store)
    }

    pub fn add_block(&mut self, block: BrinkBlock) {
        self.blocks.insert(block.id, block);
    }

    pub fn set_default_block(&mut self, block: i32) {
        self.default_block = Some(block)
    }
}
