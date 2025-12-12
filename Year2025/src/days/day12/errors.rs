use thiserror::Error;

#[derive(Debug, Error)]
pub enum Day12Error {
    #[error("Invalid query format: expected 'WxH: ...'")]
    InvalidQueryFormat,

    #[error("Invalid dimensions format: expected 'WxH'")]
    InvalidDimensionFormat,

    #[error("Failed to parse integer")]
    ParseIntError(#[from] std::num::ParseIntError),
}
