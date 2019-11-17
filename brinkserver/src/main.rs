use std::error::Error;
use std::future::Future;

extern crate serde;
extern crate bincode;
extern crate tokio;
extern crate uuid;
extern crate serde_json;
extern crate jsonpath;
extern crate chrono;

pub mod txn;
pub mod user;
pub mod server;
pub mod blocks;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");

    Ok(())
}