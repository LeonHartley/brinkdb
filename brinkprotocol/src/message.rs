use brinkstore::index::search::BrinkIndexSearchKey;
use crate::codec::{BrinkEncoder, BrinkDecoder};
use uuid::Uuid;
use crate::BrinkData;

#[derive(Debug)]
pub struct BrinkTxnBeginRequest;

#[derive(Debug)]
pub struct BrinkTxnCommitRequest {
    pub txn_id: Uuid
}


#[derive(Debug)]
pub struct BrinkTxnCommitResponse {
    pub ok: bool
}

#[derive(Debug)]
pub struct BrinkTxnBeginResponse {
    pub txn_id: Uuid
}

impl BrinkDecoder for BrinkTxnBeginRequest {
    fn decode(buffer: Vec<u8>) -> Self {
        unimplemented!()
    }
}

impl BrinkEncoder for BrinkTxnBeginResponse {}

impl BrinkDecoder for BrinkTxnCommitRequest {
    fn decode(buffer: Vec<u8>) -> Self {
        unimplemented!()
    }
}

impl BrinkEncoder for BrinkTxnCommitResponse {}

#[derive(Debug)]
pub enum Command {
    Unknown,
    Get(String),
    Set { key: String, value: String },
    Delete(String),
    IndexGet(Option<String>),
    IndexSet { key: String, selector: String },
    IndexDelete(String),
    IndexSearch(Vec<BrinkIndexSearchKey>),
    Metadata,
}

#[derive(Debug)]
pub enum CommandResult {
    Value(BrinkData),
    Ok,
    None,
    Error(String),
}

#[derive(Debug)]
pub struct BrinkCommandRequest {
    pub txn_id: Option<Uuid>,
    pub store: String,
    pub command: Command,
}

#[derive(Debug)]
pub struct BrinkCommandResponse {
    pub result: CommandResult
}

impl BrinkDecoder for BrinkCommandRequest {
    fn decode(buffer: Vec<u8>) -> Self {
        unimplemented!()
    }
}

impl BrinkEncoder for BrinkCommandResponse {}
