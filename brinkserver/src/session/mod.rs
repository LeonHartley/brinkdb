use crate::user::User;
use crate::txn::Transaction;
use brinkprotocol::codec::{BrinkDecoder, BrinkEncoder};
use brinkprotocol::message::{BrinkCommandRequest, CommandResult};
use uuid::Uuid;
use std::collections::HashMap;
use crate::server::ctx::ServerContext;
use futures::Future;
use std::sync::Arc;

pub mod command;
pub mod transaction;

pub struct Session {
    id: Uuid,
    user: Option<User>,
    transactions: HashMap<Uuid, Arc<Transaction>>,
}

impl Session {
    pub fn new() -> Session {
        let id = Uuid::new_v4();
        let user = None;
        let transactions = HashMap::new();

        Session { id, user, transactions }
    }
}

pub trait MessageHandler<Req, Res> where Req: BrinkDecoder, Res: BrinkEncoder {
    fn handle(&mut self, req: Req, context: Arc<ServerContext>) -> Box<dyn Future<Output=Res>>;
}