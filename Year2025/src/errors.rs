use crate::{days, io};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("IO. {0}")]
    Io(#[from] io::IoError),

    #[error("Invalid Day: {0}")]
    InvalidDay(u8),

    #[error("Invalid Part. It have to be 1 or 2. Value: {0}")]
    InvalidPart(u8),

    #[error("Day is not implemented yet")]
    NotImplemented,

    #[error("Day 01. {0}")]
    Day01(#[from] days::day01::errors::Day01Error),
}
