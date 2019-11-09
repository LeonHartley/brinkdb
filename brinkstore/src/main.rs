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
use crate::store::index::{BrinkIndexStore, BrinkIndex};
use crate::store::index::search::{BrinkIndexSearch, BrinkIndexSearchKey};
use std::borrow::Borrow;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let store_name = "brinkdb".to_string();
    let mut ctx = BrinkStoreContext::new();
    let mut block = BrinkBlock::new(1).await?;

    let mut indexes = BrinkIndexStore::new();
    indexes.add(BrinkIndex {
        key: "name".into(),
        json_selector: "$.name".into(),
    });

    indexes.add(BrinkIndex {
        key: "email".into(),
        json_selector: "$.email".into(),
    });

    ctx.add_store(match BrinkStoreLoader::read(store_name.clone()).await {
        Result::Ok(store) => store,
        Result::Err(_) => BrinkStore {
            name: "brinkdb".to_string(),
            keys: HashMap::new(),
            indexes,
        }
    });

    ctx.add_block(block);
    ctx.set_default_block(1);

    ctx.put(store_name.clone(), "key-1".to_string(), "{\"name\":\"Leon\", \"email\":\"leon@test.com\"}".as_bytes().to_vec()).await?;
    ctx.put(store_name.clone(), "key-2".to_string(), "{\"name\":\"Test\", \"email\":\"test@test.com\"}".as_bytes().to_vec()).await?;
    ctx.put(store_name.clone(), "key-3".to_string(), "{\"name\":\"Test2\", \"email\":\"test2@test.com\"}".as_bytes().to_vec()).await?;

    let store = ctx.get_store(&store_name).unwrap();

    BrinkIndexSearch::new(vec! {
        BrinkIndexSearchKey::new("name".into(), "Leon".into()),
    }).search(&store.indexes);

    BrinkIndexSearch::new(vec! {
        BrinkIndexSearchKey::new("name".into(), "Test2".into()),
    }).search(&store.indexes);

    BrinkIndexSearch::new(vec! {
        BrinkIndexSearchKey::new("email".into(), "test@test.com".into()),
    }).search(&store.indexes);

    BrinkStoreLoader::write(store).await?;

    Ok(())
}
