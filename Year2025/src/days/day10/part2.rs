use crate::days::day10::errors::Day10Error;
use crate::errors::Error;
use std::str::FromStr;
use z3::{Optimize, SatResult};

// Answer: 17424

pub fn process(input: String) -> Result<String, Error> {
    let configurations = parse_input(input)?;
    let sum_of_presses = solve(configurations)?;

    Ok(sum_of_presses.to_string())
}

type Z3Int = z3::ast::Int;

fn solve(configurations: Vec<MachineConfiguration>) -> Result<usize, Day10Error> {
    let mut total_presses: usize = 0;

    for (machine_id, config) in configurations.iter().enumerate() {
        let optimizer = Optimize::new();

        // Create variables: one integer variable for each button (schematic)
        // representing how many times we press it.
        let mut button_variables: Vec<Z3Int> = Vec::new();
        for button_id in 0..config.schematics.len() {
            let variable_name = format!("Machine_{}_Button_{}", machine_id, button_id);
            let variable = Z3Int::new_const(variable_name);

            let constraint = variable.ge(Z3Int::from_u64(0));
            optimizer.assert(&constraint);
            button_variables.push(variable);
        }

        // Create constraints for each joltage counter
        // Each counter must equal its specific target value.
        // The value of a counter is the sum of button presses for all buttons that affect this counter.
        for (joltage_id, &target_joltage) in config.joltages.iter().enumerate() {
            let mut influencing_buttons: Vec<&Z3Int> = Vec::new();

            for (button_id, schematic) in config.schematics.iter().enumerate() {
                // If the schematic has 'true' at this index, it means this button
                // increments this specific counter.
                let state = schematic.states[joltage_id];
                if state {
                    influencing_buttons.push(&button_variables[button_id]);
                }
            }

            // Equation: Sum(buttons_affecting_counter) == target_joltage
            let sum_of_presses = Z3Int::add(&influencing_buttons);
            let target_value = Z3Int::from_u64(target_joltage as u64);

            optimizer.assert(&sum_of_presses.eq(&target_value));
        }

        // Objective: Minimize the total number of button presses
        let all_variable_references: Vec<&Z3Int> = button_variables.iter().collect();
        let total_sum = Z3Int::add(&all_variable_references);
        optimizer.minimize(&total_sum);

        match optimizer.check(&[]) {
            SatResult::Sat => {
                let model = optimizer.get_model().ok_or(Day10Error::NoModelFound)?;
                let mut machine_total = 0;
                let model_competition = true;
                for variable in &button_variables {
                    let value = model
                        .eval(variable, model_competition)
                        .ok_or(Day10Error::VariableEvaluationFailed)?
                        .as_u64()
                        .ok_or(Day10Error::BadResult)?;
                    machine_total += value;
                }
                total_presses += machine_total as usize;
            },
            SatResult::Unsat => return Err(Day10Error::Unsatisfiable(machine_id)),
            SatResult::Unknown => {
                return Err(Day10Error::UnknownEvaluationResult(machine_id));
            },
        }
    }

    Ok(total_presses)
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
    pub _needed_schematic: Schematic,
    pub schematics: Vec<Schematic>,
    pub joltages: Vec<usize>,
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

    pub fn from_states(states: Vec<bool>) -> Self {
        Self { states }
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
            _needed_schematic: needed_schematic,
            schematics,
            joltages,
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
        let expected = "33";

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}";

        let actual = process(input.to_string()).unwrap();
        let expected = "10";

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_2() {
        let input = "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}";

        let actual = process(input.to_string()).unwrap();
        let expected = "12";

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_3() {
        let input = "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

        let actual = process(input.to_string()).unwrap();
        let expected = "11";

        assert_eq!(actual, expected);
    }
}
