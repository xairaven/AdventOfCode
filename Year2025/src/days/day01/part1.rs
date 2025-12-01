use crate::Error;
use crate::days::day01::errors::Day01Error;

// Answer: 999

pub fn process(input: String) -> Result<String, Error> {
    let mut zero_counter: i16 = 0;
    let mut position: i16 = 50;

    for line in input.lines() {
        let direction_char =
            line.chars().next().ok_or(Day01Error::InstructionNotFound)?;
        let direction = Direction::try_from(direction_char)?;
        let step = line
            .get(1..)
            .ok_or(Day01Error::FailedGetStep)?
            .trim()
            .parse::<i16>()
            .map_err(Day01Error::FailedParseInt)?;
        position = match direction {
            Direction::Left => position - step,
            Direction::Right => position + step,
        };

        // Wrap around the position within 0-100
        if position >= 100 {
            position %= 100;
        } else if position < 0 {
            position = (100 - (-position % 100)) % 100;
        }

        println!(
            "Direction: {:?}, Step: {}, Current Position: {}",
            direction, step, position
        );

        if position == 0 {
            zero_counter += 1;
        }
    }

    let code = zero_counter.to_string();
    Ok(code)
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

impl TryFrom<char> for Direction {
    type Error = Day01Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'L' => Ok(Direction::Left),
            'R' => Ok(Direction::Right),
            invalid => Err(Day01Error::InvalidInstruction(invalid)),
        }
    }
}
