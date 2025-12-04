use thiserror::Error;

#[derive(Debug, Error)]
pub enum Day04Error {
    #[error("Invalid grid size")]
    InvalidGridSize,

    #[error("Invalid item character: {0}")]
    UnknownItem(char),
}
