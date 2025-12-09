use crate::days::day09::errors::Day09Error;
use crate::errors::Error;
use std::str::FromStr;

// Answer: 4749929916

pub fn process(input: String) -> Result<String, Error> {
    let tiles = parse_input(&input)?;
    let area = find_largest_area(&tiles);

    Ok(area.to_string())
}

fn find_largest_area(tiles: &[Tile]) -> usize {
    let mut largest_area = 0;

    for i in 0..tiles.len() {
        for j in (i + 1)..tiles.len() {
            let area = tiles[i].area(&tiles[j]);
            if area > largest_area {
                largest_area = area;
            }
        }
    }

    largest_area
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

pub struct Tile {
    x: usize,
    y: usize,
}

impl Tile {
    pub fn area(&self, other_corner: &Self) -> usize {
        ((isize::abs(other_corner.x as isize - self.x as isize) + 1)
            * (isize::abs(other_corner.y as isize - self.y as isize) + 1))
            as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

        let actual = process(input.to_string()).unwrap();
        let expected = "50";

        assert_eq!(actual, expected);
    }
}
