use crate::days::day07::errors::Day07Error;
use crate::errors::Error;
use std::str::FromStr;

// Answer: 1628

pub fn process(input: String) -> Result<String, Error> {
    let mut grid: Grid = input.parse()?;

    let start_index = grid.start_index;
    let beams = process_path(&mut grid, (0, start_index))?;

    Ok(beams.to_string())
}

fn process_path(
    grid: &mut Grid, start_from: (usize, usize),
) -> Result<usize, Day07Error> {
    let mut splits: usize = 0;

    let (mut row, col) = start_from;
    loop {
        row += 1; // Move down one row

        if row >= grid.rows {
            break; // Reached the bottom of the grid
        }

        let element = grid.map[row][col].clone();
        match element {
            Item::Empty => {
                grid.map[row][col] = Item::Beam;
            },
            Item::Splitter => {
                // Left and right
                let candidate_columns = vec![col as i32 - 1, col as i32 + 1];
                for candidate in candidate_columns {
                    if !is_column_exists(candidate, grid) {
                        continue;
                    }

                    let side_element = &grid.map[row][candidate as usize];
                    if side_element.eq(&Item::Beam) {
                        continue;
                    }

                    grid.map[row][col] = Item::Beam;
                    let inner_splits = process_path(grid, (row, candidate as usize))?;
                    splits += inner_splits;
                }

                splits += 1;
                break;
            },
            Item::Start => return Err(Day07Error::UnexpectedStart(row, col)),

            // Another beam found, stop processing
            Item::Beam => return Ok(splits),
        }
    }

    Ok(splits)
}

fn is_column_exists(index: i32, grid: &Grid) -> bool {
    index >= 0 && index < grid.columns as i32
}

#[derive(Debug)]
pub struct Grid {
    map: Vec<Vec<Item>>,
    rows: usize,
    columns: usize,
    start_index: usize,
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

        let start_index = start_index.ok_or(Day07Error::StartNotFound)?;

        Ok(Grid {
            map,
            rows,
            columns,
            start_index,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Item {
    Empty,
    Start,
    Splitter,

    // Reserved for task
    Beam,
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
        let expected = "21".to_string();

        assert_eq!(actual, expected);
    }
}
