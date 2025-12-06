advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u64> {
    let (ranges, products) = parse_input(input);
    // Compute merged ranges once and reuse for membership checks.
    let merged = merge_ranges(ranges);
    let fresh = products
        .into_iter()
        .filter(|&id| contains_in_merged(&merged, id))
        .count() as u64;
    Some(fresh)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (ranges, _products) = parse_input(input);
    let merged = merge_ranges(ranges);
    let total: i64 = merged.iter().map(|r| r.end - r.start + 1).sum();
    Some(total as u64)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Range {
    start: i64,
    end: i64,
}

// Parse input with two sections separated by a blank line:
// - Section 1: list of ranges like "3-5, 18-22" possibly spread across lines
//   and separated by commas and/or whitespace.
// - Section 2: list of product IDs, one per line.
// Returns normalized ranges (start <= end) and product IDs.
fn parse_input(input: &str) -> (Vec<Range>, Vec<i64>) {
    let (ranges_str, ids_str) = match input.split_once("\n\n") {
        Some(pair) => pair,
        None => (input, ""),
    };

    let mut ranges: Vec<Range> = Vec::new();
    for line in ranges_str.lines() {
        let t = line.trim();
        if t.is_empty() { continue; }
        if let Some((a_raw, b_raw)) = t.split_once('-') {
            let a_str = a_raw.trim();
            let b_str = b_raw.trim();
            if let (Ok(a), Ok(b)) = (a_str.parse::<i64>(), b_str.parse::<i64>()) {
                let (start, end) = if a <= b { (a, b) } else { (b, a) };
                ranges.push(Range { start, end });
            }
        }
    }

    let product_ids: Vec<i64> = ids_str
        .lines()
        .filter_map(|line| {
            let t = line.trim();
            if t.is_empty() { None } else { t.parse::<i64>().ok() }
        })
        .collect();

    (ranges, product_ids)
}

// (Optional) If needed later, we can add a separate search index for performance.

// Binary search membership on merged, sorted ranges
fn contains_in_merged(merged: &[Range], id: i64) -> bool {
    if merged.is_empty() { return false; }
    let mut lo = 0usize;
    let mut hi = merged.len();
    while lo < hi {
        let mid = (lo + hi) / 2;
        if merged[mid].start <= id {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    if lo == 0 { return false; }
    let r = merged[lo - 1];
    id <= r.end
}

// (Optional) Binary search on a prebuilt index can be added if needed.

// Merge overlapping or adjacent ranges to compute a compact union.
fn merge_ranges(mut ranges: Vec<Range>) -> Vec<Range> {
    if ranges.is_empty() { return ranges; }
    ranges.sort_by_key(|r| (r.start, r.end));
    let mut out: Vec<Range> = Vec::new();
    let mut cur = ranges[0];
    for r in ranges.into_iter().skip(1) {
        // If r overlaps or touches cur, extend cur.
        if r.start <= cur.end + 1 {
            if r.end > cur.end { cur.end = r.end; }
        } else {
            out.push(cur);
            cur = r;
        }
    }
    out.push(cur);
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        // synthetic input: ranges then blank line then product IDs
        let s = "3-5\n18-22\n7-7\n\n2\n3\n5\n6\n18\n21\n7\n22\n23\n";
        // fresh IDs: 3,5 (in 3-5); 18,21,22 (in 18-22); 7 (in 7-7) => 6 total
        let result = part_one(s);
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        // Overlapping and adjacent ranges union length
        let s = "1-3\n2-5\n10-12\n13-13\n\n"; // products section empty
        // Merged: [1,5] length 5; [10,13] length 4 => total 9
        let result = part_two(s);
        assert_eq!(result, Some(9));
    }

    #[test]
    fn test_merge_ranges_disjoint() {
        let s = "100-100\n200-201\n\n";
        let result = part_two(s);
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_parse_input_basic() {
        let s = "3-5\n18-22\n7-7\n\n42\n100\n";
        let (ranges, ids) = parse_input(s);
        assert_eq!(ranges.len(), 3);
        assert_eq!(ranges[0], Range { start: 3, end: 5 });
        assert_eq!(ranges[1], Range { start: 18, end: 22 });
        assert_eq!(ranges[2], Range { start: 7, end: 7 });
        assert_eq!(ids, vec![42, 100]);
    }
}
