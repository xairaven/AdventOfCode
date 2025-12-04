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

    #[error("Day 02. {0}")]
    Day02(#[from] days::day02::errors::Day02Error),

    #[error("Day 03. {0}")]
    Day03(#[from] days::day03::errors::Day03Error),

    #[error("Day 04. {0}")]
    Day04(#[from] days::day04::errors::Day04Error),
}
