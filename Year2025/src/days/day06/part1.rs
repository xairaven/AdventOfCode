use crate::days::day06::errors::Day06Error;
use crate::errors::Error;

// Answer: ?

pub fn process(input: String) -> Result<String, Error> {
    let input = Input::try_from(input.as_str())?;
    let result = count(input);

    Ok(result.to_string())
}

fn count(input: Input) -> u64 {
    let amount_of_equations = input.grid[0].len();
    let amount_of_numbers = input.grid.len();

    let mut sum: u64 = 0;

    for equation_index in 0..amount_of_equations {
        let operation = &input.operations[equation_index];
        let mut equation_result: u64 = match operation {
            Operation::Plus => 0,
            Operation::Multiply => 1,
        };

        for number in 0..amount_of_numbers {
            let value = input.grid[number][equation_index];
            match operation {
                Operation::Plus => equation_result += value as u64,
                Operation::Multiply => equation_result *= value as u64,
            };
        }

        sum += equation_result;
    }

    sum
}

#[derive(Debug)]
pub struct Input {
    pub grid: Vec<Vec<u16>>,
    pub operations: Vec<Operation>,
}

impl TryFrom<&str> for Input {
    type Error = Day06Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut grid = Vec::new();
        let mut operations = Vec::new();

        let amount_of_lines = value.lines().count();

        for (index, line) in value.lines().enumerate() {
            let mut number_line = vec![];

            for value in line.split_whitespace() {
                if index == amount_of_lines - 1 {
                    let operation = Operation::try_from(value)?;
                    operations.push(operation);
                    continue;
                }

                let number: u16 = value
                    .parse()
                    .map_err(|_| Day06Error::InvalidNumber(value.to_string()))?;
                number_line.push(number);
            }

            if index != amount_of_lines - 1 {
                grid.push(number_line);
            }
        }

        Ok(Self { grid, operations })
    }
}

#[derive(Debug)]
pub enum Operation {
    Plus,
    Multiply,
}

impl TryFrom<&str> for Operation {
    type Error = Day06Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.trim() {
            "+" => Ok(Operation::Plus),
            "*" => Ok(Operation::Multiply),
            _ => Err(Day06Error::InvalidOperation(value.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input = "123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   + ";

        let actual = process(input.to_string()).unwrap();
        let expected = "4277556".to_string();

        assert_eq!(actual, expected);
    }
}
