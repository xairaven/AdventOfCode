use crate::Error;

pub fn run(input: String, day: u8, part: u8) -> Result<String, Error> {
    let result: Result<String, Error> = match (day, part) {
        (1, 1) => day01::part1::process(input),
        (1, 2) => day01::part2::process(input),
        (2, 1)
        | (2, 2)
        | (3, 1)
        | (3, 2)
        | (4, 1)
        | (4, 2)
        | (5, 1)
        | (5, 2)
        | (6, 1)
        | (6, 2)
        | (7, 1)
        | (7, 2)
        | (8, 1)
        | (8, 2)
        | (9, 1)
        | (9, 2)
        | (10, 1)
        | (10, 2)
        | (11, 1)
        | (11, 2)
        | (12, 1)
        | (12, 2) => Err(Error::NotImplemented),
        (day, _) => Err(Error::InvalidDay(day)),
    };

    let output = result?;

    let header = format!("--- Day {}. Part {} ---", day, part);
    let formatted_output = format!("{}\n\n{}", header, output);

    Ok(formatted_output)
}

pub mod day01 {
    pub mod errors;
    pub mod part1; // 01.12.2025
    pub mod part2; // 01.12.2025
}
