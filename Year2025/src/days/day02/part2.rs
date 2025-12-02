use crate::days::day02::errors::Day02Error;
use crate::errors::Error;

// Answer: 20077272987

pub fn process(input: String) -> Result<String, Error> {
    let ranges = input.split(",").collect::<Vec<&str>>();
    let mut processed_ranges = Vec::new();
    for range in ranges {
        let bounds = range.split("-").collect::<Vec<&str>>();
        let start = bounds
            .first()
            .ok_or(Day02Error::WithoutStartNumber(range.to_string()))?
            .parse::<i64>()
            .map_err(Day02Error::FailedToParseInt)?;
        let end = bounds
            .get(1)
            .ok_or(Day02Error::WithoutEndNumber(range.to_string()))?
            .parse::<i64>()
            .map_err(Day02Error::FailedToParseInt)?;
        let range = start..=end;
        processed_ranges.push(range);
    }
    let ranges = processed_ranges;

    let mut sum = 0;
    for range in ranges {
        sum += sum_from_range(range);
    }

    Ok(sum.to_string())
}

fn sum_from_range(range: std::ops::RangeInclusive<i64>) -> i64 {
    let mut sum = 0;
    for number in range {
        let number = number.to_string();
        if is_number_strange(&number) {
            sum += number.parse::<i64>().unwrap_or(0);
        }
    }

    sum
}

fn is_number_strange(number: &str) -> bool {
    let length = number.len();
    for i in 1..length {
        if !length.is_multiple_of(i) {
            continue;
        }

        let mut parts = number.as_bytes().chunks(i);
        let first_part = match parts.clone().next() {
            Some(part) => part,
            None => unreachable!(),
        };

        if parts.all(|part| part == first_part) {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_01() {
        let range = 11..=22;
        let result = sum_from_range(range);
        assert_eq!(result, 11 + 22);
    }

    #[test]
    fn test_02() {
        let range = 95..=115;
        let result = sum_from_range(range);
        assert_eq!(result, 99 + 111);
    }

    #[test]
    fn test_03() {
        let range = 998..=1012;
        let result = sum_from_range(range);
        assert_eq!(result, 999 + 1010);
    }

    #[test]
    fn test_04() {
        let range = "1188511880-1188511890";
        let result = process(range.to_string()).unwrap();
        assert_eq!(result, 1188511885.to_string());
    }

    #[test]
    fn test_05() {
        let range = "222220-222224";
        let result = process(range.to_string()).unwrap();
        assert_eq!(result, "222222");
    }

    #[test]
    fn test_06() {
        let range = "1698522-1698528";
        let result = process(range.to_string()).unwrap();
        assert_eq!(result, "0");
    }

    #[test]
    fn test_07() {
        let range = "446443-446449";
        let result = process(range.to_string()).unwrap();
        assert_eq!(result, "446446");
    }

    #[test]
    fn test_08() {
        let range = "38593856-38593862";
        let result = process(range.to_string()).unwrap();
        assert_eq!(result, "38593859");
    }

    #[test]
    fn test_09() {
        let range = "565653-565659";
        let result = process(range.to_string()).unwrap();
        assert_eq!(result, "565656");
    }

    #[test]
    fn test_10() {
        let range = "824824821-824824827";
        let result = process(range.to_string()).unwrap();
        assert_eq!(result, "824824824");
    }

    #[test]
    fn test_11() {
        let range = "2121212118-2121212124";
        let result = process(range.to_string()).unwrap();
        assert_eq!(result, "2121212121");
    }

    #[test]
    fn test_result() {
        let ranges = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        let result = process(ranges.to_string()).unwrap();
        assert_eq!(result, "4174379265");
    }
}
