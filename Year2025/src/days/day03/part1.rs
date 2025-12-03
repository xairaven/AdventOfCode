use crate::days::day03::errors::Day03Error;
use crate::errors::Error;

pub fn process(input: String) -> Result<String, Error> {
    let mut sum = 0;

    for line in input.lines() {
        let chars = line.chars().collect::<Vec<char>>();
        let joltage = row_joltage(chars)?;
        sum += joltage as i32;
    }

    Ok(sum.to_string())
}

fn row_joltage(chars: Vec<char>) -> Result<i8, Day03Error> {
    if chars.len() < 2 {
        return Err(Day03Error::InvalidAmountOfBatteries(chars.len()));
    }

    let mut max = 0;

    for (i, first) in chars.iter().enumerate() {
        for second in chars[i + 1..chars.len()].iter() {
            let number = format!("{}{}", first, second).parse::<i8>()?;
            if number > max {
                max = number;
            }
        }
    }

    Ok(max)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input = "987654321111111".chars().collect::<Vec<char>>();

        let actual = row_joltage(input).unwrap();
        let expected = 98;

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let input = "811111111111119".chars().collect::<Vec<char>>();

        let actual = row_joltage(input).unwrap();
        let expected = 89;

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let input = "234234234234278".chars().collect::<Vec<char>>();

        let actual = row_joltage(input).unwrap();
        let expected = 78;

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_4() {
        let input = "818181911112111".chars().collect::<Vec<char>>();

        let actual = row_joltage(input).unwrap();
        let expected = 92;

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_5() {
        let input = "12345".chars().collect::<Vec<char>>();

        let actual = row_joltage(input).unwrap();
        let expected = 45;

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
        let expected = "357".to_string();

        assert_eq!(actual, expected);
    }
}
