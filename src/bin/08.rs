advent_of_code::solution!(8);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Pt3 { x: i64, y: i64, z: i64 }

// ---------- Parsing and basic math ----------
/// Parse input lines of `x,y,z` into a list of points.
fn parse(input: &str) -> Vec<Pt3> {
    input
        .lines()
        .map(|line| {
            // Support both comma-separated and space-separated for tests, but assume well-formatted
            let parts: Vec<&str> = line.split(',').collect();
            let x = parts[0].trim().parse::<i64>().unwrap();
            let y = parts[1].trim().parse::<i64>().unwrap();
            let z = parts[2].trim().parse::<i64>().unwrap();
            Pt3 { x, y, z }
        })
        .collect()
}

/// Squared Euclidean distance between two points.
fn dist2(a: Pt3, b: Pt3) -> i128 {
    let dx = (a.x - b.x) as i128;
    let dy = (a.y - b.y) as i128;
    let dz = (a.z - b.z) as i128;
    dx * dx + dy * dy + dz * dz
}

// ---------- Edge generation and selection ----------

/// Generate all unique edges with their squared distance.
fn generate_all_edges(points: &[Pt3]) -> Vec<(usize, usize, i128)> {
    let n = points.len();
    let mut edges = Vec::with_capacity(n.saturating_mul(n.saturating_sub(1)) / 2);
    for i in 0..n { for j in (i + 1)..n { edges.push((i, j, dist2(points[i], points[j]))); } }
    edges
}

/// Select the k shortest edges among all point pairs.
fn select_k_shortest_edges(points: &[Pt3], k: usize) -> Vec<(usize, usize, i128)> {
    let mut edges = generate_all_edges(points);
    let kk = k.min(edges.len());
    if kk == 0 { return Vec::new(); }
    let (left, _pivot, _right) = edges.select_nth_unstable_by(kk - 1, |a, b| a.2.cmp(&b.2));
    left.sort_by(|a, b| a.2.cmp(&b.2));
    edges.truncate(kk);
    edges
}

/// Build connected components (circuits) from a list of edges.
fn circuits_from_pairs(n_points: usize, pairs: &[(usize, usize, i128)]) -> Vec<Vec<usize>> {
    // Build adjacency and find connected components among nodes that appear in pairs
    let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n_points];
    let mut used: std::collections::HashSet<usize> = std::collections::HashSet::new();
    for &(i, j, _) in pairs {
        adj[i].push(j);
        adj[j].push(i);
        used.insert(i);
        used.insert(j);
    }
    let mut visited = vec![false; n_points];
    let mut comps: Vec<Vec<usize>> = Vec::new();
    for &start in &used {
        if visited[start] { continue; }
        let mut stack = vec![start];
        let mut comp: Vec<usize> = Vec::new();
        visited[start] = true;
        while let Some(u) = stack.pop() {
            comp.push(u);
            for &v in &adj[u] {
                if !visited[v] {
                    visited[v] = true;
                    stack.push(v);
                }
            }
        }
        comps.push(comp);
    }
    comps
}

#[derive(Debug)]
/// Tracks which points are connected together; merges groups as edges are processed.
struct ConnectivityTracker {
    // For each node, store the parent representative of its connected group
    parent: Vec<usize>,
    // Size of each group (used to keep trees shallow when merging)
    size: Vec<usize>,
    // Number of separate groups currently present
    groups: usize,
}

impl ConnectivityTracker {
    fn new(n: usize) -> Self {
        Self { parent: (0..n).collect(), size: vec![1; n], groups: n }
    }
    // Find the representative id (root) for a node, with path compression
    fn find_root(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find_root(self.parent[x]);
        }
        self.parent[x]
    }
    // Connect two nodes' groups; returns true if a merge happened (they were separate)
    fn connect(&mut self, a: usize, b: usize) -> bool {
        let mut ra = self.find_root(a);
        let mut rb = self.find_root(b);
        if ra == rb { return false; }
        // Attach smaller group under larger group to keep tree shallow
        if self.size[ra] < self.size[rb] { std::mem::swap(&mut ra, &mut rb); }
        self.parent[rb] = ra;
        self.size[ra] += self.size[rb];
        self.groups -= 1;
        true
    }
}

/// Process edges from shortest to largest (in batches) until all points are connected.
/// Returns the final edge (indices) that completes full connectivity.
fn fully_connect_last_edge(points: &[Pt3]) -> Option<(usize, usize)> {
    let n = points.len();
    if n == 0 { return None; }
    // Generate all pairs (unique) with distance
    let mut pairs: Vec<(usize, usize, i128)> = generate_all_edges(points);

    // Connect shortest edges first until all points belong to one group.
    let mut tracker = ConnectivityTracker::new(n);
    // To keep it fast, process edges in small batches of the shortest distances
    // without sorting the entire list upfront.
    const MAX_EDGES_PER_BATCH: usize = 8_000; // tune if needed
    let mut start = 0usize;
    while start < pairs.len() {
        let remaining = pairs.len() - start;
        let take = MAX_EDGES_PER_BATCH.min(remaining).max(1);
        let end_partition = start + take - 1;
        // Partition so that [start..=end_partition] are the smallest among remaining
        let _ = pairs.select_nth_unstable_by(end_partition, |a, b| a.2.cmp(&b.2));
        // Sort just this batch for stable processing
        pairs[start..=end_partition].sort_by(|a, b| a.2.cmp(&b.2));

        for &(i, j, _) in &pairs[start..=end_partition] {
            if tracker.connect(i, j) {
                // When only one group remains, this edge completed connectivity
                if tracker.groups == 1 { return Some((i, j)); }
            }
        }
        start = end_partition + 1;
    }
    None
}

pub fn part_one(input: &str) -> Option<u64> {
    let pts = parse(input);
    if pts.len() < 2 { return None; }
    let selected = select_k_shortest_edges(&pts, 1000);
    let mut circuits = circuits_from_pairs(pts.len(), &selected);
    circuits.sort_by_key(|c| std::cmp::Reverse(c.len()));
    let product = circuits
        .iter()
        .take(3)
        .fold(1u64, |acc, c| acc.saturating_mul(c.len() as u64));
    Some(product)
}

pub fn part_two(input: &str) -> Option<i64> {
    let pts = parse(input);
    if pts.len() < 2 { return None; }
    let (i, j) = fully_connect_last_edge(&pts)?;
    let a = pts[i];
    let b = pts[j];
    let prod = (a.x as i128) * (b.x as i128);
    Some(prod as i64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        // Minimal example
        let input = "0,0,0\n1,0,0\n2,0,0";
        let result = part_one(input);
        // Only one circuit of size 3 -> product of top 3 sizes = 3
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let input = "0,0,0\n2,0,0\n4,0,0";
        let result = part_two(input);
        // Last to fully connect is (1,2): x coords 2 and 4 -> product 8
        assert_eq!(result, Some(8));
    }
}
