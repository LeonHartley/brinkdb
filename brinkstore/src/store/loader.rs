use crate::store::BrinkStore;
use tokio::fs::{read, write};
use tokio::io::Error;

pub struct BrinkStoreLoader;

impl BrinkStoreLoader {
    pub async fn write(store: &BrinkStore) -> Result<(), Error> {
        let bin = bincode::serialize(store).unwrap();

        write(format!("data/{}.brinkstore", &store.name), bin).await?;
        Ok(())
    }

    pub async fn read(name: String) -> Result<BrinkStore, Error> {
        let bytes = read(format!("data/{}.brinkstore", &name)).await?;

        Ok(bincode::deserialize(&bytes[..]).unwrap())
    }
}
