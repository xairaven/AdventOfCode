use crate::Error;
use crate::days::day01::errors::Day01Error;

// Answer: 6099

pub fn process(input: String) -> Result<String, Error> {
    let mut position: i16 = 50;
    let mut zero_counter: i16 = 0;

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
        (position, zero_counter) =
            change_position(position, zero_counter, direction, step);
    }

    let code = zero_counter.to_string();
    Ok(code)
}

fn change_position(
    mut position: i16, mut zero_counter: i16, direction: Direction, step: i16,
) -> (i16, i16) {
    let previous_position = position;
    position = match direction {
        Direction::Left => position - step,
        Direction::Right => position + step,
    };

    // Wrap around the position within 0-100
    if position >= 100 {
        let new_position = position % 100;
        zero_counter += (position - new_position) / 100;
        position = new_position;
    } else if position < 0 {
        zero_counter += (-position - (-position % 100)) / 100;
        if previous_position != 0 {
            zero_counter += 1;
        }
        position = (100 - (-position % 100)) % 100;
    } else if position == 0 {
        zero_counter += 1;
    }

    (position, zero_counter)
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

#[cfg(test)]
mod tests {
    #[test]
    fn test_1() {
        let position = 50;
        let zero_counter = 0;

        let (_, zero_counter) =
            super::change_position(position, zero_counter, super::Direction::Left, 60);

        assert_eq!(zero_counter, 1);
    }

    #[test]
    fn test_2() {
        let position = 1;

        let (_, zero_counter) =
            super::change_position(position, 0, super::Direction::Right, 123);

        assert_eq!(zero_counter, 1);
    }

    #[test]
    fn test_3() {
        let position = 1;

        let (_, zero_counter) =
            super::change_position(position, 0, super::Direction::Left, 123);

        assert_eq!(zero_counter, 2);
    }

    #[test]
    fn test_4() {
        let position = 0;

        let (_, zero_counter) =
            super::change_position(position, 0, super::Direction::Left, 200);

        assert_eq!(zero_counter, 2);
    }

    #[test]
    fn test_5() {
        let position = 50;

        let (_, zero_counter) =
            super::change_position(position, 0, super::Direction::Left, 200);

        assert_eq!(zero_counter, 2);
    }

    #[test]
    fn test_6() {
        let position = 50;

        let (_, zero_counter) =
            super::change_position(position, 0, super::Direction::Right, 200);

        assert_eq!(zero_counter, 2);
    }

    #[test]
    fn test_7() {
        let position = 0;

        let (_, zero_counter) =
            super::change_position(position, 0, super::Direction::Right, 201);

        assert_eq!(zero_counter, 2);
    }

    #[test]
    fn test_8() {
        let position = 50;

        let (_, zero_counter) =
            super::change_position(position, 0, super::Direction::Left, 50);

        assert_eq!(zero_counter, 1);
    }
}
