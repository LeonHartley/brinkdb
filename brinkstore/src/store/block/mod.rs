use tokio::fs::{File, OpenOptions};
use tokio::io::{Error, AsyncWriteExt, AsyncReadExt, BufReader};
use tokio::sync::Mutex;
use std::io::SeekFrom;

const BLOCK_CACHE_SIZE: u64 = 128_000_000;

pub struct BrinkBlockFile {
    pub inner: File,
    pub writer_index: i32,
    pub cache: BrinkBlockCache,
}

pub struct BrinkBlock {
    pub id: i32,
    pub file: Mutex<BrinkBlockFile>,
}

impl BrinkBlock {
    pub async fn new(id: i32) -> Result<BrinkBlock, Error> {
        let inner = OpenOptions::new()
            .append(true)
            .create(true)
            .read(true)
            .open(format!("data/block-{}.brinkstore", id)).await?;

        let writer_index = inner.metadata().await?.len() as i32;
        let cache = BrinkBlockCache::new();

        Ok(BrinkBlock {
            id,
            file: Mutex::new(BrinkBlockFile { inner, writer_index, cache }),
        })
    }

    pub async fn write_value(&mut self, data: Vec<u8>) -> Result<i32, Error> {
        let mut file = self.file.lock().await;
        let index = file.writer_index;

        file.inner.write(data.as_slice()).await?;
        file.writer_index += data.len() as i32;

        // TODO: if bounds is within the block cache, append what we can & shift the cache block if needed

        Ok(index)
    }

    pub async fn read(&mut self, position: i32, length: u64) -> Result<Vec<u8>, Error> {
        let mut file = self.file.lock().await;

//        println!("current writer index, {}, reading from {} to {}", file.writer_index, position, length);

        // TODO: use block cache
        file.inner.seek(SeekFrom::Start(position as u64)).await?;
        let mut contents = vec![0u8; length as usize];
        file.inner.read_exact(&mut contents).await?;

        Ok(contents)
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
