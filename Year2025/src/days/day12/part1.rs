use crate::days::day12::errors::Day12Error;
use crate::errors::Error;

// Answer: 403

pub fn process(input: String) -> Result<String, Error> {
    let (shapes, queries) = parse_input(&input)?;

    // Generate all geometric variations (rotations/flips) for base shapes
    let shape_variations: Vec<Vec<ShapePoints>> =
        shapes.iter().map(generate_variations).collect();

    let mut solvable_count = 0;

    for (w, h, requirements) in queries {
        // Prepare the list of tasks (presents to fit)
        let mut presents_to_fit = Vec::new();
        let mut total_presents_area = 0;

        for (shape_id, count) in requirements.iter().enumerate() {
            let area = shapes[shape_id].len();
            for _ in 0..*count {
                presents_to_fit.push(shape_id);
                total_presents_area += area;
            }
        }

        // Optimization: Sort by area (descending). Big pieces first = fail faster.
        presents_to_fit.sort_by(|a, b| {
            let size_a = shapes[*a].len();
            let size_b = shapes[*b].len();
            size_b.cmp(&size_a).then(a.cmp(b))
        });

        // Pre-compile shapes into "Linear Offsets" for this specific grid width.
        let compiled_variations: Vec<Vec<CompiledShape>> = shape_variations
            .iter()
            .map(|vars| vars.iter().map(|v| compile_shape(v, w, h)).collect())
            .collect();

        let mut grid = vec![false; w * h];
        let total_cells = w * h;

        // Group static context to reduce arguments
        let context = SolverContext {
            presents: &presents_to_fit,
            variations: &compiled_variations,
        };

        if solve(
            &context,
            total_cells,
            0, // present index
            &mut grid,
            0, // last pos (symmetry breaking)
            total_presents_area,
        ) {
            solvable_count += 1;
        }
    }

    Ok(solvable_count.to_string())
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Copy, PartialOrd, Ord)]
struct Point {
    r: i32,
    c: i32,
}

type ShapePoints = Vec<Point>;
type ParseResult =
    Result<(Vec<ShapePoints>, Vec<(usize, usize, Vec<usize>)>), Day12Error>;

// A shape compiled for a specific grid width
#[derive(Debug, Clone)]
struct CompiledShape {
    // Indices relative to the "anchor" (top-left) of the shape.
    offsets: Vec<usize>,
    // A list of grid indices where this shape can validly be placed
    valid_anchors: Vec<usize>,
    area: usize,
}

struct SolverContext<'a> {
    presents: &'a [usize],
    variations: &'a [Vec<CompiledShape>],
}

fn solve(
    ctx: &SolverContext, empty_cells: usize, present_idx: usize, grid: &mut [bool],
    last_pos_index: usize, required_area: usize,
) -> bool {
    // Base Case: Success
    if present_idx == ctx.presents.len() {
        return true;
    }

    // Pruning: Not enough space left physically
    if empty_cells < required_area {
        return false;
    }

    let shape_id = ctx.presents[present_idx];
    let possible_shapes = &ctx.variations[shape_id];

    // All variations of the same shape have the same area
    let current_piece_area = possible_shapes.first().map(|s| s.area).unwrap_or(0);

    // Symmetry breaking: identical pieces must be placed in order of grid index.
    let start_search_after = if present_idx > 0
        && ctx.presents[present_idx] == ctx.presents[present_idx - 1]
    {
        last_pos_index
    } else {
        0
    };

    for shape in possible_shapes {
        for &anchor in &shape.valid_anchors {
            if anchor < start_search_after {
                continue;
            }

            // Optimization: Check the anchor point first
            if grid[anchor] {
                continue;
            }

            // Collision Check using pre-computed linear offsets
            let mut collision = false;
            for &offset in &shape.offsets {
                if grid[anchor + offset] {
                    collision = true;
                    break;
                }
            }

            if !collision {
                // Place
                for &offset in &shape.offsets {
                    grid[anchor + offset] = true;
                }

                // Recurse
                if solve(
                    ctx,
                    empty_cells - current_piece_area,
                    present_idx + 1,
                    grid,
                    anchor, // Pass current anchor for symmetry constraint
                    required_area - current_piece_area,
                ) {
                    return true;
                }

                // Backtrack (Unplace)
                for &offset in &shape.offsets {
                    grid[anchor + offset] = false;
                }
            }
        }
    }

    false
}

fn compile_shape(points: &ShapePoints, grid_w: usize, grid_h: usize) -> CompiledShape {
    let max_r = points.iter().map(|p| p.r).max().unwrap_or(0) as usize;
    let max_c = points.iter().map(|p| p.c).max().unwrap_or(0) as usize;

    let shape_h = max_r + 1;
    let shape_w = max_c + 1;

    let offsets: Vec<usize> = points
        .iter()
        .map(|p| (p.r as usize) * grid_w + (p.c as usize))
        .collect();

    let mut valid_anchors = Vec::new();

    if shape_h <= grid_h && shape_w <= grid_w {
        for r in 0..=(grid_h - shape_h) {
            for c in 0..=(grid_w - shape_w) {
                valid_anchors.push(r * grid_w + c);
            }
        }
    }

    CompiledShape {
        offsets,
        valid_anchors,
        area: points.len(),
    }
}

fn parse_input(input: &str) -> ParseResult {
    let mut shapes = Vec::new();
    let mut queries = Vec::new();
    let parts: Vec<&str> = input.trim().split("\n\n").collect();

    for block in parts {
        let first_line = block.lines().next().unwrap_or("");

        if block.contains("x") && block.contains(":") && !first_line.contains('#') {
            for line in block.lines() {
                if line.is_empty() {
                    continue;
                }
                let (dim, counts) =
                    line.split_once(":").ok_or(Day12Error::InvalidQueryFormat)?;
                let (w_s, h_s) = dim
                    .split_once("x")
                    .ok_or(Day12Error::InvalidDimensionFormat)?;
                let w: usize = w_s.trim().parse()?;
                let h: usize = h_s.trim().parse()?;
                let c: Vec<usize> = counts
                    .split_whitespace()
                    .map(|n| n.parse::<usize>())
                    .collect::<Result<_, _>>()?;
                queries.push((w, h, c));
            }
        } else {
            let mut points = Vec::new();
            let mut lines = block.lines();
            let _ = lines.next();
            for (r, line) in lines.enumerate() {
                for (c, ch) in line.chars().enumerate() {
                    if ch == '#' {
                        points.push(Point {
                            r: r as i32,
                            c: c as i32,
                        });
                    }
                }
            }
            if !points.is_empty() {
                normalize(&mut points);
                shapes.push(points);
            }
        }
    }
    Ok((shapes, queries))
}

fn normalize(points: &mut ShapePoints) {
    if points.is_empty() {
        return;
    }
    let min_r = points.iter().map(|p| p.r).min().unwrap_or(0);
    let min_c = points.iter().map(|p| p.c).min().unwrap_or(0);
    for p in points.iter_mut() {
        p.r -= min_r;
        p.c -= min_c;
    }
    points.sort();
}

fn rotate(shape: &ShapePoints) -> ShapePoints {
    let mut s: ShapePoints = shape.iter().map(|p| Point { r: p.c, c: -p.r }).collect();
    normalize(&mut s);
    s
}

fn flip(shape: &ShapePoints) -> ShapePoints {
    let mut s: ShapePoints = shape.iter().map(|p| Point { r: p.r, c: -p.c }).collect();
    normalize(&mut s);
    s
}

fn generate_variations(base: &ShapePoints) -> Vec<ShapePoints> {
    let mut vars = Vec::new();
    let mut curr = base.clone();
    normalize(&mut curr);
    for _ in 0..4 {
        vars.push(curr.clone());
        vars.push(flip(&curr));
        curr = rotate(&curr);
    }
    vars.sort();
    vars.dedup();
    vars
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input = "0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2";

        let actual = process(input.to_string()).unwrap();
        let expected = "2";

        assert_eq!(actual, expected);
    }
}
