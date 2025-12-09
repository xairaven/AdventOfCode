use thiserror::Error;

#[derive(Debug, Error)]
pub enum Day09Error {
    #[error("Invalid input format")]
    InvalidInputFormat,

    #[error("Invalid coordinate value: {0}")]
    InvalidCoordinate(#[from] std::num::ParseIntError),
}
