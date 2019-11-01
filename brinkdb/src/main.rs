use std::collections::{HashMap, LinkedList};
use crate::brink::{BrinkNamespace, BrinkMetadata, BrinkSystemData};
use crate::context::BrinkSystemContext;

pub mod brink;
pub mod context;

#[cfg(test)]
pub mod test;

fn main() {
    let version = "0.0.1".to_string();
    let server = "brink-01".to_string();

    let mut system = BrinkSystemData::new(BrinkMetadata {
        version,
        server,
    });

    let system_ctx = BrinkSystemContext::new(system);
}

