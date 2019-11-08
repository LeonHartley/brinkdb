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
    let store_name = "brinkdb".to_string();
    let mut ctx = BrinkStoreContext::new();
    let mut block = BrinkBlock::new(1).await?;
    ctx.add_store(BrinkStoreLoader::read(store_name.clone()).await?);

    ctx.add_block(block);
    ctx.set_default_block(1);

    ctx.put(store_name.clone(), "key-1".to_string(), "this is a value alalala".as_bytes().to_vec()).await?;

    println!("{:?}", ctx.get(store_name.clone(), "key-1".to_string()).await?);
    let store = ctx.get_store_mut(&store_name).unwrap();
    BrinkStoreLoader::write(store).await?;

    Ok(())
}
