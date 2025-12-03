use crate::days::day03::errors::Day03Error;
use crate::errors::Error;

// Answer: 168794698570517

pub fn process(input: String) -> Result<String, Error> {
    let mut sum = 0;

    for line in input.lines() {
        let chars = line.chars().collect::<Vec<char>>();
        let mut numbers_row: Vec<u8> = vec![];
        for c in &chars {
            let number = c
                .to_string()
                .parse::<u8>()
                .map_err(Day03Error::FailedToConvertJoltage)?;
            numbers_row.push(number);
        }

        let joltage = row_joltage(numbers_row)?;
        sum += joltage as i128;
    }

    Ok(sum.to_string())
}

fn row_joltage(numbers: Vec<u8>) -> Result<i64, Day03Error> {
    if numbers.len() < 12 {
        return Err(Day03Error::InvalidAmountOfBatteries(numbers.len()));
    }

    const VOLTAGE_NUMBER_SIZE: u8 = 12;
    find_combination(&numbers, VOLTAGE_NUMBER_SIZE)
        .ok_or(Day03Error::ValueNotFound)?
        .parse::<i64>()
        .map_err(Day03Error::FailedToConvertJoltage)
}

fn find_combination(rest: &[u8], voltage_number_size: u8) -> Option<String> {
    if voltage_number_size == 0 {
        return Some(String::new());
    }

    let mut current_max: u8 = 0;
    let mut solved_part = String::new();

    for (index, number) in rest.iter().enumerate() {
        if *number > current_max {
            let rest = &rest[index + 1..];
            let rest_of_number_size = voltage_number_size - 1;

            if rest.len() < rest_of_number_size as usize {
                continue;
            }

            if let Some(new_solved_part) = find_combination(rest, rest_of_number_size) {
                current_max = *number;
                solved_part = new_solved_part;
            }
        }
    }

    let current_solved_part = format!("{}{}", current_max, solved_part);
    Some(current_solved_part)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input = "987654321111111".to_string();

        let actual = process(input).unwrap();
        let expected = "987654321111";

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let input = "811111111111119".to_string();

        let actual = process(input).unwrap();
        let expected = "811111111119";

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let input = "234234234234278".to_string();

        let actual = process(input).unwrap();
        let expected = "434234234278";

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let input = "818181911112111".to_string();

        let actual = process(input).unwrap();
        let expected = "888911112111";

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_overall() {
        let input = "987654321111111
811111111111119
234234234234278
818181911112111"
            .to_string();

        let actual = process(input).unwrap();
        let expected = "3121910778619".to_string();

        assert_eq!(actual, expected);
    }
}
