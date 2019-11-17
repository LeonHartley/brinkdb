use brinkstore::ctx::BrinkStoreContext;
use std::error::Error;
use brinkstore::store::util::IsJson;
use serde_json::Value;
use brinkstore::index::search::{BrinkIndexSearchKey, BrinkIndexSearch};
use brinkprotocol::message::Command;

pub async fn handle_command(store: String, command: Command, ctx: &mut BrinkStoreContext) -> Result<(), Box<dyn Error>> {
    match command {
        Command::Get(key) => handle_get(store, key, ctx).await,
        Command::Set { key, value } => handle_set(store, key, value, ctx).await,
        Command::Delete(key) => handle_delete(store, key, ctx).await,
        Command::Metadata => handle_metadata(store, ctx).await,
        Command::IndexGet(key) => handle_index_get(store, key, ctx).await,
        Command::IndexSet { key, selector } => handle_index_set(store, key, selector, ctx).await,
        Command::IndexDelete(key) => handle_index_delete(store, key, ctx).await,
        Command::IndexSearch(keys) => handle_index_search(store, keys, ctx).await,
        Command::Unknown => {
            println!("unknown command");

            Ok(())
        }
    }
}

pub async fn handle_get(store: String, key: String, ctx: &mut BrinkStoreContext) -> Result<(), Box<dyn Error>> {
    let result = {
        let value = ctx.get(store, key.clone()).await?.unwrap();
        let s = &String::from_utf8(value.blob.clone()).unwrap();
        if value.blob.is_json() {
            if let Ok(v) = serde_json::from_str::<Value>(s) {
                format!("{} v{}\n{}", &key, value.version, serde_json::to_string_pretty(&v).unwrap())
            } else {
                format!("{} v{}\n{}", &key, value.version, s)
            }
        } else {
            format!("{} v{}\n{}", &key, value.version, s)
        }
    };

    println!("\n{}", result);
    Ok(())
}

pub async fn handle_set(store: String, key: String, value: String, ctx: &mut BrinkStoreContext) -> Result<(), Box<dyn Error>> {
    ctx.put(store, key, value.as_bytes().to_vec()).await?;

    Ok(())
}

pub async fn handle_metadata(_store: String, _ctx: &mut BrinkStoreContext) -> Result<(), Box<dyn Error>> {
    Ok(())
}


pub async fn handle_delete(store: String, key: String, ctx: &mut BrinkStoreContext) -> Result<(), Box<dyn Error>> {
    ctx.del(store, key).await?;

    Ok(())
}

pub async fn handle_index_get(store: String, key: Option<String>, ctx: &mut BrinkStoreContext) -> Result<(), Box<dyn Error>> {
    match key.as_ref() {
        Some(key) => {
            let store = ctx.get_store(&store).unwrap();

            println!("{}", serde_json::to_string_pretty(store.indexes.indexes.get(key).unwrap()).unwrap());
        }
        None => {
            println!("{}", serde_json::to_string_pretty(&ctx.index_metadata(&store).unwrap()).unwrap());
        }
    }
    Ok(())
}

pub async fn handle_index_set(_store: String, _key: String, _selector: String, _ctx: &mut BrinkStoreContext) -> Result<(), Box<dyn Error>> {
    Ok(())
}

pub async fn handle_index_delete(_store: String, _key: String, _ctx: &mut BrinkStoreContext) -> Result<(), Box<dyn Error>> {
    Ok(())
}

pub async fn handle_index_search(store: String, keys: Vec<BrinkIndexSearchKey>, ctx: &mut BrinkStoreContext) -> Result<(), Box<dyn Error>> {
    let store = ctx.get_store(&store).unwrap();

    let _res = BrinkIndexSearch::new(keys).search(&store.indexes);

    Ok(())
}