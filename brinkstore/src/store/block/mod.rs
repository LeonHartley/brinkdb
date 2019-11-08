use tokio::fs::File;
use tokio::io::{Error, AsyncWriteExt};

pub struct BrinkBlock {
    pub id: i32,
    pub file: File,
    pub writer_index: i32,
    pub cache: BrinkBlockCache,
}

impl BrinkBlock {
    pub fn new(id: i32, file: File) -> BrinkBlock {
        BrinkBlock {
            id,
            file,
            writer_index: 0,
            cache: BrinkBlockCache::new(),
        }
    }

    pub async fn write_value(&mut self, data: Vec<u8>) -> Result<i32, Error> {
        let index = self.writer_index;

        self.file.write(data.as_slice()).await?;
        self.writer_index += data.len() as i32;

        Ok(index)
    }

    pub async fn read(&mut self, position: i32, length: i32) -> Result<Vec<u8>, Error> {
        Ok(vec![])
    }
}


pub struct BrinkBlockCache {
    pub position: i32,
    pub length: i32,
    pub block: Vec<u8>,
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
