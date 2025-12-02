use thiserror::Error;

#[derive(Debug, Error)]
pub enum Day02Error {
    #[error("Failed to parse integer: {0}")]
    FailedToParseInt(#[from] std::num::ParseIntError),
    #[error("Input is missing start number: {0}")]
    WithoutStartNumber(String),
    #[error("Input is missing end number: {0}")]
    WithoutEndNumber(String),
}
