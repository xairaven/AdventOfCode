use crate::days::day10::errors::Day10Error;
use crate::errors::Error;
use itertools::Itertools;
use std::ops::BitXor;
use std::str::FromStr;

// Answer: 452

pub fn process(input: String) -> Result<String, Error> {
    let configurations = parse_input(input)?;
    let sum_of_presses = find_sum_of_presses(configurations)?;

    Ok(sum_of_presses.to_string())
}

fn find_sum_of_presses(
    configurations: Vec<MachineConfiguration>,
) -> Result<usize, Day10Error> {
    let mut total_presses = 0;

    for configuration in configurations {
        total_presses += find_fewest_button_presses(
            &configuration.needed_schematic,
            &configuration.schematics,
        )?;
    }

    Ok(total_presses)
}

fn find_fewest_button_presses(
    needed_state: &Schematic, schematics: &[Schematic],
) -> Result<usize, Day10Error> {
    let combinations: Vec<Vec<Schematic>> = schematics
        .iter()
        .cloned()
        .powerset()
        .filter(|c| !c.is_empty())
        .collect();

    // Checking if all empty
    let mut is_machine_disabled = true;
    for value in &needed_state.states {
        if *value {
            is_machine_disabled = false;
            break;
        }
    }
    if is_machine_disabled {
        return Ok(0);
    }

    let mut fewest_presses = usize::MAX;
    // Checking all existing combinations...
    for combination in combinations {
        let presses = combination.len();
        let mut current_state = Schematic::from_states(vec![false; needed_state.len()]);
        for schema in combination {
            current_state = schema.bitxor(&current_state)?;
        }
        if needed_state.eq(&current_state) && presses < fewest_presses {
            fewest_presses = presses;
        }
    }

    Ok(fewest_presses)
}

fn parse_input(input: String) -> Result<Vec<MachineConfiguration>, Day10Error> {
    let lines = input.lines().collect::<Vec<&str>>();

    let mut configurations = Vec::new();
    for line in lines {
        let configuration = MachineConfiguration::from_str(line)?;
        configurations.push(configuration);
    }

    Ok(configurations)
}

#[derive(Debug)]
struct MachineConfiguration {
    pub needed_schematic: Schematic,
    pub schematics: Vec<Schematic>,
}

#[derive(Debug, Clone)]
struct Schematic {
    states: Vec<bool>,
}

impl Schematic {
    pub fn new(buffer: Vec<usize>, size: usize) -> Self {
        let mut states = vec![false; size];
        for index in buffer {
            states[index] = true;
        }
        Self { states }
    }

    pub fn len(&self) -> usize {
        self.states.len()
    }

    pub fn from_states(states: Vec<bool>) -> Self {
        Self { states }
    }
}

impl PartialEq for Schematic {
    fn eq(&self, other: &Self) -> bool {
        if self.len() != other.len() {
            return false;
        }

        for i in 0..self.len() {
            if self.states[i] != other.states[i] {
                return false;
            }
        }

        true
    }
}

impl BitXor for &Schematic {
    type Output = Result<Schematic, Day10Error>;

    fn bitxor(self, rhs: Self) -> Self::Output {
        if self.len() != rhs.len() {
            return Err(Day10Error::SchematicSizeMismatch);
        }

        let mut new_state = Vec::new();
        for i in 0..self.len() {
            new_state.push(self.states[i] ^ rhs.states[i]);
        }

        Ok(Schematic::from_states(new_state))
    }
}

impl FromStr for MachineConfiguration {
    type Err = Day10Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split_whitespace().collect::<Vec<&str>>();

        let light_indicators = parts
            .first()
            .ok_or(Self::Err::InvalidInputFormat)?
            .trim()
            .trim_start_matches('[')
            .trim_end_matches(']')
            .chars()
            .map(|symbol| match symbol {
                '.' => Ok(false),
                '#' => Ok(true),
                _ => Err(Self::Err::InvalidLightIndicator(symbol)),
            })
            .collect::<Vec<Result<bool, Self::Err>>>();
        let mut needed_states = Vec::new();
        for indicator in light_indicators {
            needed_states.push(indicator?);
        }
        let needed_schematic = Schematic::from_states(needed_states);

        let joltage_results = parts
            .last()
            .ok_or(Self::Err::InvalidInputFormat)?
            .trim()
            .trim_start_matches('{')
            .trim_end_matches('}')
            .split(',')
            .map(|number| number.parse::<usize>().map_err(Self::Err::InvalidJoltage))
            .collect::<Vec<Result<usize, Self::Err>>>();
        let mut joltages: Vec<usize> = Vec::new();
        for joltage in joltage_results {
            joltages.push(joltage?);
        }

        let mut schematics: Vec<Schematic> = Vec::new();
        // Iterating on line, skipping first element (states) and last (joltages)
        for schematic in parts.iter().skip(1).take(parts.len() - 2) {
            let schematic_results = schematic
                .trim()
                .trim_start_matches('(')
                .trim_end_matches(')')
                .split(',')
                .map(|number| {
                    number.parse::<usize>().map_err(Self::Err::InvalidSchematic)
                })
                .collect::<Vec<Result<usize, Self::Err>>>();
            let mut schematic = Vec::new();
            for button_number in schematic_results {
                schematic.push(button_number?);
            }
            let schematic = Schematic::new(schematic, needed_schematic.states.len());
            schematics.push(schematic);
        }

        Ok(Self {
            needed_schematic,
            schematics,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0() {
        let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

        let actual = process(input.to_string()).unwrap();
        let expected = "7";

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}";

        let actual = process(input.to_string()).unwrap();
        let expected = "2";

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let input = "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}";

        let actual = process(input.to_string()).unwrap();
        let expected = "3";

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let input = "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

        let actual = process(input.to_string()).unwrap();
        let expected = "2";

        assert_eq!(actual, expected);
    }
}
