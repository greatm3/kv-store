use std::collections::HashMap;

pub struct Store {
    data_store: HashMap<String, String>
}

impl Store {
    pub fn new() -> Store {
        let data_store = HashMap::new();
        Store { data_store }
    }

    pub fn set(&mut self, key: String, value: String) {
        self.data_store.entry(key).or_insert(value);
    }
}