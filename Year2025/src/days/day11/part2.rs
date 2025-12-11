use crate::days::day11::errors::Day11Error;
use crate::errors::Error;
use std::collections::HashMap;

// Answer: 458618114529380

pub fn process(input: String) -> Result<String, Error> {
    let graph_map = parse_input(input)?;

    // Helper closure to calculate paths between specific start and end nodes
    let calc_segment = |start, end| -> u64 {
        let mut memo = HashMap::new();
        count_paths(start, end, &graph_map, &mut memo)
    };

    // Scenario 1: svr -> dac -> fft -> out
    // Total paths = (svr->dac) * (dac->fft) * (fft->out)
    let svr_to_dac = calc_segment(
        identifiers::SERVER_RACK,
        identifiers::DIGITAL_ANALOG_CONVERTER,
    );
    let dac_to_fft = calc_segment(
        identifiers::DIGITAL_ANALOG_CONVERTER,
        identifiers::FAST_FOURIER_TRANSFORM,
    );
    let fft_to_out = calc_segment(identifiers::FAST_FOURIER_TRANSFORM, identifiers::END);

    let paths_scenario_1 = svr_to_dac * dac_to_fft * fft_to_out;

    // Scenario 2: svr -> fft -> dac -> out
    // Total paths = (svr->fft) * (fft->dac) * (dac->out)
    let svr_to_fft = calc_segment(
        identifiers::SERVER_RACK,
        identifiers::FAST_FOURIER_TRANSFORM,
    );
    let fft_to_dac = calc_segment(
        identifiers::FAST_FOURIER_TRANSFORM,
        identifiers::DIGITAL_ANALOG_CONVERTER,
    );
    let dac_to_out =
        calc_segment(identifiers::DIGITAL_ANALOG_CONVERTER, identifiers::END);

    let paths_scenario_2 = svr_to_fft * fft_to_dac * dac_to_out;

    let total = paths_scenario_1 + paths_scenario_2;

    Ok(total.to_string())
}

fn count_paths(
    current_node: usize, target_node: usize, graph: &HashMap<usize, Vec<usize>>,
    memo: &mut HashMap<usize, u64>,
) -> u64 {
    // Base case: If we reached the specific target for this segment
    if current_node == target_node {
        return 1;
    }

    // Check memoization cache
    if let Some(&count) = memo.get(&current_node) {
        return count;
    }

    let mut total_paths = 0;

    // Iterate over neighbors
    if let Some(neighbors) = graph.get(&current_node) {
        for &neighbor in neighbors {
            total_paths += count_paths(neighbor, target_node, graph, memo);
        }
    }

    memo.insert(current_node, total_paths);
    total_paths
}

mod identifiers {
    // "you"
    pub const START: usize = 0;
    // "out"
    pub const END: usize = 1;
    // "dac"
    pub const DIGITAL_ANALOG_CONVERTER: usize = 2;
    // "fft"
    pub const FAST_FOURIER_TRANSFORM: usize = 3;
    // "svr"
    pub const SERVER_RACK: usize = 4;
}

fn parse_input(input: String) -> Result<HashMap<usize, Vec<usize>>, Day11Error> {
    let mut graph_map: HashMap<usize, Vec<usize>> = HashMap::new();

    let mut identifiers: HashMap<String, usize> = HashMap::new();

    identifiers.insert("you".to_string(), identifiers::START);
    identifiers.insert("out".to_string(), identifiers::END);
    identifiers.insert("dac".to_string(), identifiers::DIGITAL_ANALOG_CONVERTER);
    identifiers.insert("fft".to_string(), identifiers::FAST_FOURIER_TRANSFORM);
    identifiers.insert("svr".to_string(), identifiers::SERVER_RACK);

    // Because "you", "out", "dac", "fft" and "svr" are already taken
    let mut id_counter: usize = 5;

    for line in input.lines() {
        let parts: Vec<&str> = line.split(":").collect();
        if parts.len() != 2 {
            return Err(Day11Error::InvalidInputFormat(line.to_string()));
        }
        let device_name = parts[0].trim();

        let outputs = parts[1]
            .split_whitespace()
            .map(|s| s.trim())
            .collect::<Vec<&str>>();

        let device_id =
            get_unique_device_id(device_name, &mut identifiers, &mut id_counter);

        let output_ids = outputs
            .iter()
            .map(|&output_name| {
                get_unique_device_id(output_name, &mut identifiers, &mut id_counter)
            })
            .collect::<Vec<usize>>();

        graph_map.insert(device_id, output_ids);
    }

    Ok(graph_map)
}

fn get_unique_device_id(
    name: &str, map: &mut HashMap<String, usize>, counter: &mut usize,
) -> usize {
    match map.get(name) {
        Some(id) => *id,
        None => {
            let new_id = *counter;
            map.insert(name.to_string(), new_id);
            *counter += 1;
            new_id
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";

        let actual = process(input.to_string()).unwrap();
        let expected = "2";

        assert_eq!(actual, expected);
    }
}
