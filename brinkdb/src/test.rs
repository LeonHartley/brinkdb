use crate::brink::{BrinkSystemData, BrinkMetadata, BrinkNamespace};

pub fn create_system_data() -> BrinkSystemData {
    BrinkSystemData::new(BrinkMetadata {
        version: "1".to_string(),
        server: "0".to_string(),
    })
}

pub fn create_namespace() -> BrinkNamespace {
    BrinkNamespace::new(1, "kek".to_string(), 1, 0)
}

#[test]
pub fn brink_add_default_namespace() {
    let mut system = create_system_data();

    assert_eq!(system.active_namespaces().len(), 1);
}


