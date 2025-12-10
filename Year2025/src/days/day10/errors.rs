use thiserror::Error;

#[derive(Debug, Error)]
pub enum Day10Error {
    // Part 1
    #[error("Invalid input format")]
    InvalidInputFormat,

    #[error("Invalid light indicator: {0}")]
    InvalidLightIndicator(char),

    #[error("Invalid joltage: {0}")]
    InvalidJoltage(std::num::ParseIntError),

    #[error("Invalid schematic: {0}")]
    InvalidSchematic(std::num::ParseIntError),

    #[error("Schematics have different lengths")]
    SchematicSizeMismatch,

    // Part 2
    #[error("No model found that matches the needed schematic and joltages")]
    NoModelFound,

    #[error("Variable evaluation failed")]
    VariableEvaluationFailed,

    #[error("Encountered a bad result during evaluation")]
    BadResult,

    #[error("Evaluation led to unsatisfiable conditions. Machine ID: {0}")]
    Unsatisfiable(usize),

    #[error("The query was interrupted, timed out or otherwise failed. Machine ID: {0}")]
    UnknownEvaluationResult(usize),
}
