use std::collections::{VecDeque, HashMap};
use brinkprotocol::message::Command;
use std::error::Error;
use uuid::Uuid;
use crate::txn::commit::{CommitDispatcher, CommitDispatchError};
use std::sync::RwLockWriteGuard;
use tokio::sync::MutexGuard;
use crate::storage::{BrinkDataKey, BrinkData};

pub mod commit;

pub struct TransactionKeyStore {
    keys: HashMap<String, BrinkData>
}

pub struct Transaction {
    pub id: Uuid,
    pub user_id: i64,
    pub changelog: Option<TransactionChangelog>,
    pub state: TransactionState,
    pub stores: HashMap<String, TransactionKeyStore>,
}

pub enum TransactionState {
    Open,
    Active,
    Commit,
}

pub struct TransactionChangelog {
    pub txn_id: Uuid,
    pub changes: VecDeque<Command>,
}

pub enum TransactionError {
    Unknown,
    CommitError(CommitDispatchError),
}

impl Transaction {
    pub fn new() -> Transaction {
        let id = Uuid::new_v4();
        Transaction {
            id,
            user_id: 0,
            changelog: None,
            state: TransactionState::Open,
            stores: HashMap::new(),
        }
    }

    async fn command(&mut self, command: Command) {
        match command {
            _ => {}
        };

        let mut changelog = match &mut self.changelog {
            Some(changelog) => changelog,
            None => {
                let changelog = TransactionChangelog::new(self.id.clone());
                self.changelog = Some(changelog);

                self.changelog.as_mut().unwrap()
            }
        };

        changelog.changes.push_back(command);
    }

    async fn commit<Dispatcher>(mut self, dispatcher: &mut Dispatcher) -> Result<(), TransactionError>
        where Dispatcher: CommitDispatcher {
        match self.changelog {
            Some(mut changelog) => match dispatcher.commit(changelog).await {
                Ok(res) => Ok(()),
                Err(e) => Err(TransactionError::CommitError(e))
            },
            None => Ok(())
        }
    }
}

impl TransactionChangelog {
    pub fn new(txn_id: Uuid) -> TransactionChangelog {
        TransactionChangelog { txn_id, changes: VecDeque::new() }
    }

    pub fn count(&self) -> usize { self.changes.len() }
}


#[cfg(test)]
pub mod test {
    use crate::txn::{Transaction, TransactionChangelog};
    use crate::txn::commit::{CommitDispatcher, CommitDispatchError, Commit};
    use brinkprotocol::message::Command;
    use std::collections::VecDeque;
    use std::sync::Mutex;

    pub struct TestCommitDispatcher {
        committed_commands: VecDeque<Command>
    }

    impl TestCommitDispatcher {
        pub fn new() -> TestCommitDispatcher { TestCommitDispatcher { committed_commands: VecDeque::new() } }
    }

    impl CommitDispatcher for TestCommitDispatcher {
        fn commit(&mut self, changelog: TransactionChangelog) -> Commit {
            Commit { changelog }
        }
    }

    #[test]
    pub fn transaction_changelog_add_cmd() {
        let cmd = create_set_cmd();
        let mut transaction = Transaction::new();

        transaction.command(cmd);

        let changelog = transaction.changelog.unwrap();

        assert_eq!(changelog.changes.len(), 1);
    }

    #[test]
    pub fn transaction_changelog_commit() {
        let cmd = create_set_cmd();

        let mut transaction = Transaction::new();

        transaction.command(cmd);

        let cmd_count = transaction.changelog
            .as_ref()
            .unwrap()
            .count();

        let mut dispatcher = TestCommitDispatcher::new();

        transaction.commit(&mut dispatcher);

        assert_eq!(cmd_count, dispatcher.committed_commands.len());
    }


    pub fn create_set_cmd() -> Command {
        Command::Set { key: "key".to_string(), value: "value".to_string() }
    }
}