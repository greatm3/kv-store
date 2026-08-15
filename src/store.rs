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

    pub fn delete(&mut self, key: &str) -> bool {
        let result = self.data_store.remove(key);

        match result {
            Some(_) => true,
            None => false
        }
    }
}

