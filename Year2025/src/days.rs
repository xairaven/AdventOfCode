use crate::Error;

pub fn run(input: String, day: u8, part: u8) -> Result<String, Error> {
    let result: Result<String, Error> = match (day, part) {
        (1, 1) => Err(Error::NotImplemented),
        (day, _) => Err(Error::InvalidDay(day)),
    };

    let output = result?;

    let header = format!("--- Day {}. Part {} ---", day, part);
    let formatted_output = format!("{}\n\n{}", header, output);

    Ok(formatted_output)
}

pub mod day_01_1; // 01.12.2025
