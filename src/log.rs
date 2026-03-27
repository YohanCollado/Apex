use crate::command::Command;

#[derive(Debug, Clone, PartialEq)] //debug = print, clone = copy, partialeq = compare
pub struct LogEntry {
    pub index: u64,
    pub command: Command,
}

pub struct ReplicatedLog {
    entries: Vec<LogEntry>,
}

impl ReplicatedLog {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub fn append(&mut self, command: Command) -> u64 {
        let index = self.entries.len() as u64 + 1; //start at index 1

        let entry = LogEntry { index, command };
        self.entries.push(entry);

        index
    }

    pub fn get(&self, index: u64) -> Option<&LogEntry> {
        if index == 0 {
            return None;
        }

        self.entries.get((index - 1) as usize)
    }

    pub fn last_index(&self) -> u64 {
        self.entries.len() as u64
    }

    pub fn entries (&self) -> &[LogEntry] {
        &self.entries
    }
}

#[cfg(test)] 
mod test {
    use super::*;
    use crate::command::Command;

    #[test]
    fn test_append_increase_index() {
        let mut log = ReplicatedLog::new();

        let index1 = log.append(Command::Put {
            key: "a".to_string(),
            value: "1".to_string(),
        });

        let index2 = log.append(Command::Put {
            key: "b".to_string(),
            value: "2".to_string(),
        });

        assert_eq!(index1, 1);
        assert_eq!(index2, 2);
        assert_eq!(log.last_index(), 2);
    }

    #[test]
    fn test_get_entry_by_index() {
        let mut log = ReplicatedLog::new();

        log.append(Command::Put {
            key: "Name".to_string(),
            value: "Yohan".to_string(),
        });

        let entry = log.get(1);

        assert_eq!(
            entry,
            Some(&LogEntry {
                index: 1,
                command: Command::Put {
                    key: "Name".to_string(),
                    value: "Yohan".to_string(),
                },
            })
        );
    }

    #[test]
    fn test_get_invalid_index_returns_none() {
        let log = ReplicatedLog::new();

        assert_eq!(log.get(0), None);
        assert_eq!(log.get(1), None);
    }
}