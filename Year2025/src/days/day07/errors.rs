use thiserror::Error;

#[derive(Debug, Error)]
pub enum Day07Error {
    #[error("Invalid item character: {0}")]
    InvalidItem(char),

    #[error("Start position not found in the grid")]
    StartNotFound,

    #[error("Unexpected start position at [{0}, {1}]")]
    UnexpectedStart(usize, usize),
}
