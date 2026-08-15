use std::collections::HashMap;

pub struct Store {
    data_store: HashMap<String, String>
}

impl Store {
    pub fn new() -> Store {
        Store {
            data_store: HashMap::new()
        }
    }

    pub fn set(&mut self, key: String, value: String) {
        self.data_store.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.data_store.get(key)
    }
}