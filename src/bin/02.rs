advent_of_code::solution!(2);



pub fn part_one(input: &str) -> Option<u64> {
    // Use arithmetic fast path for two equal halves
    Some(sum_over_ranges(input, Some(2)))
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(sum_over_ranges(input, None))
}

// Shared functional: sum IDs across ranges that satisfy repetition rule
fn sum_over_ranges(input: &str, exact_reps: Option<usize>) -> u64 {
    parse_ranges(input)
        .into_iter()
        .flat_map(|r| r.start..=r.end)
        .filter(|&n| is_repetition_general(n, exact_reps))
        .map(|n| n as u64)
        .sum()
}

fn parse_ranges(input: &str) -> Vec<Range> {
    // Input is a single line: "11-22,25-66,..."
    let mut out = Vec::new();
    let line = input.trim();
    if line.is_empty() {
        return out;
    }
    for part in line.split(',') {
        let part = part.trim();
        if let Some((a, b)) = part.split_once('-') {
            if let (Ok(s), Ok(e)) = (a.trim().parse::<i64>(), b.trim().parse::<i64>()) {
                let (start, end) = if s <= e { (s, e) } else { (e, s) };
                out.push(Range { start, end });
            }
        }
    }
    out
}

// Note: Overlap handling is unnecessary per current requirements.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Range {
    start: i64,
    end: i64,
}

// General repetition check: if exact_reps is Some(k), require exactly k repeats;
// if exact_reps is None, allow any k >= 2.
fn is_repetition_general(n: i64, exact_reps: Option<usize>) -> bool {
    let s = n.to_string();
    let bytes = s.as_bytes();
    let len = bytes.len();
    match exact_reps {
        Some(k) => {
            if k < 2 || len % k != 0 {
                return false;
            }
            let unit = len / k;
            is_repetition_with_unit_bytes(bytes, unit)
        }
        None => {
            // Fast path: unit = 1 (all digits identical) checked first
            if bytes.len() >= 2 && bytes[1..].iter().all(|&b| b == bytes[0]) {
                return true;
            }
            (2..=len / 2)
                .filter(|&unit| len % unit == 0)
                .any(|unit| is_repetition_with_unit_bytes(bytes, unit))
        }
    }
}

fn is_repetition_with_unit_bytes(bytes: &[u8], unit: usize) -> bool {
    let len = bytes.len();
    if unit == 0 || len % unit != 0 {
        return false;
    }
    // Compare each chunk to the first unit using byte equality without creating substrings.
    (unit..len)
        .step_by(unit)
        .all(|i| &bytes[i..i + unit] == &bytes[0..unit])
}

// Fast path for part one (two halves) using arithmetic to avoid string allocation where possible.
// Note: We still use the generalized function in part_one for code clarity, but this helper is
// available if we want to micro-opt in tight loops.
// This is just a copilot suggestion that i tested, it works and is a bit faster
#[allow(dead_code)]
fn is_two_halves_arith(n: i64) -> bool {
    // Determine decimal length
    if n <= 0 { return false; }
    let mut tmp = n;
    let mut len = 0;
    while tmp > 0 { tmp /= 10; len += 1; }
    if len % 2 != 0 { return false; }
    // Compute pow10^(len/2)
    let half = len / 2;
    let mut pow10 = 1i64;
    for _ in 0..half { pow10 *= 10; }
    let left = n / pow10;
    let right = n % pow10;
    left == right
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code::template::Day;

    const DAY: Day = advent_of_code::day!(2);

    #[test]
    fn test_part_one() {
        // Non-overlapping ranges include 11 and 33 as invalids; sum should be 44
        let input = "10-15,30-35";
        let result = part_one(input);
        assert_eq!(result, Some(44)); // 11 + 33
    }

    #[test]
    fn detect_overlap_returns_none() {
        // Overlaps are allowed now; ensure overlapping inputs still compute correctly.
        let input = "10-20,15-25"; // overlap
        // Invalids between 10-25 are 11 and 22 => sum 33
        let result = part_one(input);
        assert_eq!(result, Some(33));
    }

    #[test]
    fn test_part_two() {
        // Examples: 111 (1 repeated 3x), 121212 (12 repeated 3x), 33 (3 repeated 2x)
        let input = "110-123,120-125,30-35";
        // Repetition IDs here: 111, 121212 not present, 33; also 121212 is outside range
        let result = part_two(input);
        assert_eq!(result, Some(111 + 33));
    }

    #[test]
    fn parse_simple_line() {
        let input = "11-22,25-66";
        let ranges = parse_ranges(input);
        assert_eq!(
            ranges,
            vec![
                Range { start: 11, end: 22 },
                Range { start: 25, end: 66 },
            ]
        );
    }

    #[test]
    fn parse_whitespace() {
        let input = " 11-22, 25-66 , 90-10 ";
        let ranges = parse_ranges(input);
        assert_eq!(
            ranges,
            vec![
                Range { start: 11, end: 22 },
                Range { start: 25, end: 66 },
                // reversed inputs normalize to start <= end
                Range { start: 10, end: 90 },
            ]
        );
    }

    #[test]
    fn invalid_examples_even_halves() {
        assert!(is_repetition_general(55, Some(2)));
        assert!(is_repetition_general(6464, Some(2)));
        assert!(is_repetition_general(123123, Some(2)));
        assert!(!is_repetition_general(101, Some(2))); // odd length -> valid ID but not invalid per rule
    }

    #[test]
    fn repetition_examples_varied_lengths() {
        assert!(is_repetition_general(111, None)); // 1 x3
        assert!(is_repetition_general(1212121212, None)); // 12 x5
        assert!(is_repetition_general(33, None)); // 3 x2
        assert!(!is_repetition_general(12345, None)); // no repetition
    }
}

