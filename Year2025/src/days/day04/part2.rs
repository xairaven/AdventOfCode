use crate::days::day04::errors::Day04Error;
use crate::errors::Error;

// Answer: 9182

type Grid = Vec<Vec<Item>>;

pub fn process(input: String) -> Result<String, Error> {
    let grid = create_item_grid(input)?;
    let amount_removed_rolls = remove_count_rolls(grid)?;
    Ok(amount_removed_rolls.to_string())
}

fn create_item_grid(input: String) -> Result<Grid, Day04Error> {
    let mut grid: Vec<Vec<Item>> = vec![];
    for line in input.lines() {
        let mut row: Vec<Item> = Vec::with_capacity(line.len());

        for symbol in line.chars() {
            let item = Item::try_from(&symbol)?;
            row.push(item);
        }

        grid.push(row);
    }

    Ok(grid)
}

fn find_amount_of_adjacent_rolls(
    grid: &Grid, row: usize, column: usize,
) -> Result<usize, Day04Error> {
    let rows = grid.len();
    let columns = grid.first().ok_or(Day04Error::InvalidGridSize)?.len();

    const DIRECTIONS: [(i16, i16); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let mut sum = 0;
    for direction in DIRECTIONS.iter() {
        let item_row = match usize::try_from(direction.0 + row as i16) {
            Ok(value) => value,
            Err(_) => continue,
        };
        let item_column = match usize::try_from(direction.1 + column as i16) {
            Ok(value) => value,
            Err(_) => continue,
        };

        if item_row >= rows || item_column >= columns {
            continue;
        }

        if let Item::RollOfPaper = grid[item_row][item_column] {
            sum += 1;
        }
    }

    Ok(sum)
}

fn remove_count_rolls(mut grid: Grid) -> Result<usize, Day04Error> {
    let mut sum = 0;

    loop {
        let rolls_to_remove = rolls_to_remove(&grid)?;
        let amount_to_remove = rolls_to_remove.len();
        if amount_to_remove == 0 {
            break;
        }

        for (row_index, column_index) in rolls_to_remove {
            grid[row_index][column_index] = Item::Empty;
        }

        sum += amount_to_remove;
    }

    Ok(sum)
}

fn rolls_to_remove(grid: &Grid) -> Result<Vec<(usize, usize)>, Day04Error> {
    let mut to_remove: Vec<(usize, usize)> = vec![];

    const MAX_ADJACENT_ROLLS: usize = 3;
    for (row_index, row) in grid.iter().enumerate() {
        for (column_index, item) in row.iter().enumerate() {
            if let Item::Empty = item {
                continue;
            }

            let adjacent_rolls =
                find_amount_of_adjacent_rolls(grid, row_index, column_index)?;
            if adjacent_rolls <= MAX_ADJACENT_ROLLS {
                to_remove.push((row_index, column_index));
            }
        }
    }

    Ok(to_remove)
}

pub enum Item {
    RollOfPaper,
    Empty,
}

impl TryFrom<&char> for Item {
    type Error = Day04Error;

    fn try_from(value: &char) -> Result<Self, Self::Error> {
        match value {
            '@' => Ok(Item::RollOfPaper),
            '.' => Ok(Item::Empty),
            _ => Err(Day04Error::UnknownItem(value.to_owned())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
        let expected = 43.to_string();

        let result = process(input.to_string()).unwrap();
        assert_eq!(result, expected);
    }
}
