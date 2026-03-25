#[derive(Debug, PartialEq)]
pub enum StoreError {
    KeyNotFound(String),
}