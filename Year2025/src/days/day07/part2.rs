use crate::days::day07::errors::Day07Error;
use crate::errors::Error;
use std::collections::HashMap;
use std::str::FromStr;

// Answer: 27055852018812

pub fn process(input: String) -> Result<String, Error> {
    let grid: Grid = input.parse()?;

    let mut memo = HashMap::new();
    let beams = process_path(&grid, grid.start.clone(), &mut memo)?;

    Ok(beams.to_string())
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Coordinates {
    row: usize,
    col: usize,
}

impl From<(usize, usize)> for Coordinates {
    fn from(value: (usize, usize)) -> Self {
        Coordinates {
            row: value.0,
            col: value.1,
        }
    }
}

fn process_path(
    grid: &Grid, start_from: Coordinates, memo: &mut HashMap<Coordinates, usize>,
) -> Result<usize, Day07Error> {
    if let Some(&saved_result) = memo.get(&start_from) {
        return Ok(saved_result);
    }
    let timelines: usize;

    let (mut row, col) = (start_from.row, start_from.col);
    loop {
        row += 1;
        if row >= grid.rows {
            timelines = 1;
            break;
        }

        let element = grid.map[row][col].clone();
        match element {
            Item::Empty => continue,
            Item::Splitter => {
                // Left and right
                let candidate_columns = vec![col as i32 - 1, col as i32 + 1];

                let mut current_split_sum = 0;
                for candidate in candidate_columns {
                    if !is_column_exists(candidate, grid) {
                        continue;
                    }

                    current_split_sum += process_path(
                        grid,
                        Coordinates::from((row, candidate as usize)),
                        memo,
                    )?;
                }
                timelines = current_split_sum;

                break;
            },
            Item::Start => return Err(Day07Error::UnexpectedStart(row, col)),
        }
    }
    memo.insert(start_from, timelines);

    Ok(timelines)
}

fn is_column_exists(index: i32, grid: &Grid) -> bool {
    index >= 0 && index < grid.columns as i32
}

#[derive(Debug)]
pub struct Grid {
    map: Vec<Vec<Item>>,
    rows: usize,
    columns: usize,
    start: Coordinates,
}

impl FromStr for Grid {
    type Err = Day07Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map = Vec::new();

        for line in s.lines() {
            let mut row = Vec::new();
            for ch in line.chars() {
                row.push(Item::try_from(ch)?);
            }
            map.push(row);
        }

        let rows = map.len();
        let columns = if rows > 0 { map[0].len() } else { 0 };
        let mut start_index: Option<usize> = None;
        for row in &map {
            debug_assert_eq!(row.len(), columns);
            if start_index.is_none() {
                for (index, item) in row.iter().enumerate() {
                    if let Item::Start = item {
                        start_index = Some(index);
                        break;
                    }
                }
            }
        }

        let start = Coordinates {
            row: 0,
            col: start_index.ok_or(Day07Error::StartNotFound)?,
        };

        Ok(Grid {
            map,
            rows,
            columns,
            start,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Item {
    Empty,
    Start,
    Splitter,
}

impl TryFrom<char> for Item {
    type Error = Day07Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Item::Empty),
            'S' => Ok(Item::Start),
            '^' => Ok(Item::Splitter),
            _ => Err(Self::Error::InvalidItem(value)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

        let actual = process(input.to_string()).unwrap();
        let expected = "40".to_string();

        assert_eq!(actual, expected);
    }
}
