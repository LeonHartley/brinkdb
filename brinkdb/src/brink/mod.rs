pub mod store;

use std::collections::{LinkedList, HashMap};
use std::iter::*;
use store::BrinkStore;

pub struct BrinkSystemData {
    metadata: BrinkMetadata,
    namespaces: HashMap<i32, BrinkNamespace>,
    namespace_ids: HashMap<String, i32>,
    stores: HashMap<i32, BrinkStore>,
}

pub struct BrinkNamespace {
    pub id: i32,
    pub namespace: String,
    pub store_id: i32,
}

pub struct BrinkMetadata {
    pub version: String,
    pub server: String,
}

impl BrinkSystemData {
    pub fn new(metadata: BrinkMetadata) -> BrinkSystemData {
        let mut data = BrinkSystemData {
            metadata,
            namespaces: HashMap::new(),
            namespace_ids: HashMap::new(),
            stores: HashMap::new(),
        };

        data.add_namespace(BrinkNamespace::new(1, "brink".to_string(), 1, 0));

        data
    }

    pub fn add_namespace(&mut self, namespace: BrinkNamespace) {
        self.namespace_ids.insert(namespace.namespace.clone(), namespace.id);
        self.namespaces.insert(namespace.id, namespace);
    }

    pub fn active_namespaces(&mut self) -> Vec<String> {
        self.namespace_ids.keys()
            .map(|k| k.clone())
            .collect()
    }
}

impl BrinkNamespace {
    pub fn new(id: i32, namespace: String, store_id: i32, keys_index: i32) -> BrinkNamespace {
        let namespace = namespace.into();

        BrinkNamespace {
            id,
            namespace,
            store_id,
        }
    }
}
