use crate::cli::Cli;
use thiserror::Error;

pub fn read_input_file(args: &Cli) -> Result<String, IoError> {
    let path = format!("./inputs/day{:02}.txt", args.day);

    std::fs::read_to_string(path).map_err(IoError::InputFileNotFound)
}

#[derive(Debug, Error)]
pub enum IoError {
    #[error("Input file not found: {0}")]
    InputFileNotFound(std::io::Error),
}
