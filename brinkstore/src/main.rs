pub(crate) mod ctx;
pub(crate) mod store;

#[cfg(test)]
pub mod test;

#[macro_use]
extern crate serde;
extern crate bincode;
extern crate tokio;
extern crate uuid;

#[macro_use]
extern crate async_trait;

use std::error::Error;
use std::collections::HashMap;
use tokio::fs::OpenOptions;
use crate::store::BrinkStore;
use crate::store::block::BrinkBlock;
use crate::ctx::BrinkStoreContext;
use crate::store::loader::BrinkStoreLoader;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut ctx = BrinkStoreContext::new();
    let mut store = BrinkStore {
        name: "brinkdb".to_string(),
        keys: HashMap::new(),
    };

    let block_file = OpenOptions::new()
        .append(true)
        .create(true)
        .read(true)
        .open("block-1.brinkdb").await?;

    let metadata = block_file.metadata().await?;
    let mut block = BrinkBlock::new(1, block_file);

    block.writer_index = metadata.len() as i32;

    let store_name = "brinkdb".to_string();
    let val = "test".as_bytes().to_vec();
    let allocated_index = block.write_value(val).await?;

    ctx.add_block(block);
    ctx.add_store(store);

    ctx.set_default_block(1);

    println!("index allocated: {}", allocated_index);

    ctx.put_key(store_name.clone(), "key-1".to_string(), "value".to_string()).await?;
    let store = ctx.get_store_mut(&store_name).unwrap();
    BrinkStoreLoader::write(store).await?;
    Ok(())
}
