use crate::command::Command;
use crate::error::StoreError;
use crate::log::ReplicatedLog;
use crate::store::Store;
use crate::log::LogEntry;

pub struct ApexNode {
    store: Store,
    log: ReplicatedLog,
}

impl ApexNode {
    pub fn new() -> Self {
        Self {
            store: Store::new(),
            log: ReplicatedLog::new(),
        }
    }

    pub fn apply_command(&mut self, command: Command) -> Result<u64, StoreError> {
        let log_index = self.log.append(command.clone());

        match self.store.apply(command) {
            Ok(_) => Ok(log_index),
            Err(error) => Err(error)
        }
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.store.get(key)
    }

    pub fn last_log_index(&self) -> u64 {
        self.log.last_index()
    }

    pub fn get_log(&self) ->&[crate::log::LogEntry] {
        self.log.entries()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::command::Command;
    use crate::error::StoreError;

    #[test]
    fn test_apply_put_updates_store_and_log() {
        let mut node = ApexNode::new();

        let result = node.apply_command(Command:: Put {
            key: "name".to_string(),
            value: "Yohan".to_string(),
        });

        assert_eq!(result, Ok(1));
        assert_eq!(node.get("name"), Some(&"Yohan".to_string()));
        assert_eq!(node.last_log_index(), 1);
    }

    #[test]
    fn test_apply_delete_existing_key() {
        let mut node = ApexNode::new();

        node.apply_command(Command::Put {
            key: "project".to_string(),
            value: "Apex".to_string(),
        })
        .unwrap();

        let result = node.apply_command(Command::Delete {
            key: "project".to_string(),
        });

        assert_eq!(result, Ok(2));
        assert_eq!(node.get("project"), None);
        assert_eq!(node.last_log_index(), 2);
    }

    #[test]
    fn test_apply_delete_missing_key() {
        let mut node = ApexNode::new();

        let result = node.apply_command(Command::Delete {
            key: "missing".to_string(),
        });

        assert_eq!(result, Err(StoreError::KeyNotFound("missing".to_string())));
        assert_eq!(node.last_log_index(), 1)
    }
}