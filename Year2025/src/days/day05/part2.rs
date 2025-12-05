use crate::days::day05::errors::Day05Error;
use crate::errors::Error;
use std::ops::RangeInclusive;

// Answer: 354226555270043

pub fn process(input: String) -> Result<String, Error> {
    let input = Input::try_from(input)?;
    let amount = sum_of_fresh_ids(input);

    Ok(amount.to_string())
}

fn sum_of_fresh_ids(input: Input) -> usize {
    let mut ranges = input.ranges;

    ranges.sort_by_key(|range| *range.start());

    let mut merged_ranges: Vec<RangeInclusive<u64>> = vec![];

    let mut current = (*ranges[0].start(), *ranges[0].end());

    for range in ranges.iter().skip(1) {
        if *range.start() <= current.1 + 1 {
            current.1 = current.1.max(*range.end());
        } else {
            merged_ranges.push(RangeInclusive::new(current.0, current.1));
            current.0 = *range.start();
            current.1 = *range.end();
        }
    }

    // Last range
    merged_ranges.push(RangeInclusive::new(current.0, current.1));

    merged_ranges
        .iter()
        .map(|range| range.end() - range.start() + 1)
        .sum::<u64>() as usize
}

#[derive(Debug)]
pub struct Input {
    pub ranges: Vec<RangeInclusive<u64>>,
}

impl TryFrom<String> for Input {
    type Error = Day05Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let mut ranges: Vec<RangeInclusive<u64>> = vec![];

        for line in value.lines() {
            if line.trim().is_empty() {
                break;
            }

            let parts: Vec<&str> = line.split('-').collect();
            if parts.len() != 2 {
                return Err(Self::Error::InvalidRangeFormat);
            }
            let start: u64 = parts
                .first()
                .ok_or(Self::Error::InvalidRangeFormat)?
                .trim()
                .parse()
                .map_err(Self::Error::FailedToParseInt)?;
            let end: u64 = parts
                .get(1)
                .ok_or(Self::Error::InvalidRangeFormat)?
                .trim()
                .parse()
                .map_err(Self::Error::FailedToParseInt)?;

            ranges.push(RangeInclusive::new(start, end));
        }

        Ok(Input { ranges })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

        let actual = process(input.to_string()).unwrap();
        let expected = "14".to_string();
        assert_eq!(actual, expected);
    }
}
