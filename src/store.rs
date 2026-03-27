use std::collections::HashMap;

use crate::command::Command;
use crate::error::StoreError;

pub struct Store {
    data: HashMap<String, String>,
}

impl Store {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn apply(&mut self, command: Command) -> Result<Option<String>, StoreError> {
        match command {
            Command::Put { key, value } => Ok(self.data.insert(key, value)),
            Command::Delete{ key } => match self.data.remove(&key) {
                Some(value) => Ok(Some(value)),
                None => Err(StoreError::KeyNotFound(key)),
            },
        }
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.data.get(key)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::command::Command;
    //use crate::error::StoreError;

    #[test]
    fn tests_apply_put(){
        let mut store = Store::new();

        let result = store.apply(Command::Put {
            key: "name".to_string(),
            value: "Yohan".to_string(),
        });

        assert_eq!(result, Ok(None));
        assert_eq!(store.get("name"), Some(&"Yohan".to_string()));
    }

    #[test]
    fn tests_apply_delete_existing_key() {
        let mut store = Store::new();

        store.apply(Command::Put {
            key: "project".to_string(),
            value: "Apex".to_string(),
        }).unwrap();

        let result = store.apply(Command::Delete {
            key: "project".to_string(),
        });

        assert_eq!(result, Ok(Some("Apex".to_string())));
        assert_eq!(store.get("project"), None);
    }

    #[test]
    fn test_apply_delete_missing_key() {
        let mut store = Store::new();

        let result = store.apply(Command::Delete {
            key: "missing".to_string(),
        });
        assert_eq!(result, Err(StoreError::KeyNotFound("missing".to_string())));
    }
}   
