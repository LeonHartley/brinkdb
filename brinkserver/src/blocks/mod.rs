use std::collections::HashMap;
use tokio::fs::File;
use tokio::sync::Mutex;

pub mod io;

pub struct BrinkBlockCache {
    pub position: i32,
    pub length: i32,
    pub block: Vec<u8>,
}

pub struct BrinkBlockFile {
    pub inner: File,
    pub writer_index: i32,
    pub cache: BrinkBlockCache,
}

pub struct BrinkBlock {
    pub id: i32,
    pub file: BrinkBlockFile,
}

impl BrinkBlockCache {
    pub fn new() -> BrinkBlockCache {
        BrinkBlockCache {
            position: 0,
            length: 0,
            block: vec![],
        }
    }
}