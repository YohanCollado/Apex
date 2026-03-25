use std::collections::HashMap;

use crate::command::Command;

pub struct Store {
    data: HashMap<String, String>,
}

impl Store {
    pub fn new()-> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn apply(&mut self, command: Command) -> Option<String> {
        match command {
            Command::Put {key, value} => self.data.insert(key,value),
            Command::Delete{key} => self.data.remove(&key),
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

    #[test]
    fn test_apply_test() {
        let mut store = Store::new();

        let command = Command::Put {
            key: "name".to_string(),
            value: "Yohan".to_string(),
        };

        store.apply(command);

        assert_eq!(store.get("name"), Some(&"Yohan".to_string()));
    }

    #[test]
    fn test_apply_delete() {
        let mut store = Store::new();

        store.apply(Command::Put {
            key:"project".to_string(),
            value: "Apex".to_string(),
        });

        store.apply(Command::Delete{
            key: "project".to_string(),
        });

        assert_eq!(store.get("project"), None);
    }
}