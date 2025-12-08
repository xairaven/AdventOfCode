use thiserror::Error;

#[derive(Debug, Error)]
pub enum Day08Error {
    #[error("Invalid input format: {0}")]
    InvalidInputFormat(String),

    #[error("Invalid coordinate value: {0}")]
    InvalidCoordinate(#[from] std::num::ParseIntError),
}
