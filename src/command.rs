#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    Put {key: String, value: String },
    Delete {key: String}
}