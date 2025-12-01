use thiserror::Error;

#[derive(Debug, Error)]
pub enum Day01Error {
    // Part 1
    #[error("Part 1. Invalid instruction: {0}")]
    InvalidInstruction(char),

    #[error("Part 1. No instruction found in the line")]
    InstructionNotFound,

    #[error("Part 1. Failed to get step value from instruction")]
    FailedGetStep,

    #[error("Part 1. Failed to parse integer from step value: {0}")]
    FailedParseInt(#[from] std::num::ParseIntError),
}
