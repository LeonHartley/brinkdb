use crate::index::search::{BrinkIndexSearch, BrinkIndexSearchKey};
use crate::index::{BrinkIndexStore, BrinkIndex};

#[test]
pub fn brink_index_search() {
    let search = BrinkIndexSearch::new(vec! {
        BrinkIndexSearchKey::new("name".into(), "Leon".into())
    });

    let mut store = BrinkIndexStore::new();
    store.add(BrinkIndex {
        key: "name".into(),
        json_selector: "$.name".into(),
    });

    store.add(BrinkIndex {
        key: "email".into(),
        json_selector: "$.email".into(),
    });

    search.search(&mut store);
}
