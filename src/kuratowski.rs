//! Kuratowski subgraph detection.
//!
//! Identifies K₅ and K₃,₃ subdivisions/minors in non-planar graphs
//! by Kuratowski's theorem.

use crate::Graph;

/// Result of Kuratowski subgraph detection.
#[derive(Debug, Clone)]
pub struct KuratowskiResult {
    /// Whether the graph contains a Kuratowski subgraph.
    pub found: bool,
    /// Type of obstruction found.
    pub obstruction: KuratowskiObstruction,
}

/// The type of Kuratowski obstruction found.
#[derive(Debug, Clone, PartialEq)]
pub enum KuratowskiObstruction {
    /// No obstruction found.
    None,
    /// K₅ (complete graph on 5 vertices) subdivision.
    K5(Vec<usize>),
    /// K₃,₃ (complete bipartite graph) subdivision.
    K33(Vec<usize>, Vec<usize>),
}

/// Detect Kuratowski subgraph obstructions.
///
/// By Kuratowski's theorem, a graph is non-planar if and only if
/// it contains a subdivision of K₅ or K₃,₃.
pub fn detect_kuratowski(graph: &Graph) -> KuratowskiResult {
    let n = graph.vertex_count();

    // Check for K₅
    if n >= 5 {
        if let Some(vertices) = find_k5(graph) {
            return KuratowskiResult {
                found: true,
                obstruction: KuratowskiObstruction::K5(vertices),
            };
        }
    }

    // Check for K₃,₃
    if n >= 6 {
        if let Some((left, right)) = find_k33(graph) {
            return KuratowskiResult {
                found: true,
                obstruction: KuratowskiObstruction::K33(left, right),
            };
        }
    }

    KuratowskiResult {
        found: false,
        obstruction: KuratowskiObstruction::None,
    }
}

fn find_k5(graph: &Graph) -> Option<Vec<usize>> {
    let n = graph.vertex_count();
    if !(5..=15).contains(&n) {
        return None;
    }

    for mask in 0u32..(1u32 << n) {
        if mask.count_ones() == 5 {
            let vertices: Vec<usize> = (0..n).filter(|&i| (mask >> i) & 1 == 1).collect();
            if is_complete(graph, &vertices) {
                return Some(vertices);
            }
        }
    }
    None
}

fn find_k33(graph: &Graph) -> Option<(Vec<usize>, Vec<usize>)> {
    let n = graph.vertex_count();
    if !(6..=15).contains(&n) {
        return None;
    }

    for mask in 0u32..(1u32 << n) {
        if mask.count_ones() != 6 {
            continue;
        }
        let vertices: Vec<usize> = (0..n).filter(|&i| (mask >> i) & 1 == 1).collect();

        for partition in 0u32..(1u32 << 6) {
            if partition.count_ones() != 3 {
                continue;
            }
            let left: Vec<usize> = (0..6).filter(|&i| (partition >> i) & 1 == 1).map(|i| vertices[i]).collect();
            let right: Vec<usize> = (0..6).filter(|&i| (partition >> i) & 1 == 0).map(|i| vertices[i]).collect();

            let mut is_k33 = true;
            'outer: for &l in &left {
                for &r in &right {
                    if !graph.has_edge(l, r) {
                        is_k33 = false;
                        break 'outer;
                    }
                }
            }
            // Also check no edges within left or right
            if is_k33 {
                for i in 0..3 {
                    for j in (i + 1)..3 {
                        if graph.has_edge(left[i], left[j]) || graph.has_edge(right[i], right[j]) {
                            is_k33 = false;
                            break;
                        }
                    }
                    if !is_k33 {
                        break;
                    }
                }
            }

            if is_k33 {
                return Some((left, right));
            }
        }
    }
    None
}

fn is_complete(graph: &Graph, vertices: &[usize]) -> bool {
    for i in 0..vertices.len() {
        for j in (i + 1)..vertices.len() {
            if !graph.has_edge(vertices[i], vertices[j]) {
                return false;
            }
        }
    }
    true
}

/// Check if the graph contains K₅ as a subgraph (not subdivision).
pub fn contains_k5(graph: &Graph) -> bool {
    find_k5(graph).is_some()
}

/// Check if the graph contains K₃,₃ as a subgraph.
pub fn contains_k33(graph: &Graph) -> bool {
    find_k33(graph).is_some()
}

/// Get the minimum Kuratowski obstruction (fewest vertices).
pub fn minimal_obstruction(graph: &Graph) -> KuratowskiObstruction {
    let result = detect_kuratowski(graph);
    result.obstruction
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Graph;

    #[test]
    fn test_k5_detected() {
        let mut g = Graph::new(5);
        for i in 0..5 {
            for j in (i + 1)..5 {
                g.add_edge(i, j);
            }
        }
        let result = detect_kuratowski(&g);
        assert!(result.found);
        assert!(matches!(result.obstruction, KuratowskiObstruction::K5(_)));
    }

    #[test]
    fn test_k33_detected() {
        let mut g = Graph::new(6);
        for i in 0..3 {
            for j in 3..6 {
                g.add_edge(i, j);
            }
        }
        let result = detect_kuratowski(&g);
        assert!(result.found);
        assert!(matches!(result.obstruction, KuratowskiObstruction::K33(_, _)));
    }

    #[test]
    fn test_planar_no_obstruction() {
        let mut g = Graph::new(4);
        for i in 0..4 {
            for j in (i + 1)..4 {
                g.add_edge(i, j);
            }
        }
        let result = detect_kuratowski(&g);
        assert!(!result.found);
    }

    #[test]
    fn test_contains_k5() {
        let mut g = Graph::new(5);
        for i in 0..5 {
            for j in (i + 1)..5 {
                g.add_edge(i, j);
            }
        }
        assert!(contains_k5(&g));
    }

    #[test]
    fn test_no_k5() {
        let mut g = Graph::new(5);
        for i in 0..4 {
            g.add_edge(i, i + 1);
        }
        assert!(!contains_k5(&g));
    }

    #[test]
    fn test_k5_not_enough_vertices() {
        let g = Graph::new(4);
        assert!(!contains_k5(&g));
    }

    #[test]
    fn test_minimal_obstruction_k5() {
        let mut g = Graph::new(5);
        for i in 0..5 {
            for j in (i + 1)..5 {
                g.add_edge(i, j);
            }
        }
        let obs = minimal_obstruction(&g);
        assert!(matches!(obs, KuratowskiObstruction::K5(_)));
    }
}
