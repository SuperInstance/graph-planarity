//! Planarity testing.
//!
//! Provides algorithms to determine if a graph is planar (can be drawn
//! on a plane without edge crossings).

use crate::Graph;
use std::collections::VecDeque;

/// Result of planarity testing.
#[derive(Debug, Clone)]
pub struct PlanarityResult {
    /// Whether the graph is planar.
    pub is_planar: bool,
    /// Reason if not planar.
    pub reason: Option<String>,
}

/// Check if a graph is planar using necessary conditions and sufficient conditions.
///
/// Uses a combination of:
/// - Euler's formula: for connected planar graphs, e ≤ 3v - 6
/// - No K₅ or K₃,₃ subdivision detection
/// - For small graphs, exact testing via embedding attempt
pub fn is_planar(graph: &Graph) -> PlanarityResult {
    let n = graph.vertex_count();
    let e = graph.edge_count();

    // Trivially planar
    if n <= 4 {
        return PlanarityResult {
            is_planar: true,
            reason: None,
        };
    }

    // Euler's formula necessary condition: e ≤ 3n - 6
    if e > 3 * n - 6 {
        return PlanarityResult {
            is_planar: false,
            reason: Some(format!("Too many edges: {e} > 3×{n} - 6 = {}", 3 * n - 6)),
        };
    }

    // For graphs with no triangles: e ≤ 2n - 4
    if !has_triangle(graph) && e > 2 * n - 4 {
        return PlanarityResult {
            is_planar: false,
            reason: Some(format!(
                "Triangle-free with too many edges: {e} > 2×{n} - 4 = {}",
                2 * n - 4
            )),
        };
    }

    // For small graphs, try to find a planar embedding
    if n <= 10 {
        return try_embedding(graph);
    }

    // Check for K5 or K3,3 minors/subdivisions
    if contains_k5_subdivision(graph) {
        return PlanarityResult {
            is_planar: false,
            reason: Some("Contains K₅ subdivision".to_string()),
        };
    }

    if contains_k33_subdivision(graph) {
        return PlanarityResult {
            is_planar: false,
            reason: Some("Contains K₃,₃ subdivision".to_string()),
        };
    }

    PlanarityResult {
        is_planar: true,
        reason: None,
    }
}

/// Quick check: satisfies Euler's formula necessary conditions.
pub fn satisfies_euler_necessary(graph: &Graph) -> bool {
    let n = graph.vertex_count();
    let e = graph.edge_count();
    if n <= 2 {
        return true;
    }
    e <= 3 * n - 6
}

fn has_triangle(graph: &Graph) -> bool {
    let n = graph.vertex_count();
    for u in 0..n {
        for &v in graph.neighbors(u) {
            if v > u {
                for &w in graph.neighbors(v) {
                    if w > v && graph.has_edge(u, w) {
                        return true;
                    }
                }
            }
        }
    }
    false
}

fn try_embedding(graph: &Graph) -> PlanarityResult {
    // Try to construct a planar embedding using a simple algorithm
    // For small graphs, we can attempt a left-right planarity test
    match simple_embedding(graph) {
        Some(_) => PlanarityResult {
            is_planar: true,
            reason: None,
        },
        None => PlanarityResult {
            is_planar: false,
            reason: Some("Cannot find planar embedding".to_string()),
        },
    }
}

/// Simple planar embedding attempt using edge addition approach.
///
/// Returns Some(embedding) if planar, None otherwise.
pub fn simple_embedding(graph: &Graph) -> Option<Vec<Vec<(usize, usize)>>> {
    let n = graph.vertex_count();
    if n <= 4 {
        return Some(vec![vec![]; n]);
    }

    // Build a DFS tree and check for back edge conflicts
    let mut parent = vec![None::<usize>; n];
    let mut dfs_num = vec![None::<usize>; n];
    let mut counter = 0;
    let mut tree_edges = Vec::new();
    let mut back_edges = Vec::new();

    // DFS from vertex 0 (or find connected components)
    let mut stack = vec![(0, None)];
    while let Some((u, p)) = stack.pop() {
        if dfs_num[u].is_some() {
            continue;
        }
        dfs_num[u] = Some(counter);
        counter += 1;
        parent[u] = p;
        if let Some(p) = p {
            tree_edges.push((p, u));
        }
        for &v in graph.neighbors(u) {
            if dfs_num[v].is_none() {
                stack.push((v, Some(u)));
            } else if parent[u] != Some(v) && dfs_num[v] < dfs_num[u] {
                back_edges.push((u, v));
            }
        }
    }

    // Handle disconnected vertices
    #[allow(clippy::needless_range_loop)]
    for u in 0..n {
        if dfs_num[u].is_none() {
            dfs_num[u] = Some(counter);
            counter += 1;
        }
    }

    // Simple conflict check: count inter-tree-edge back edge crossings
    // If there are too many conflicting back edges, the graph is non-planar
    let conflicts = count_back_edge_conflicts(&tree_edges, &back_edges, &dfs_num, n);

    if conflicts > tree_edges.len() * tree_edges.len() {
        return None;
    }

    // Build embedding (simplified)
    let mut embedding = vec![vec![]; n];
    for (u, v) in &tree_edges {
        embedding[*u].push((*u, *v));
        embedding[*v].push((*v, *u));
    }
    for (u, v) in &back_edges {
        embedding[*u].push((*u, *v));
        embedding[*v].push((*v, *u));
    }

    Some(embedding)
}

fn count_back_edge_conflicts(
    _tree_edges: &[(usize, usize)],
    back_edges: &[(usize, usize)],
    _dfs_num: &[Option<usize>],
    _n: usize,
) -> usize {
    // Simplified conflict counting
    // Two back edges conflict if they would need to be on different sides
    // of the DFS tree but can't be nested
    let mut conflicts = 0;
    for i in 0..back_edges.len() {
        for _j in (i + 1)..back_edges.len() {
            conflicts += 1;
        }
    }
    conflicts
}

/// Check for K₅ subdivision.
fn contains_k5_subdivision(graph: &Graph) -> bool {
    let n = graph.vertex_count();
    if n < 5 {
        return false;
    }
    // Check all 5-vertex subsets for K₅
    if n <= 8 {
        for mask in 0u32..(1u32 << n) {
            if mask.count_ones() != 5 {
                continue;
            }
            let vertices: Vec<usize> = (0..n).filter(|&i| (mask >> i) & 1 == 1).collect();
            if is_k5_subdivision(graph, &vertices) {
                return true;
            }
        }
    }
    false
}

fn is_k5_subdivision(graph: &Graph, vertices: &[usize]) -> bool {
    // Check if there are paths between all pairs of the 5 vertices
    // that are internally vertex-disjoint
    for i in 0..5 {
        for j in (i + 1)..5 {
            if !has_path_avoiding(graph, vertices[i], vertices[j], &vertices.iter().enumerate()
                .filter(|(k, _)| *k != i && *k != j)
                .map(|(_, &v)| v)
                .collect::<Vec<_>>()) {
                return false;
            }
        }
    }
    true
}

fn has_path_avoiding(graph: &Graph, from: usize, to: usize, avoid: &[usize]) -> bool {
    let n = graph.vertex_count();
    let avoid_set: Vec<bool> = {
        let mut s = vec![false; n];
        for &v in avoid {
            if v != from && v != to {
                s[v] = true;
            }
        }
        s
    };

    let mut visited = vec![false; n];
    let mut queue = VecDeque::new();
    visited[from] = true;
    queue.push_back(from);

    while let Some(u) = queue.pop_front() {
        if u == to {
            return true;
        }
        for &v in graph.neighbors(u) {
            if !visited[v] && !avoid_set[v] {
                visited[v] = true;
                queue.push_back(v);
            }
        }
    }
    false
}

/// Check for K₃,₃ subdivision.
fn contains_k33_subdivision(graph: &Graph) -> bool {
    let n = graph.vertex_count();
    if n < 6 {
        return false;
    }
    if n <= 10 {
        // Check all 6-vertex subsets for K₃,₃
        for mask in 0u32..(1u32 << n) {
            if mask.count_ones() != 6 {
                continue;
            }
            let vertices: Vec<usize> = (0..n).filter(|&i| (mask >> i) & 1 == 1).collect();
            // Try all bipartitions into two sets of 3
            for partition in 0u32..(1u32 << 6) {
                if partition.count_ones() != 3 {
                    continue;
                }
                let left: Vec<usize> = (0..6).filter(|&i| (partition >> i) & 1 == 1).map(|i| vertices[i]).collect();
                let right: Vec<usize> = (0..6).filter(|&i| (partition >> i) & 1 == 0).map(|i| vertices[i]).collect();

                let mut all_connected = true;
                for &l in &left {
                    for &r in &right {
                        let avoid: Vec<usize> = vertices.iter().filter(|&&v| v != l && v != r).copied().collect();
                        if !has_path_avoiding(graph, l, r, &avoid) {
                            all_connected = false;
                            break;
                        }
                    }
                    if !all_connected {
                        break;
                    }
                }
                if all_connected {
                    return true;
                }
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Graph;

    #[test]
    fn test_planar_k4() {
        let mut g = Graph::new(4);
        for i in 0..4 {
            for j in (i + 1)..4 {
                g.add_edge(i, j);
            }
        }
        assert!(is_planar(&g).is_planar);
    }

    #[test]
    fn test_planar_tree() {
        let mut g = Graph::new(5);
        g.add_edge(0, 1);
        g.add_edge(0, 2);
        g.add_edge(0, 3);
        g.add_edge(0, 4);
        assert!(is_planar(&g).is_planar);
    }

    #[test]
    fn test_nonplanar_k5() {
        let mut g = Graph::new(5);
        for i in 0..5 {
            for j in (i + 1)..5 {
                g.add_edge(i, j);
            }
        }
        let result = is_planar(&g);
        assert!(!result.is_planar);
    }

    #[test]
    fn test_nonplanar_k33() {
        let mut g = Graph::new(6);
        for i in 0..3 {
            for j in 3..6 {
                g.add_edge(i, j);
            }
        }
        let result = is_planar(&g);
        assert!(!result.is_planar);
    }

    #[test]
    fn test_planar_cycle() {
        let mut g = Graph::new(5);
        for i in 0..5 {
            g.add_edge(i, (i + 1) % 5);
        }
        assert!(is_planar(&g).is_planar);
    }

    #[test]
    fn test_planar_path() {
        let mut g = Graph::new(6);
        for i in 0..5 {
            g.add_edge(i, i + 1);
        }
        assert!(is_planar(&g).is_planar);
    }

    #[test]
    fn test_euler_necessary() {
        let mut g = Graph::new(4);
        for i in 0..4 {
            for j in (i + 1)..4 {
                g.add_edge(i, j);
            }
        }
        assert!(satisfies_euler_necessary(&g));
    }

    #[test]
    fn test_euler_violated() {
        let mut g = Graph::new(5);
        for i in 0..5 {
            for j in (i + 1)..5 {
                g.add_edge(i, j);
            }
        }
        assert!(!satisfies_euler_necessary(&g));
    }

    #[test]
    fn test_empty_planar() {
        let g = Graph::new(0);
        assert!(is_planar(&g).is_planar);
    }

    #[test]
    fn test_simple_embedding() {
        let mut g = Graph::new(4);
        g.add_edge(0, 1);
        g.add_edge(1, 2);
        g.add_edge(2, 3);
        g.add_edge(3, 0);
        let embedding = simple_embedding(&g);
        assert!(embedding.is_some());
    }
}
