use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::storage::{BrinkStore, BrinkDataRef, BrinkDataFileRef};
use brinkprotocol::codec::BrinkDecoder;
use std::io::Read;
use futures::io::Error;
use futures::Future;
use crate::blocks::BrinkBlock;

pub type BrinkStoreMap = HashMap<String, BrinkStoreRef>;
pub type BrinkBlockMap = HashMap<i32, BrinkBlockRef>;

#[derive(Clone)]
pub struct BrinkStoreRef {
    inner: Arc<Mutex<BrinkStore>>
}

#[derive(Clone)]
pub struct BrinkBlockRef {
    inner: Arc<Mutex<BrinkBlock>>
}

pub struct ServerContext {
    blocks: BrinkBlockMap,
    stores: BrinkStoreMap,
}

impl ServerContext {
    pub fn new(blocks: BrinkBlockMap, stores: BrinkStoreMap) -> Arc<ServerContext> {
        Arc::new(ServerContext { blocks, stores })
    }

    pub fn get_store(&self, store: String) -> Option<BrinkStoreRef> {
        if let Some(store) = self.stores.get(&store) {
            Some(store.clone())
        } else {
            None
        }
    }

    pub fn get_block(&self, block: i32) -> Option<BrinkBlockRef> {
        if let Some(block) = self.blocks.get(&block) {
            Some(block.clone())
        } else {
            None
        }
    }
}

impl BrinkStoreRef {
    pub async fn get(&self, key: String) -> Option<BrinkDataRef> {
        self.inner.lock().await.get(key)
    }
}

impl BrinkBlockRef {
    pub async fn read<T: BrinkDecoder>(&self, file_ref: &BrinkDataFileRef) -> Option<T> {
        match self.inner.lock().await.read(file_ref.index, file_ref.length as u64).await {
            Ok(bytes) => Some(T::decode(bytes)),
            _ => None
        }
    }
}

