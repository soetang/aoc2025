advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    // Count how many times the dial points at 0 after applying each move
    let moves = parse(input);
    let count_zeros = moves
        .into_iter()
        .scan(50u32, |pos, mv| {
            *pos = apply_move(*pos, mv);
            Some(*pos == 0)
        })
        .filter(|hit_zero| *hit_zero)
        .count() as u64;
    Some(count_zeros)
}

pub fn part_two(input: &str) -> Option<u64> {
    let moves = parse(input);
    // Count hits of 0 both during movement and upon landing, including multiple wraps
    let total_hits: u64 = moves
        .into_iter()
        .scan(50u32, |pos, (dir, amt)| {
            let hits = hits_crossing_zero(*pos, dir, amt);
            *pos = apply_move(*pos, (dir, amt));
            Some(hits as u64)
        })
        .sum();
    Some(total_hits)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Dir {
    Left,
    Right,
}

#[inline]
fn parse(input: &str) -> Vec<(Dir, u32)> {
    input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|line| {
            let (d, n) = line.split_at(1);
            let dir = match d {
                "L" => Dir::Left,
                "R" => Dir::Right,
                other => panic!("Invalid direction: {}", other),
            };
            let amt: u32 = n.trim().parse().expect("Invalid number");
            (dir, amt)
        })
        .collect()
}

#[inline]
fn apply_move(pos: u32, mv: (Dir, u32)) -> u32 {
    let (dir, amt) = mv;
    match dir {
        Dir::Left => wrap100(pos as i32 - amt as i32) as u32,
        Dir::Right => wrap100(pos as i32 + amt as i32) as u32,
    }
}

#[inline]
fn wrap100(x: i32) -> i32 {
    // wrap into 0..=99 using Euclidean modulo
    let m = 100;
    let r = x % m;
    if r < 0 { r + m } else { r }
}

// Number of times we hit 0 during a move of size `amt` starting at `pos`, in `dir`
#[inline]
fn hits_crossing_zero(pos: u32, dir: Dir, amt: u32) -> u32 {
    let first = match dir {
        Dir::Right => if pos == 0 { 100 } else { 100 - pos },
        Dir::Left => if pos == 0 { 100 } else { pos },
    };
    if amt < first { 0 } else { 1 + (amt - first) / 100 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        // Sample: hitting 0 should be counted
        // Start 50, R50 -> 0 (count 1), L1 -> 99 (no count), R1 -> 0 (count 2)
        let input = "R50\nL1\nR1\n";
        let result = part_one(input);
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        // Crossing zero multiple times
        // Start 50, R150 -> crosses at 50 (hit 0), and 150 (hit 0) => 2 hits
        // Then L200 -> crosses at 50 and 150 => 2 hits; total 4
        let input = "R150\nL200\n";
        let result = part_two(input);
        assert_eq!(result, Some(4));
    }
}
