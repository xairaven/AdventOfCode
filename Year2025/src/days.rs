use crate::Error;

pub fn run(input: String, day: u8, part: u8) -> Result<String, Error> {
    let result: Result<String, Error> = match (day, part) {
        (1, 1) => day01::part1::process(input),
        (1, 2) => day01::part2::process(input),
        (2, 1) => day02::part1::process(input),
        (2, 2) => day02::part2::process(input),
        (3, 1) => day03::part1::process(input),
        (3, 2) => day03::part2::process(input),
        (4, 1) => day04::part1::process(input),
        (4, 2) => day04::part2::process(input),
        (5, 1) => day05::part1::process(input),
        (5, 2) => day05::part2::process(input),
        (6, 1) => day06::part1::process(input),
        (6, 2) => day06::part2::process(input),
        (7, 1) => day07::part1::process(input),
        (7, 2) => day07::part2::process(input),
        (8, 1) => day08::part1::process(input),
        (8, 2) => day08::part2::process(input),
        (9, 1) => day09::part1::process(input),
        (9, 2) => day09::part2::process(input),
        (10, 1) => day10::part1::process(input),
        (10, 2) | (11, 1) | (11, 2) | (12, 1) | (12, 2) => Err(Error::NotImplemented),
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
pub mod day02 {
    pub mod errors;
    pub mod part1; // 02.12.2025
    pub mod part2; // 02.12.2025
}
pub mod day03 {
    pub mod errors;
    pub mod part1; // 03.12.2025
    pub mod part2; // 03.12.2025
}
pub mod day04 {
    pub mod errors;
    pub mod part1; // 04.12.2025
    pub mod part2; // 04.12.2025
}
pub mod day05 {
    pub mod errors;
    pub mod part1; // 05.12.2025
    pub mod part2; // 05.12.2025
}
pub mod day06 {
    pub mod errors;
    pub mod part1; // 06.12.2025
    pub mod part2; // 06.12.2025
}
pub mod day07 {
    pub mod errors;
    pub mod part1; // 07.12.2025
    pub mod part2; // 07.12.2025
}
pub mod day08 {
    pub mod errors;
    pub mod part1; // 08.12.2025
    pub mod part2; // 08.12.2025
}
pub mod day09 {
    pub mod errors;
    pub mod part1; // 09.12.2025
    pub mod part2; // 09.12.2025
}
pub mod day10 {
    pub mod errors;
    pub mod part1; // 10.12.2025
}
