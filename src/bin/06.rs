advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u64> {
    Some(parse_columns(input).iter().map(eval_column).sum::<i128>() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(
        parse_columns(input)
            .iter()
            .map(|c| apply_symbol(numbers_by_position(c).into_iter(), c.op))
            .sum::<i128>() as u64,
    )
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Column { values: Vec<i64>, op: char, rows: Vec<String> }

fn apply_symbol(nums: impl IntoIterator<Item = i128>, op: char) -> i128 {
    match op {
        '+'=> nums.into_iter().sum(),
        '*'=> nums.into_iter().fold(1, |a, v| a * v),
        _  => panic!("unknown operator: {}", op),
    }
}

// Find contiguous non-space spans across all rows, then extract numbers and the last op in each span.
fn parse_columns(input: &str) -> Vec<Column> {
    let lines: Vec<&str> = input.lines().collect();
    let w = lines.iter().map(|l| l.len()).max().unwrap_or(0);
    let grid: Vec<Vec<char>> = lines
        .iter()
        .map(|l| {
            let mut r: Vec<char> = l.chars().collect();
            r.resize(w, ' ');
            r
        })
        .collect();

    // Compute spans of contiguous columns that have any non-space
    let mut spans = Vec::<(usize, usize)>::new();
    let mut i = 0usize;
    while i < w {
        while i < w && !grid.iter().any(|row| row[i] != ' ') { i += 1; }
        if i >= w { break; }
        let s = i;
        while i < w && grid.iter().any(|row| row[i] != ' ') { i += 1; }
        spans.push((s, i));
    }

    spans
        .into_iter()
        .map(|(s, e)| {
            let mut vals = Vec::new();
            let mut rows = Vec::new();
            let mut last = '+';
            for r in &grid {
                let slice: String = r[s..e].iter().collect();
                let t = slice.trim();
                if t.is_empty() { continue; }
                if let Ok(v) = t.parse::<i64>() {
                    vals.push(v);
                    rows.push(slice);
                } else {
                    // Only update op when the token isn't a number
                    last = t.chars().find(|c| !c.is_whitespace()).unwrap_or(last);
                }
            }
            Column { values: vals, op: last, rows }
        })
        .collect()
}

fn eval_column(col: &Column) -> i128 { apply_symbol(col.values.iter().copied().map(|v| v as i128), col.op) }

// Build numbers by digit position (right-to-left) across the fixed-width rows.
fn numbers_by_position(col: &Column) -> Vec<i128> {
    if col.rows.is_empty() { return vec![]; }
    let width = col.rows.iter().map(|s| s.len()).max().unwrap_or(0);

    let last_digit_idx: Vec<Option<usize>> = col
        .rows
        .iter()
        .map(|r| r.chars().enumerate().filter(|(_, c)| c.is_ascii_digit()).map(|(i, _)| i).last())
        .collect();

    let build_at_pos = |pos: usize| -> Option<i128> {
        let mut acc = 0i128;
        let mut any = false;
        for (ri, r) in col.rows.iter().enumerate() {
            match r.chars().nth(pos) {
                Some(ch) if ch.is_ascii_digit() => { acc = acc * 10 + ch.to_digit(10).unwrap() as i128; any = true; }
                _ if col.op == '*' => {
                    if let Some(ld) = last_digit_idx[ri] { if pos > ld { acc *= 10; any = true; } }
                }
                _ => {}
            }
        }
        if any { Some(acc) } else { None }
    };

    (0..width).rev().filter_map(build_at_pos).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   + \n";

    #[test]
    fn day6_part1() { assert_eq!(part_one(INPUT), Some(4_277_556)); }

    #[test]
    fn day6_part2() { assert_eq!(part_two(INPUT), Some(3_263_827)); }
}
