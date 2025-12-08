use crate::days::day08::errors::Day08Error;
use crate::errors::Error;
use std::collections::HashMap;
use std::str::FromStr;

// Answer: 107256172

pub fn process(input: String) -> Result<String, Error> {
    let junction_boxes = parse_input(&input)?;
    let result = solve(&junction_boxes);

    Ok(result.to_string())
}

fn solve(junction_boxes: &[JunctionBox]) -> usize {
    let mut distances: HashMap<(usize, usize), f64> = HashMap::new();

    for i in 0..junction_boxes.len() {
        for j in (i + 1)..junction_boxes.len() {
            let distance = junction_boxes[i].distance_to(&junction_boxes[j]);
            distances.insert((i, j), distance);
        }
    }

    let mut sorted_distances: Vec<((usize, usize), f64)> =
        distances.into_iter().collect();
    sorted_distances.sort_by(|a, b| a.1.total_cmp(&b.1));

    let mut connections: Vec<usize> = (0..junction_boxes.len()).collect();
    let mut two_last: (usize, usize) = (0, 0);

    for ((i, j), _) in sorted_distances {
        if connections[i] != connections[j] {
            let (new, old) = (connections[i], connections[j]);
            for connection in connections.iter_mut() {
                if *connection == old {
                    *connection = new;
                }
            }
            two_last = (i, j);
        }
    }

    let last_connected = (
        junction_boxes[two_last.0].clone(),
        junction_boxes[two_last.1].clone(),
    );

    last_connected.0.x * last_connected.1.x
}

fn parse_input(input: &str) -> Result<Vec<JunctionBox>, Day08Error> {
    let mut junction_boxes = Vec::new();

    for line in input.lines() {
        let parts_str: Vec<&str> = line.split(',').collect();
        if parts_str.len() != 3 {
            return Err(Day08Error::InvalidInputFormat(line.to_string()));
        }

        let mut parts: [usize; 3] = [0; 3];
        for (i, part) in parts_str.iter().enumerate() {
            parts[i] =
                usize::from_str(part.trim()).map_err(Day08Error::InvalidCoordinate)?;
        }

        junction_boxes.push(JunctionBox {
            x: parts[0],
            y: parts[1],
            z: parts[2],
        });
    }

    Ok(junction_boxes)
}

#[derive(Debug, Clone)]
pub struct JunctionBox {
    pub x: usize,
    pub y: usize,
    pub z: usize,
}

impl JunctionBox {
    pub fn distance_to(&self, other: &Self) -> f64 {
        let distance = ((self.x as isize - other.x as isize).pow(2)
            + (self.y as isize - other.y as isize).pow(2)
            + (self.z as isize - other.z as isize).pow(2)) as f64;

        distance.sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

        let actual = process(input.to_string()).unwrap();
        let expected = "25272".to_string();

        assert_eq!(actual, expected);
    }
}
