advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    let grid = parse(input);
    Some(simulate_total_splits(&grid))
}

pub fn part_two(input: &str) -> Option<u64> {
    let grid = parse(input);
    let (start_x, start_y) = grid.start?;
    let mut ways = vec![0u32; grid.width];
    ways[start_x] = 1;
    let mut next = vec![0u32; grid.width];

    for y in start_y..grid.height.saturating_sub(1) {
        // Compute next row counts into preallocated buffer
        next.fill(0);
        step_ways_row(&grid, y, &ways, &mut next);
        // Early exit if no paths remain
        if next.iter().all(|&v| v == 0) {
            ways.clear(); // empty to sum to 0
            break;
        }
        std::mem::swap(&mut ways, &mut next);
    }

    Some(ways.iter().map(|&v| v as u64).sum())
}

fn parse(input: &str) -> Grid {
    // Collect non-empty lines first to determine a consistent width.
    let lines: Vec<&str> = input
        .lines()
        .collect();

    let height = lines.len();
    let width = lines.first().map(|l| l.len()).unwrap_or(0);

    // If lines are ragged, we choose to treat unknowns as error-resilient open spaces,
    // but we do NOT pad; instead, we only read within each line's actual length.
    // For AoC inputs, lines are typically rectangular; if not, indices outside a line are "Open".
    let mut cells = Vec::with_capacity(width * height);
    let mut start: Option<(usize, usize)> = None;

    for (y, &line) in lines.iter().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            let cell = match ch {
                '.' => Cell::Open,
                '^' => Cell::Splitter,
                'S' => {
                    start = Some((x, y));
                    Cell::Start
                }
                _ => Cell::Open,
            };
            cells.push(cell);
        }
        // If this line is shorter than width, fill the remainder with Open to keep a true matrix.
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Open,
    Splitter,
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



// Progress beams one row down in a functional style, returning next positions and split count.
fn step_beams_row(
    grid: &Grid,
    y: usize,
    current_beams: &std::collections::HashSet<usize>,
) -> (std::collections::HashSet<usize>, u32) {
    if y + 1 >= grid.height {
        return (std::collections::HashSet::new(), 0);
    }
    let (next_vec, splits): (Vec<usize>, u32) = current_beams
        .iter()
        .fold((Vec::new(), 0u32), |(mut acc, s), &x| {
            match grid.get(x, y + 1) {
                Some(Cell::Splitter) => {
                    if x > 0 { acc.push(x - 1); }
                    if x + 1 < grid.width { acc.push(x + 1); }
                    (acc, s + 1)
                }
                _ => {
                    acc.push(x);
                    (acc, s)
                }
            }
        });

    let next_beams: std::collections::HashSet<usize> = next_vec.into_iter().collect();
    (next_beams, splits)
}

// Simulate full descent counting total splits until beams exit the grid.
fn simulate_total_splits(grid: &Grid) -> u64 {
    let (start_x, start_y) = match grid.start {
        Some(s) => s,
        None => return 0,
    };
    (start_y..grid.height.saturating_sub(1))
        .scan(
            {
                let mut s = std::collections::HashSet::new();
                s.insert(start_x);
                s
            },
            |beams, y| {
                let (next, splits) = step_beams_row(grid, y, beams);
                *beams = next;
                Some(splits as u64)
            },
        )
        .sum()
}

// Dynamic programming one-row step: update ways vector for the next row.
fn step_ways_row(grid: &Grid, y: usize, ways: &[u32], next: &mut [u32]) {
    if y + 1 >= grid.height {
        return;
    }
    (0..grid.width)
        .filter(|&x| ways[x] > 0)
        .for_each(|x| match grid.get(x, y + 1) {
            Some(Cell::Splitter) => {
                let cnt = ways[x];
                if x > 0 {
                    next[x - 1] = next[x - 1].saturating_add(cnt);
                }
                if x + 1 < grid.width {
                    next[x + 1] = next[x + 1].saturating_add(cnt);
                }
            }
            _ => {
                next[x] = next[x].saturating_add(ways[x]);
            }
        });
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
