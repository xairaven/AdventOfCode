use crate::cli::Cli;
use crate::errors::Error;
use clap::Parser;

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

    let result = days::run(input, args.day, args.part);

    match result {
        Ok(output) => println!("{}", output),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        },
    }
}

mod cli;
mod days;
mod errors;
mod io;
