use std::collections::{HashMap, LinkedList};
use crate::store::BrinkStore;
use crate::store::block::BrinkBlock;
use std::sync::{RwLock, Arc};
use tokio::io::Error;

pub type BrinkContext = Arc<RwLock<BrinkStoreContext>>;

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

    pub async fn put_key(&mut self, store: String, key: String, value: String) -> Result<(), Error> {
        let mut store = self.stores.get_mut(&store).unwrap();
        let mut default_block = self.blocks.get_mut(&self.default_block.unwrap()).unwrap();

        store.put_key(key, value, &mut default_block).await
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
