use thiserror::Error;

#[derive(Debug, Error)]
pub enum Day10Error {
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
}
