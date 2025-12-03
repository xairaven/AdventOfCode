use thiserror::Error;

#[derive(Debug, Error)]
pub enum Day03Error {
    #[error("Invalid amount of batteries: {0}")]
    InvalidAmountOfBatteries(usize),

    #[error("Failed to convert joltage: {0}")]
    FailedToConvertJoltage(#[from] std::num::ParseIntError),

    #[error("Failed to find answer")]
    ValueNotFound,
}
