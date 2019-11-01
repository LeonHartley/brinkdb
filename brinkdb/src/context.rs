use crate::brink::BrinkSystemData;
use std::sync::RwLock;

pub struct BrinkSystemContext {
    system: RwLock<BrinkSystemData>
}

impl BrinkSystemContext {
    pub fn new(data: BrinkSystemData) -> BrinkSystemContext {
        BrinkSystemContext {
            system: RwLock::new(data)
        }
    }
}
