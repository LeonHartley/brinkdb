use crate::storage::{BrinkDataRef, BrinkStore};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum StoreOperation {
    Get(String),
    Set { key: String, value: String },
    Delete(String),
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum StoreOperationResult {
    Ok(BrinkDataRef),
    Error,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum StoreBulkOperationResult {
    Ok(Vec<BrinkDataRef>),
    Unsuccessful,
}

impl StoreOperationResult {
    pub fn unwrap(self) -> BrinkDataRef {
        match self {
            StoreOperationResult::Ok(d) => d,
            _ => panic!("expected BrinkDataRef")
        }
    }

    pub fn is_ok(&self) -> bool {
        match self {
            StoreOperationResult::Ok(_) => true,
            _ => false
        }
    }
}

pub trait StoreOperationHandler {
    type Context;

    fn handle(&mut self, op: StoreOperation, ctx: &mut Self::Context) -> StoreOperationResult {
        match op {
            StoreOperation::Get(key) => self.get(key, ctx),
            StoreOperation::Set { key, value } => self.set(key, value, ctx),
            StoreOperation::Delete(key) => self.delete(key, ctx)
        }
    }

    fn handle_all(&mut self, ops: Vec<StoreOperation>, ctx: &mut Self::Context) -> StoreBulkOperationResult {
        let count = ops.len();
        let successful_res: Vec<BrinkDataRef> = ops
            .into_iter()
            .filter_map(|op| match self.handle(op, ctx) {
                StoreOperationResult::Ok(data) => Some(data),
                _ => None
            })
            .collect();

        if successful_res.len() != count {
            StoreBulkOperationResult::Unsuccessful
        } else {
            StoreBulkOperationResult::Ok(successful_res)
        }
    }

    fn get(&mut self, key: String, ctx: &mut Self::Context) -> StoreOperationResult;

    fn set(&mut self, key: String, value: String, ctx: &mut Self::Context) -> StoreOperationResult;

    fn delete(&mut self, key: String, ctx: &mut Self::Context) -> StoreOperationResult;
}