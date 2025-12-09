use crate::days::day09::errors::Day09Error;
use crate::errors::Error;
use std::collections::{BTreeSet, VecDeque};
use std::str::FromStr;

// Answer: 1572047142

pub fn process(input: String) -> Result<String, Error> {
    let red_tiles = parse_input(&input)?;
    let result = solve(&red_tiles)?;
    Ok(result.to_string())
}

fn solve(red_tiles: &[Tile]) -> Result<usize, Error> {
    if red_tiles.is_empty() {
        return Ok(0);
    }

    // Coordinate Compression
    // Collect unique sorted coordinates
    let unique_x: Vec<usize> = red_tiles
        .iter()
        .map(|t| t.x)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();

    let unique_y: Vec<usize> = red_tiles
        .iter()
        .map(|t| t.y)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();

    // Grid dimensions
    let width = unique_x.len() * 2 + 1;
    let height = unique_y.len() * 2 + 1;

    let mut grid = vec![vec![0; width]; height];

    // Draw Boundaries on Compressed Grid
    let len = red_tiles.len();
    for i in 0..len {
        let t1 = &red_tiles[i];
        let t2 = &red_tiles[(i + 1) % len];

        let x1_idx = find_in_set(t1.x, &unique_x)? * 2 + 1;
        let y1_idx = find_in_set(t1.y, &unique_y)? * 2 + 1;
        let x2_idx = find_in_set(t2.x, &unique_x)? * 2 + 1;
        let y2_idx = find_in_set(t2.y, &unique_y)? * 2 + 1;

        if t1.x == t2.x {
            let start_y = y1_idx.min(y2_idx);
            let end_y = y1_idx.max(y2_idx);
            for row in grid.iter_mut().take(end_y + 1).skip(start_y) {
                row[x1_idx] = 1;
            }
        } else {
            let start_x = x1_idx.min(x2_idx);
            let end_x = x1_idx.max(x2_idx);
            for cell in grid[y1_idx].iter_mut().take(end_x + 1).skip(start_x) {
                *cell = 1;
            }
        }
    }

    // Flood Fill
    let mut queue = VecDeque::new();
    queue.push_back((0, 0));
    grid[0][0] = -1;

    let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    while let Some((cx, cy)) = queue.pop_front() {
        for (dx, dy) in directions {
            let nx = cx as isize + dx;
            let ny = cy as isize + dy;

            if nx >= 0 && nx < width as isize && ny >= 0 && ny < height as isize {
                let nx = nx as usize;
                let ny = ny as usize;
                if grid[ny][nx] == 0 {
                    grid[ny][nx] = -1;
                    queue.push_back((nx, ny));
                }
            }
        }
    }

    // Prefix Sums
    let mut prefix_sum = vec![vec![0; width + 1]; height + 1];

    for y in 0..height {
        for x in 0..width {
            let is_invalid = if grid[y][x] == -1 { 1 } else { 0 };
            prefix_sum[y + 1][x + 1] =
                is_invalid + prefix_sum[y][x + 1] + prefix_sum[y + 1][x]
                    - prefix_sum[y][x];
        }
    }

    // Helper: Count invalid cells
    let check_compressed_rect_valid =
        |x1_idx: usize, y1_idx: usize, x2_idx: usize, y2_idx: usize| -> bool {
            let min_x = x1_idx.min(x2_idx);
            let max_x = x1_idx.max(x2_idx);
            let min_y = y1_idx.min(y2_idx);
            let max_y = y1_idx.max(y2_idx);

            let rx = max_x + 1;
            let ry = max_y + 1;
            let lx = min_x;
            let ly = min_y;

            let invalid_count = (prefix_sum[ry][rx] + prefix_sum[ly][lx])
                - prefix_sum[ry][lx]
                - prefix_sum[ly][rx];

            invalid_count == 0
        };

    // Check all pairs
    let mut max_area = 0;

    for i in 0..len {
        for j in (i + 1)..len {
            let t1 = &red_tiles[i];
            let t2 = &red_tiles[j];

            let x1_grid = find_in_set(t1.x, &unique_x)? * 2 + 1;
            let y1_grid = find_in_set(t1.y, &unique_y)? * 2 + 1;
            let x2_grid = find_in_set(t2.x, &unique_x)? * 2 + 1;
            let y2_grid = find_in_set(t2.y, &unique_y)? * 2 + 1;

            if check_compressed_rect_valid(x1_grid, y1_grid, x2_grid, y2_grid) {
                let area = t1.area(t2);
                if area > max_area {
                    max_area = area;
                }
            }
        }
    }

    Ok(max_area)
}

fn find_in_set(value: usize, unique: &[usize]) -> Result<usize, Day09Error> {
    unique
        .binary_search(&value)
        .map_err(Day09Error::MissingCoordinateInSet)
}

fn parse_input(input: &str) -> Result<Vec<Tile>, Day09Error> {
    let mut tiles = Vec::new();

    for line in input.lines() {
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() != 2 {
            return Err(Day09Error::InvalidInputFormat);
        }

        let mut coordinates: [usize; 2] = [0; 2];
        for (i, coordinate) in parts.iter().enumerate() {
            coordinates[i] = usize::from_str(coordinate.trim())
                .map_err(Day09Error::InvalidCoordinate)?;
        }

        tiles.push(Tile {
            x: coordinates[0],
            y: coordinates[1],
        });
    }

    Ok(tiles)
}

#[derive(Debug)]
pub struct Tile {
    x: usize,
    y: usize,
}

impl Tile {
    pub fn area(&self, other_corner: &Self) -> usize {
        // Use inclusive calculation: (|x2-x1| + 1) * (|y2-y1| + 1)
        ((isize::abs(other_corner.x as isize - self.x as isize) + 1)
            * (isize::abs(other_corner.y as isize - self.y as isize) + 1))
            as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2_example() {
        let input = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";
        let actual = process(input.to_string()).unwrap();
        assert_eq!(actual, "24");
    }
}
