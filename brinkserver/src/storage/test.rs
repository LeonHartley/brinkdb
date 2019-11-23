use crate::storage::ops::{StoreOperationHandler, StoreOperationResult, StoreOperation, StoreBulkOperationResult};
use crate::storage::{BrinkDataRef, BrinkDataState};
use std::borrow::Borrow;

pub struct TestStore {
    on_get: fn(String) -> StoreOperationResult,
    on_set: fn(String, String) -> StoreOperationResult,
    on_delete: fn(String) -> StoreOperationResult,
}

pub struct HandlerContext;

impl StoreOperationHandler for TestStore {
    type Context = HandlerContext;

    fn get(&mut self, key: String, ctx: &mut Self::Context) -> StoreOperationResult {
        (self.on_get)(key)
    }

    fn set(&mut self, key: String, value: String, ctx: &mut Self::Context) -> StoreOperationResult {
        (self.on_set)(key, value)
    }

    fn delete(&mut self, key: String, ctx: &mut Self::Context) -> StoreOperationResult {
        (self.on_delete)(key)
    }
}

#[test]
pub fn operationhandler_handle() {
    let mut ctx = HandlerContext;
    let mut store = create_test_store();

    let result_a = store.handle(StoreOperation::Get("get-key".to_string()), &mut ctx);
    let result_b = store.handle(StoreOperation::Set { key: "set-key".to_string(), value: "set-value".to_string() }, &mut ctx);
    let result_c = store.handle(StoreOperation::Delete("delete-key".to_string()), &mut ctx);

    assert_eq!(result_a.unwrap().version, 1);
    assert_eq!(result_b.unwrap().version, 2);
    assert_eq!(result_c.unwrap().version, 3);
}

#[test]
pub fn operationhandler_handle_all() {
    let mut ctx = HandlerContext;
    let mut store = create_test_store();

    let result = store.handle_all(vec! {
        StoreOperation::Get("get-key".to_string()),
        StoreOperation::Set { key: "set-key".to_string(), value: "set-value".to_string() },
        StoreOperation::Delete("delete-key".to_string())
    }, &mut ctx);

    match result {
        StoreBulkOperationResult::Ok(r) => {
            assert_eq!(r[0].version, 1);
            assert_eq!(r[1].version, 2);
            assert_eq!(r[2].version, 3);
        }
        _ => panic!("bad result")
    }
}

#[test]
pub fn operationhandler_handle_all_unsuccessful() {
    let mut ctx = HandlerContext;
    let mut store = create_test_store();

    store.on_set = |k, v| {
        StoreOperationResult::Error
    };

    let result = store.handle_all(vec! {
        StoreOperation::Get("get-key".to_string()),
        StoreOperation::Set { key: "set-key".to_string(), value: "set-value".to_string() },
        StoreOperation::Delete("delete-key".to_string())
    }, &mut ctx);

    assert_eq!(result, StoreBulkOperationResult::Unsuccessful);
}

fn create_test_store() -> TestStore {
    TestStore {
        on_get: |key| {
            assert_eq!(key, "get-key".to_string());

            StoreOperationResult::Ok(create_data_ref(1))
        },
        on_set: |key, val| {
            assert_eq!(key, "set-key".to_string());
            assert_eq!(val, "set-value".to_string());

            StoreOperationResult::Ok(create_data_ref(2))
        },
        on_delete: |key| {
            assert_eq!(key, "delete-key".to_string());

            StoreOperationResult::Ok(create_data_ref(3))
        },
    }
}

fn create_data_ref(version: i32) -> BrinkDataRef {
    BrinkDataRef {
        version,
        file: None,
        state: BrinkDataState::Created,
        timestamp: 0,
    }
}