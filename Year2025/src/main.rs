use crate::Error::NotImplemented;
use crate::cli::Cli;
use clap::Parser;
use thiserror::Error;

pub fn main() {
    let args = Cli::parse();

    if ![1, 2].contains(&args.part) {
        eprintln!("Error: {}", Error::InvalidPart(args.part));
        std::process::exit(1);
    }

    let input = io::read_input_file(&args).unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    });

    let result: Result<String, Error> = match (args.day, args.part) {
        (1, 1) => Err(NotImplemented),
        (day, _) => Err(Error::InvalidDay(day)),
    };

    match result {
        Ok(output) => {
            let header = format!("--- Day {}. Part {} ---", args.day, args.part);
            println!("{}\n\n{}", header, output)
        },
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        },
    }
}

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
}

mod cli;
mod io;
