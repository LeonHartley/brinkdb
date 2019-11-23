use crate::storage::{BrinkDataRef, BrinkDataState, BrinkDataFileRef};
use chrono::Utc;

pub struct DataRefBuilder {
    state: BrinkDataState,
    timestamp: i64,
    version: Option<i32>,
    file: Option<BrinkDataFileRef>,
}

impl DataRefBuilder {
    fn new() -> DataRefBuilder {
        let state = BrinkDataState::Created;
        let timestamp = Utc::now().timestamp();
        let version = None;
        let file = None;

        DataRefBuilder { state, timestamp, version, file }
    }

    fn version(&mut self, version: i32) -> &mut DataRefBuilder {
        self.version = Some(version);

        self
    }

    fn file(&mut self, file: BrinkDataFileRef) -> &mut DataRefBuilder {
        self.file = Some(file);

        self
    }

    fn build(self) -> BrinkDataRef {
        BrinkDataRef {
            version: self.version.expect("BrinkDataRef::version"),
            file: self.file,
            state: self.state,
            timestamp: self.timestamp,
        }
    }
}