use thiserror::Error;

#[derive(Debug, Error)]
pub enum Day05Error {
    #[error("Failed to parse integer")]
    FailedToParseInt(#[from] std::num::ParseIntError),

    #[error("Invalid range format")]
    InvalidRangeFormat,
}
