use brinkstore::index::search::BrinkIndexSearchKey;

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

pub struct BrinkRequest {
    txn_id: Option<i64>,
    command: Command,
}

impl BrinkRequest {
    pub fn new(txn_id: Option<i64>, command: Command) -> BrinkRequest {
        BrinkRequest { txn_id, command }
    }
}