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

    #[error("Day 05. {0}")]
    Day05(#[from] days::day05::errors::Day05Error),

    #[error("Day 06. {0}")]
    Day06(#[from] days::day06::errors::Day06Error),

    #[error("Day 07. {0}")]
    Day07(#[from] days::day07::errors::Day07Error),

    #[error("Day 08. {0}")]
    Day08(#[from] days::day08::errors::Day08Error),
}
