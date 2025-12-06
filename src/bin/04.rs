advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u64> {
    Some(count_accessible_rolls(input) as u64)
}

pub fn part_two(_input: &str) -> Option<u64> {
    Some(simulate_total_removals(_input) as u64)
}

// Directions for 8-neighbor adjacency
const DIRS8: [(isize, isize); 8] = [
    (-1, -1), (-1, 0), (-1, 1),
    (0, -1),           (0, 1),
    (1, -1),  (1, 0),  (1, 1),
];

// Parse the input into a rectangular byte grid
fn parse_grid(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| line.trim_end().bytes().collect())
        .collect()
}

// Count '@' neighbors around (r,c)
fn adj_rolls(grid: &[Vec<u8>], r: usize, c: usize) -> u8 {
    let h = grid.len();
    let w = grid[0].len();
    let mut adj = 0u8;
    for (dr, dc) in DIRS8.iter() {
        let nr = r as isize + dr;
        let nc = c as isize + dc;
        if nr >= 0 && nr < h as isize && nc >= 0 && nc < w as isize {
            if grid[nr as usize][nc as usize] == b'@' { adj += 1; }
        }
    }
    adj
}

// Count rolls of paper ('@') that have fewer than 4 adjacent rolls
// considering all 8 neighboring positions.
fn count_accessible_rolls(input: &str) -> usize {
    let grid = parse_grid(input);
    let h = grid.len();
    if h == 0 { return 0; }
    let w = grid[0].len();

    let mut total = 0usize;
    for r in 0..h {
        for c in 0..w {
            if grid[r][c] != b'@' { continue; }
            if adj_rolls(&grid, r, c) < 4 { total += 1; }
        }
    }
    total
}

// Repeatedly remove all accessible rolls ('@' with <4 adjacent '@') until none remain.
// Return the total number of rolls removed across all rounds.
fn simulate_total_removals(input: &str) -> usize {
    let mut grid = parse_grid(input);
    let h = grid.len();
    if h == 0 { return 0; }
    let w = grid[0].len();

    // Precompute adjacency counts and use a queue to process removals incrementally.
    let mut deg: Vec<Vec<u8>> = vec![vec![0u8; w]; h];
    for r in 0..h {
        for c in 0..w {
            if grid[r][c] == b'@' {
                deg[r][c] = adj_rolls(&grid, r, c);
            }
        }
    }

    let mut q: Vec<(usize, usize)> = Vec::new();
    for r in 0..h {
        for c in 0..w {
            if grid[r][c] == b'@' && deg[r][c] < 4 {
                q.push((r, c));
            }
        }
    }

    let mut total_removed = 0usize;
    // Process queue: removing a roll reduces degree of neighboring rolls;
    // neighbors may become newly accessible (<4) and join the queue.
    while let Some((r, c)) = q.pop() {
        if grid[r][c] != b'@' { continue; } // might have been removed earlier
        grid[r][c] = b'.';
        total_removed += 1;
        for (dr, dc) in DIRS8.iter() {
            let nr = r as isize + dr;
            let nc = c as isize + dc;
            if nr >= 0 && nr < h as isize && nc >= 0 && nc < w as isize {
                let ur = nr as usize;
                let uc = nc as usize;
                if grid[ur][uc] == b'@' {
                    // This neighbor loses one adjacent roll because (r,c) was removed.
                    if deg[ur][uc] > 0 { deg[ur][uc] -= 1; }
                    if deg[ur][uc] < 4 {
                        q.push((ur, uc));
                    }
                }
            }
        }
    }

    total_removed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_count() {
        let example = "..@@.@@@@.\n@@@.@.@.@@\n@@@@@.@.@@\n@.@@@@..@.\n@@.@@@@.@@\n.@@@@@@@.@\n.@.@.@.@@@\n@.@@@.@@@@\n.@@@@@@@@.\n@.@.@@@.@.\n";
        assert_eq!(count_accessible_rolls(example), 13);
        assert_eq!(part_one(example), Some(13));
    }

    #[test]
    fn example_total_removals() {
        let example = "..@@.@@@@.\n@@@.@.@.@@\n@@@@@.@.@@\n@.@@@@..@.\n@@.@@@@.@@\n.@@@@@@@.@\n.@.@.@.@@@\n@.@@@.@@@@\n.@@@@@@@@.\n@.@.@@@.@.\n";
        assert_eq!(simulate_total_removals(example), 43);
        assert_eq!(part_two(example), Some(43));
    }
}
