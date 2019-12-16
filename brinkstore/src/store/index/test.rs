use crate::store::index::search::{BrinkIndexSearch, BrinkIndexSearchKey, BrinkIndexSearchResult};
use crate::store::index::{BrinkIndex, BrinkIndexStore};

#[test]
pub fn brink_index_search() {
    let search =
        BrinkIndexSearch::new(vec![BrinkIndexSearchKey::new("name".into(), "Leon".into())]);

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
