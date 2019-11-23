use std::error::Error;
use std::future::Future;
use crate::txn::commit::Commit;
use crate::txn::TransactionChangelog;
use uuid::Uuid;

#[macro_use]
extern crate serde;
extern crate bincode;
extern crate tokio;
extern crate uuid;
extern crate serde_json;
extern crate jsonpath;
extern crate chrono;
extern crate futures;
extern crate async_std;

pub mod txn;
pub mod user;
pub mod server;
pub mod blocks;
pub mod storage;
pub mod session;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");

    let commit = Commit { changelog: TransactionChangelog::new(Uuid::new_v4()) };

    let s = commit.await;
    println!("{:?}", s);
    Ok(())
}