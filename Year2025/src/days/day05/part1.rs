use crate::days::day05::errors::Day05Error;
use crate::errors::Error;
use std::ops::RangeInclusive;

// Answer: 726

pub fn process(input: String) -> Result<String, Error> {
    let input = Input::try_from(input)?;
    let amount = amount_of_fresh_ingredients(input);

    Ok(amount.to_string())
}

fn amount_of_fresh_ingredients(input: Input) -> i16 {
    let mut count: i16 = 0;

    for id in input.ids {
        for range in &input.ranges {
            if range.contains(&id) {
                count += 1;
                break;
            }
        }
    }

    count
}

#[derive(Debug)]
pub struct Input {
    pub ranges: Vec<RangeInclusive<i64>>,
    pub ids: Vec<i64>,
}

impl TryFrom<String> for Input {
    type Error = Day05Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let mut ranges: Vec<RangeInclusive<i64>> = vec![];
        let mut ids: Vec<i64> = vec![];

        let mut are_we_parsing_ids = false;
        for line in value.lines() {
            if line.trim().is_empty() {
                are_we_parsing_ids = true;
                continue;
            }

            match are_we_parsing_ids {
                false => {
                    let parts: Vec<&str> = line.split('-').collect();
                    if parts.len() != 2 {
                        return Err(Self::Error::InvalidRangeFormat);
                    }
                    let start: i64 = parts
                        .first()
                        .ok_or(Self::Error::InvalidRangeFormat)?
                        .trim()
                        .parse()
                        .map_err(Self::Error::FailedToParseInt)?;
                    let end: i64 = parts
                        .get(1)
                        .ok_or(Self::Error::InvalidRangeFormat)?
                        .trim()
                        .parse()
                        .map_err(Self::Error::FailedToParseInt)?;

                    ranges.push(RangeInclusive::new(start, end));
                },
                true => {
                    let id: i64 =
                        line.trim().parse().map_err(Self::Error::FailedToParseInt)?;
                    ids.push(id);
                },
            }
        }

        Ok(Input { ranges, ids })
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
        let input = Input::try_from(input.to_string()).unwrap();
        let result = amount_of_fresh_ingredients(input);
        assert_eq!(result, 3);
    }
}
