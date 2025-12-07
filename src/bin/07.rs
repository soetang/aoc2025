advent_of_code::solution!(7);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    /// '.' open space
    Open,
    /// '^' splitter
    Splitter,
    /// 'S' starting point (also treated as open for movement)
    Start,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Grid {
    width: usize,
    height: usize,
    cells: Vec<Cell>,
    start: Option<(usize, usize)>, // (x, y)
}

impl Grid {
    fn idx(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    fn get(&self, x: usize, y: usize) -> Option<Cell> {
        if x < self.width && y < self.height {
            Some(self.cells[self.idx(x, y)])
        } else {
            None
        }
    }
}

fn parse(input: &str) -> Grid {
    let mut cells = Vec::new();
    let mut width = 0usize;
    let mut height = 0usize;
    let mut start: Option<(usize, usize)> = None;

    for (y, line) in input.lines().enumerate() {
        // Skip completely empty lines (common in AoC inputs)
        if line.trim().is_empty() {
            continue;
        }
        height += 1;
        width = width.max(line.len());
        for (x, ch) in line.chars().enumerate() {
            let cell = match ch {
                '.' => Cell::Open,
                '^' => Cell::Splitter,
                'S' => {
                    start = Some((x, y));
                    Cell::Start
                }
                _ => {
                    // Treat any unknown char as open to be resilient
                    Cell::Open
                }
            };
            cells.push(cell);
        }
        // If lines are ragged, pad to width with open cells
        if line.len() < width {
            cells.extend(std::iter::repeat(Cell::Open).take(width - line.len()));
        }
    }

    Grid {
        width,
        height,
        cells,
        start,
    }
}

// Progress beams one row down, returning the next set of beam x-positions and
// the number of splits that occurred in this step.
fn step_beams_row(
    grid: &Grid,
    y: usize,
    current_beams: &std::collections::BTreeSet<usize>,
) -> (std::collections::BTreeSet<usize>, u64) {
    let mut next_beams: std::collections::BTreeSet<usize> = std::collections::BTreeSet::new();
    let mut splits = 0u64;
    if y + 1 >= grid.height {
        return (next_beams, splits);
    }
    for &x in current_beams {
        match grid.get(x, y + 1) {
            Some(Cell::Splitter) => {
                splits += 1;
                if x > 0 {
                    next_beams.insert(x - 1);
                }
                if x + 1 < grid.width {
                    next_beams.insert(x + 1);
                }
            }
            Some(Cell::Open) | Some(Cell::Start) | None => {
                next_beams.insert(x);
            }
        }
    }
    (next_beams, splits)
}

// Simulate full descent counting total splits until beams exit the grid.
fn simulate_total_splits(grid: &Grid) -> u64 {
    let (start_x, start_y) = match grid.start {
        Some(s) => s,
        None => return 0,
    };
    let mut current_beams: std::collections::BTreeSet<usize> = std::collections::BTreeSet::new();
    current_beams.insert(start_x);
    let mut y = start_y;
    let mut total = 0u64;
    while y + 1 < grid.height && !current_beams.is_empty() {
        let (next, splits) = step_beams_row(grid, y, &current_beams);
        total += splits;
        current_beams = next;
        y += 1;
    }
    total
}

// Dynamic programming one-row step: update ways vector for the next row.
fn step_ways_row(grid: &Grid, y: usize, ways: &[u64], next: &mut [u64]) {
    if y + 1 >= grid.height {
        return;
    }
    for x in 0..grid.width {
        let count = ways[x];
        if count == 0 {
            continue;
        }
        match grid.get(x, y + 1) {
            Some(Cell::Splitter) => {
                if x > 0 {
                    next[x - 1] = next[x - 1].saturating_add(count);
                }
                if x + 1 < grid.width {
                    next[x + 1] = next[x + 1].saturating_add(count);
                }
            }
            Some(Cell::Open) | Some(Cell::Start) | None => {
                next[x] = next[x].saturating_add(count);
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let grid = parse(input);
    Some(simulate_total_splits(&grid))
}

pub fn part_two(input: &str) -> Option<u64> {
    let grid = parse(input);
    let (start_x, start_y) = grid.start?;
    let mut ways: Vec<u64> = vec![0; grid.width];
    ways[start_x] = 1;
    let mut y = start_y;
    while y + 1 < grid.height {
        let mut next: Vec<u64> = vec![0; grid.width];
        step_ways_row(&grid, y, &ways, &mut next);
        ways = next;
        y += 1;
    }
    Some(ways.iter().copied().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_basic_grid() {
        let sample = "S.^\n..^\n...\n";
        let g = parse(sample);
        assert_eq!(g.width, 3);
        assert_eq!(g.height, 3);
        assert_eq!(g.start, Some((0, 0)));
        assert_eq!(g.get(1, 0), Some(Cell::Open)); // '.' at (1,0)
        assert_eq!(g.get(2, 0), Some(Cell::Splitter)); // '^' at (2,0)
        assert_eq!(g.get(2, 1), Some(Cell::Splitter));
    }

    #[test]
    fn test_part_one() {
        // Minimal synthetic example: Start above two splitters stacked to cause multiple splits
        // Grid:
    // Row0: S..
    // Row1: ^..
    // Row2: .^.
        // Simulation:
        // y=0 beams {0}; y=1 below(0,1)='^' -> split -> {0-1 invalid, 1} == {1}; splits=1
        // y=1 beams {1}; y=2 below(1,2)='^' -> split -> {0,2}; splits=2
        // y=2 is last row, stop. Total splits=2
    let input = "S..\n^..\n.^.\n";
        let result = part_one(input);
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        // Example demonstrating collisions and path counting
    // Grid (3 rows):
    // Row0: ..S..
    // Row1: ..^..
    // Row2: .^.^.
    // From S at x=2:
    // y=0->1: below(2,1)='^' -> ways at y=1: x=1:1, x=3:1
    // y=1->2: below(1,2)='^' -> split to x=0 and x=2; below(3,2)='^' -> split to x=2 and x=4
    // total at last row = ways at x={0:1,2:2,4:1} -> 4
    let input = "..S..\n..^..\n.^.^.\n";
    let result = part_two(input);
    assert_eq!(result, Some(4));
    }
}
