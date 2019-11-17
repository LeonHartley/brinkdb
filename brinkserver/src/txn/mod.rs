use std::collections::VecDeque;
use brinkprotocol::message::Command;
use std::error::Error;

pub struct Transaction {
    pub id: i64,
    pub user_id: i64,
    pub changelog: Option<TransactionChangelog>,
    pub state: TransactionState,
}

pub enum TransactionState {
    Open,
    Active,
    Commit,
}

pub struct TransactionChangelog {
    pub txn_id: i64,
    pub changes: VecDeque<Command>,
}

impl Transaction {
    pub fn new(id: i64, user_id: i64) -> Transaction {
        Transaction {
            id,
            user_id,
            changelog: None,
            state: TransactionState::Open,
        }
    }

    fn command(&mut self, command: Command) {
//        let mut changelog = match &mut self.changelog {
//            Some(changelog) => changelog,
//            None => {
//                let changelog = TransactionChangelog::new();
//                self.changelog = Some(changelog);
//
//                &mut self.changelog.unwrap()
//            }
//        };
//
//        changelog.changes.push_back(command);
    }

    async fn commit(mut self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}

impl TransactionChangelog {
    pub fn new() -> TransactionChangelog {
        TransactionChangelog {
            txn_id: 0,
            changes: VecDeque::new(),
        }
    }
}