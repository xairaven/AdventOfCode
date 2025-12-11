use thiserror::Error;

#[derive(Debug, Error)]
pub enum Day11Error {
    #[error("Invalid input format: {0}")]
    InvalidInputFormat(String),
}
