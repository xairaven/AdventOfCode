use thiserror::Error;

#[derive(Debug, Error)]
pub enum Day06Error {
    #[error("Invalid number: {0}")]
    InvalidNumber(String),

    #[error("Invalid operation: {0}")]
    InvalidOperation(String),
}
