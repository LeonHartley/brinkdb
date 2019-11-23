use crate::session::{MessageHandler, Session};
use brinkprotocol::message::{BrinkCommandRequest, Command, CommandResult, BrinkCommandResponse};
use crate::server::ctx::{ServerContext, BrinkStoreRef};
use std::future::Future;
use std::error::Error;
use std::sync::Arc;
use brinkprotocol::BrinkData;
use crate::txn::Transaction;
use uuid::Uuid;

impl MessageHandler<BrinkCommandRequest, BrinkCommandResponse> for Session {
    fn handle(&mut self, message: BrinkCommandRequest, ctx: Arc<ServerContext>) -> Box<dyn Future<Output=BrinkCommandResponse>> {
        let txn = match message.txn_id {
            Some(txn_id) => match self.transactions.get(&txn_id) {
                Some(txn) => Some(txn.clone()),
                None => None
            },
            None => None
        };

        let fut: Box<dyn Future<Output=BrinkCommandResponse>> = match ctx.get_store(message.store) {
            Some(store) => Box::new(handle_command_request(message.command, store, ctx, txn)),
            None => Box::new(error_store_not_found())
        };

        fut
    }
}

async fn handle_command_request(cmd: Command, store: BrinkStoreRef, ctx: Arc<ServerContext>, txn: Option<Arc<Transaction>>) -> BrinkCommandResponse {
    let result = match cmd {
        Command::Get(key) => get(key, store, ctx, txn).await,
        Command::Set { key, value } => set(key, value, store, ctx, txn).await,
        Command::Delete(key) => delete(key, store, ctx, txn).await,
        _ => CommandResult::None,
    };

    BrinkCommandResponse { result }
}

async fn get(key: String, store: BrinkStoreRef, ctx: Arc<ServerContext>, txn: Option<Arc<Transaction>>) -> CommandResult {
    match store.get(key).await {
        Some(data_ref) => {
            let file = data_ref.file.expect("file");
            let block = ctx.get_block(file.block).expect("block");

            match block.read(&file).await {
                Some(data) => CommandResult::Value(data),
                None => CommandResult::None
            }
        }
        None => CommandResult::None
    }
}

async fn set(key: String, value: String, store: BrinkStoreRef, ctx: Arc<ServerContext>, txn: Option<Arc<Transaction>>) -> CommandResult {
    CommandResult::None
}

async fn delete(key: String, store: BrinkStoreRef, ctx: Arc<ServerContext>, txn: Option<Arc<Transaction>>) -> CommandResult {
    CommandResult::None
}

async fn error_store_not_found() -> BrinkCommandResponse {
    BrinkCommandResponse { result: CommandResult::Error("Store not found".to_string()) }
}
