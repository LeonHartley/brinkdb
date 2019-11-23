use crate::blocks::{BrinkBlock, BrinkBlockCache, BrinkBlockFile};
use tokio::fs::OpenOptions;
use std::io::Error;
use futures::io::SeekFrom;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::Mutex;

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
            file: BrinkBlockFile { inner, writer_index, cache },
        })
    }

    pub async fn write_value(&mut self, data: Vec<u8>) -> Result<i32, Error> {
        let index = self.file.writer_index;

        self.file.inner.write(data.as_slice()).await?;
        self.file.writer_index += data.len() as i32;

        Ok(index)
    }

    pub async fn read(&mut self, position: i32, length: u64) -> Result<Vec<u8>, Error> {
        self.file.inner.seek(SeekFrom::Start(position as u64)).await?;

        let mut contents = vec![0u8; length as usize];
        self.file.inner.read_exact(&mut contents).await?;

        Ok(contents)
    }
}
