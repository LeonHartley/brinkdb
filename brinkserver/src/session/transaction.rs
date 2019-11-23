use brinkprotocol::message::{BrinkTxnBeginRequest, BrinkTxnBeginResponse, BrinkTxnCommitRequest, BrinkTxnCommitResponse};
use crate::session::{MessageHandler, Session};
use crate::server::ctx::ServerContext;
use crate::txn::Transaction;
use std::sync::Arc;
use futures::Future;
use uuid::Uuid;

impl MessageHandler<BrinkTxnBeginRequest, BrinkTxnBeginResponse> for Session {
    fn handle(&mut self, req: BrinkTxnBeginRequest, ctx: Arc<ServerContext>) -> Box<dyn Future<Output=BrinkTxnBeginResponse>> {
        let transaction = Transaction::new();
        let txn_id = transaction.id.clone();

        self.transactions.insert(transaction.id, Arc::new(transaction));

        Box::new(begin_transaction(txn_id))
    }
}

impl MessageHandler<BrinkTxnCommitRequest, BrinkTxnCommitResponse> for Session {
    fn handle(&mut self, req: BrinkTxnCommitRequest, ctx: Arc<ServerContext>) -> Box<dyn Future<Output=BrinkTxnCommitResponse>> {
        let txn = match self.transactions.get(&req.txn_id) {
            Some(txn) => Some(txn.clone()),
            None => None
        };

        Box::new(commit_transaction(txn, ctx))
    }
}

async fn begin_transaction(txn_id: Uuid) -> BrinkTxnBeginResponse {
    BrinkTxnBeginResponse { txn_id }
}

async fn commit_transaction(txn: Option<Arc<Transaction>>, ctx: Arc<ServerContext>) -> BrinkTxnCommitResponse {
    match txn {
        Some(txn) => {
            BrinkTxnCommitResponse { ok: true }
        }
        None => BrinkTxnCommitResponse { ok: false }
    }
}