use brinkstore::ctx::BrinkStoreContext;
use std::error::Error;
use brinkstore::store::util::IsJson;
use serde_json::Value;

#[derive(Debug)]
pub enum Command {
    Unknown,
    Get(String),
    Set { key: String, value: String },
    Delete(String),
    IndexGet(String),
    IndexSet { key: String, selector: String },
    IndexDelete(String),
    Metadata,
}

pub async fn handle_command(store: String, command: Command, ctx: &mut BrinkStoreContext) -> Result<(), Box<dyn Error>> {
    match command {
        Command::Get(key) => handle_get(store, key, ctx).await,
        Command::Set { key, value } => handle_set(store, key, value, ctx).await,
        Command::Delete(key) => handle_delete(store, key, ctx).await,
        Command::Metadata => handle_metadata(store, ctx).await,
        Command::Unknown => {
            println!("unknown command");

            Ok(())
        },
        Command::IndexGet(_) => Ok(()),
        Command::IndexSet { key: _, selector: _ } => Ok(()),
        Command::IndexDelete(_) => Ok(()),
    }
}

pub async fn handle_get(store: String, key: String, ctx: &mut BrinkStoreContext) -> Result<(), Box<dyn Error>> {
    let result = match key.as_ref() {
        "indexes" => {
            let store = ctx.get_store(&store).unwrap();

            serde_json::to_string_pretty(&store.indexes.indexes).unwrap()
        }
        _ => {
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
        }
    };

    println!("\n{}", result);
    Ok(())
}

pub async fn handle_set(store: String, key: String, value: String, ctx: &mut BrinkStoreContext) -> Result<(), Box<dyn Error>> {
    ctx.put(store, key, value.as_bytes().to_vec()).await?;

    Ok(())
}

pub async fn handle_metadata(store: String, ctx: &mut BrinkStoreContext) -> Result<(), Box<dyn Error>> {
    Ok(())
}


pub async fn handle_delete(store: String, key: String, ctx: &mut BrinkStoreContext) -> Result<(), Box<dyn Error>> {
    ctx.del(store, key).await?;

    Ok(())
}
