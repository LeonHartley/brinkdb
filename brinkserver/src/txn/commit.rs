use crate::txn::TransactionChangelog;
use std::pin::Pin;
use futures::Future;
use futures::task::{Context, Poll};

pub struct Commit {
    pub changelog: TransactionChangelog,
}

#[derive(Debug)]
pub enum CommitDispatchError {
    Unknown
}

impl Future for Commit {
    type Output = Result<String, CommitDispatchError>;

    fn poll(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Self::Output> {
        println!("kek");

        Poll::Ready(Ok(String::from("yoyo")))
    }
}

pub trait CommitDispatcher {
    fn commit(&mut self, changelog: TransactionChangelog) -> Commit {
        Commit { changelog }
    }
}