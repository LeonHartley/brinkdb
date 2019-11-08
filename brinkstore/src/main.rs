pub(crate) mod ctx;
pub(crate) mod store;

#[cfg(test)]
pub mod test;

#[macro_use]
extern crate serde;
extern crate bincode;
extern crate tokio;
extern crate uuid;
extern crate serde_json;
extern crate jsonpath;

#[macro_use]
extern crate async_trait;

use std::error::Error;
use std::collections::HashMap;
use tokio::fs::OpenOptions;
use crate::store::BrinkStore;
use crate::store::block::BrinkBlock;
use crate::ctx::BrinkStoreContext;
use crate::store::loader::BrinkStoreLoader;
use crate::store::index::BrinkIndexStore;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let store_name = "brinkdb".to_string();
    let mut ctx = BrinkStoreContext::new();
    let mut block = BrinkBlock::new(1).await?;

    ctx.add_store(match BrinkStoreLoader::read(store_name.clone()).await {
        Result::Ok(store) => store,
        Result::Err(_) => BrinkStore {
            name: "brinkdb".to_string(),
            keys: HashMap::new(),
            indexes: BrinkIndexStore::new()
        }
    });

    ctx.add_block(block);
    ctx.set_default_block(1);

    ctx.put(store_name.clone(), "key-1".to_string(), "{\"name\":\"Leon\"}".as_bytes().to_vec()).await?;

    let data = match ctx.get(store_name.clone(), "key-1".to_string()).await? {
        Some(data) => data,
        None => panic!("no key found")
    };

    println!("version: {}, value: {}", data.version, String::from_utf8(data.blob).unwrap());
    let store = ctx.get_store_mut(&store_name).unwrap();
    BrinkStoreLoader::write(store).await?;

    Ok(())
}
