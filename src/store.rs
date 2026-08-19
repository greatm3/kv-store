use crate::command::Command;
use std::collections::HashMap;

pub struct Store {
    data_store: HashMap<String, String>,
}

impl Store {
    pub fn new() -> Self {
        Store {
            data_store: HashMap::new(),
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

    pub fn execute(&mut self, cmd: Command) -> String {
        match cmd {
            Command::Set { key, value } => {
                self.set(key, value);
                "OK".to_string()
            }
            Command::Get { key } => {
                return match self.get(key.as_str()) {
                    Some(value) => value.to_string(),
                    None => "NIL".to_string(),
                };
            }
            Command::Del { key } => {
                if self.delete(key.as_str()) {
                    return "OK".to_string();
                }

                return "NOT FOUND".to_string();
            }
        }
    }
}

#[cfg(test)]
mod store_tests {
    use super::*;

    fn test_helper() -> Store {
        Store::new()
    }

    #[test]
    fn test_set_get_data() {
        let mut t_store = test_helper();
        t_store.set("name".to_string(), "John".to_string());

        let name = t_store.get("name");

        assert_eq!(name, Some(&String::from("John")))
    }

    #[test]
    fn test_delete_data() {
        let mut t_store = test_helper();
        t_store.set("name".to_string(), "John".to_string());

        assert!(t_store.delete("name"))
    }

    #[test]
    fn test_get_unset_data() {
        let t_store = test_helper();

        let unset_key = t_store.get("name");

        assert_eq!(unset_key, None)
    }

    #[test]
    fn test_delete_unset_data() {
        let mut t_store = test_helper();

        // should return false on unexistent key
        assert!(!t_store.delete("name"))
    }

    #[test]
    fn test_update_data() {
        let mut t_store = test_helper();

        t_store.set("name".to_string(), "mark".to_string());

        // update name
        t_store.set("name".to_string(), "John".to_string());

        let name = t_store.get("name");

        assert_eq!(name, Some(&String::from("John")))
    }
}
