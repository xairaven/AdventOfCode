use crate::days::day11::errors::Day11Error;
use crate::errors::Error;
use std::collections::HashMap;

// Answer: 511

pub fn process(input: String) -> Result<String, Error> {
    let graph_map = parse_input(input)?;

    // Cache to store the number of paths from a specific node to the end.
    // This prevents recalculating paths for nodes we've already visited.
    let mut memo: HashMap<usize, u64> = HashMap::new();

    let count = count_paths(IDENTIFIER_START, &graph_map, &mut memo);

    Ok(count.to_string())
}

// Recursive function to find all paths from current_node to IDENTIFIER_END
fn count_paths(
    current_node: usize, graph: &HashMap<usize, Vec<usize>>,
    memo: &mut HashMap<usize, u64>,
) -> u64 {
    // Base case: If we reached "out", we found 1 valid path.
    if current_node == IDENTIFIER_END {
        return 1;
    }

    // Check if we have already calculated the result for this node.
    if let Some(&count) = memo.get(&current_node) {
        return count;
    }

    let mut total_paths = 0;

    // Iterate over all connected output devices
    if let Some(neighbors) = graph.get(&current_node) {
        for &neighbor in neighbors {
            total_paths += count_paths(neighbor, graph, memo);
        }
    }

    // Store the result in memo before returning
    memo.insert(current_node, total_paths);
    total_paths
}

const IDENTIFIER_START: usize = 0;
const IDENTIFIER_END: usize = 1;

fn parse_input(input: String) -> Result<HashMap<usize, Vec<usize>>, Day11Error> {
    let mut graph_map: HashMap<usize, Vec<usize>> = HashMap::new();

    let mut identifiers: HashMap<String, usize> = HashMap::new();

    identifiers.insert("you".to_string(), IDENTIFIER_START);
    identifiers.insert("out".to_string(), IDENTIFIER_END);

    // Because "you" and "out" are already taken
    let mut id_counter: usize = 2;

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
        let input = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";

        let actual = process(input.to_string()).unwrap();
        let expected = "5";

        assert_eq!(actual, expected);
    }
}
