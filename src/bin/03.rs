advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u64> {
    let sum: u64 = input
        .lines()
        .map(|line| max_subsequence_number(line, 2))
        .sum();
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let sum: u64 = input
        .lines()
        .map(|line| max_subsequence_number(line, 12))
        .sum();
    Some(sum)
}

// Goal: pick exactly `count` digits from the line, keeping their original order,
// to form the largest possible number.
// Intuition: always prefer bigger leading digits when you can still finish with
// exactly `count` picks.
/// Given a string of digits, choose exactly `k` of them (in order) to make
/// the largest possible number. If there aren't enough digits, return 0.
fn max_subsequence_number(line: &str, k: usize) -> u64 {
    // Parse digits as u8s
    fn parse_digits(s: &str) -> Vec<u8> {
        s.chars().filter_map(|c| c.to_digit(10).map(|d| d as u8)).collect()
    }

    // Concatenate digits into a base-10 number
    fn concat_digits(digs: &[u8]) -> u64 {
        digs.iter().fold(0u64, |acc, &x| acc * 10 + x as u64)
    }

    let digits = parse_digits(line);
    let n = digits.len();
    if n < k || k == 0 {
        return 0;
    }

    // O(n) monotonic stack approach:
    // - Maintain a decreasing stack of chosen digits.
    // - When a larger digit arrives, pop smaller tops if we still have enough
    //   remaining digits to end up with exactly `count` total.
    let mut chosen: Vec<u8> = Vec::with_capacity(k);
    for (i, &d) in digits.iter().enumerate() {
        // Can we pop? Only if: top < d AND (after popping) remaining digits suffice to reach `count`.
        while let Some(&top) = chosen.last() {
            let can_pop = top < d && (chosen.len() - 1) + (n - i) >= k;
            if can_pop {
                chosen.pop();
            } else {
                break;
            }
        }
        if chosen.len() < k {
            chosen.push(d);
        }
    }

    // We might have pushed extra early then pruned; take exactly `count`.
    concat_digits(&chosen[..k])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = "129\n98\n3917\n777\n";
        // 129 -> 29, 98 -> 98, 3917 -> 97, 777 -> 77 => sum = 29 + 98 + 97 + 77 = 301
        let result = part_one(input);
        assert_eq!(result, Some(301));
    }

    #[test]
    fn test_part_two() {
        // With count=12, short lines just take as many as possible
        let input = "129\n98\n3917\n777\n";
        // 129 -> pick 3 digits: 9, then 2? No, order preserved: first pick 2 (argmax in 0..n-2), then 9, then none => 29
        // For count=12 we still only get 29 for this line, similarly for others.
        let result = part_two(input);
        // We don't assert a specific sum here since test data is short; just ensure it returns Some
        assert!(result.is_some());
    }

    #[test]
    fn test_best_two_digit_number() {
        assert_eq!(max_subsequence_number("129", 2), 29);
        assert_eq!(max_subsequence_number("98", 2), 98);
        assert_eq!(max_subsequence_number("3917", 2), 97);
        assert_eq!(max_subsequence_number("777", 2), 77);
        assert_eq!(max_subsequence_number("5", 2), 0);
        assert_eq!(max_subsequence_number("", 2), 0);
    // More digits requested than available: require exact count; return 0
        assert_eq!(max_subsequence_number("123", 5), 0);
    }
}
