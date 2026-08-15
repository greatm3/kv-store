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
        // is_some() - check it out later. used this instead of match
        self.data_store.remove(key).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_helper() -> Store {
        Store::new()
    }

    #[test]
    fn set_get_data() {

        let mut t_store = test_helper();
        t_store.set("name".to_string(), "John".to_string());

        let name = t_store.get("name");

        assert_eq!(name, Some(&String::from("John")))
    }

    #[test]
    fn delete_data() {

        let mut t_store = test_helper();
        t_store.set("name".to_string(), "John".to_string());

        assert!(t_store.delete("name"))

    }
}