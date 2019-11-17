use std::sync::Arc;
use crate::blocks::ServerBlockStore;

pub struct ServerContext {
    blocks: Arc<ServerBlockStore>,
}

impl ServerContext {
    pub fn new(blocks: Arc<ServerBlockStore>) -> ServerContext {
        ServerContext { blocks }
    }
}