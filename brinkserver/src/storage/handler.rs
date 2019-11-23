use crate::storage::ops::{StoreOperationHandler, StoreOperationResult};
use crate::storage::BrinkStore;
use crate::server::ctx::BrinkStoreRef;

impl StoreOperationHandler for BrinkStore {
    type Context = ();

    fn get(&mut self, key: String, ctx: &mut Self::Context) -> StoreOperationResult {
        unimplemented!()
    }

    fn set(&mut self, key: String, value: String, ctx: &mut Self::Context) -> StoreOperationResult {
        unimplemented!()
    }

    fn delete(&mut self, key: String, ctx: &mut Self::Context) -> StoreOperationResult {
        unimplemented!()
    }
}

