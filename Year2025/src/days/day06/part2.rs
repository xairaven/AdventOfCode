use crate::days::day06::errors::Day06Error;
use crate::errors::Error;

// Answer: 12841228084455

type GridNumber = u64;

pub fn process(input: String) -> Result<String, Error> {
    let input = Input::try_from(input.as_str())?;
    let result = count(input)?;

    Ok(result.to_string())
}

fn count(input: Input) -> Result<GridNumber, Error> {
    debug_assert_eq!(input.grid.len(), input.operations.len());

    let mut sum = 0;

    for (index, operation) in input.operations.iter().enumerate() {
        let mut result_of_equation: GridNumber = match operation {
            Operation::Plus => 0,
            Operation::Multiply => 1,
        };

        for number in &input.grid[index] {
            match operation {
                Operation::Plus => result_of_equation += number,
                Operation::Multiply => result_of_equation *= number,
            }
        }

        sum += result_of_equation;
    }

    Ok(sum)
}

#[derive(Debug)]
pub struct Input {
    pub grid: Vec<Vec<GridNumber>>,
    pub operations: Vec<Operation>,
}

impl TryFrom<&str> for Input {
    type Error = Day06Error;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let amount_of_lines = input.lines().count();
        let amount_of_symbols = input.lines().map(|line| line.len()).max().unwrap_or(0);

        let mut grid = Vec::new();
        let mut numbers = Vec::new();
        let mut operations = Vec::new();

        for j in 0..amount_of_symbols {
            let mut is_row_separator = true;
            let mut number = String::new();

            for i in 0..amount_of_lines {
                let symbol =
                    match input.lines().nth(i).and_then(|line| line.chars().nth(j)) {
                        None => continue,
                        Some(value) => value,
                    };

                if !symbol.is_whitespace() {
                    is_row_separator = false;
                } else {
                    continue;
                }

                if i == amount_of_lines - 1 {
                    if !symbol.is_whitespace() {
                        let operation = Operation::try_from(symbol.to_string().as_str())?;
                        operations.push(operation);
                    }
                } else {
                    number.push(symbol);
                }
            }

            if j == amount_of_symbols - 1 {
                let number = number
                    .parse::<GridNumber>()
                    .map_err(|_| Day06Error::InvalidNumber(number.clone()))?;
                numbers.push(number);
                is_row_separator = true;
            }

            if is_row_separator {
                grid.push(numbers);
                numbers = Vec::new();
            } else {
                let number = number
                    .parse::<GridNumber>()
                    .map_err(|_| Day06Error::InvalidNumber(number.clone()))?;
                numbers.push(number);
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
        let input = "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  ";

        let actual = process(input.to_string()).unwrap();
        let expected = "3263827".to_string();

        assert_eq!(actual, expected);
    }
}
