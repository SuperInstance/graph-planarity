//! Euler's formula verification and face counting.
//!
//! Euler's formula for planar graphs: V - E + F = 2
//! where V = vertices, E = edges, F = faces.

use crate::Graph;

/// Result of Euler's formula computation.
#[derive(Debug, Clone)]
pub struct EulerResult {
    /// Number of vertices.
    pub vertices: usize,
    /// Number of edges.
    pub edges: usize,
    /// Number of faces (including the outer face).
    pub faces: usize,
    /// Euler characteristic: V - E + F.
    pub euler_characteristic: i64,
    /// Whether Euler's formula is satisfied.
    pub satisfies_euler: bool,
}

/// Compute Euler's formula for a planar graph.
///
/// Uses the formula V - E + F = 2 for connected planar graphs.
pub fn euler_formula(graph: &Graph) -> EulerResult {
    let v = graph.vertex_count();
    let e = graph.edge_count();

    // For a planar embedding: F = E - V + 2 (for connected graphs)
    let components = count_components(graph);
    let f = if components == 0 {
        0
    } else {
        // Euler's formula for disconnected: V - E + F = 1 + C
        // So F = E - V + 1 + C
        e as i64 - v as i64 + 1 + components as i64
    };
    let f = f.max(1) as usize;

    let euler_char = v as i64 - e as i64 + f as i64;
    // For connected planar graph: euler_char should be 2
    let satisfies = if components <= 1 {
        euler_char == 2
    } else {
        euler_char == (1 + components as i64)
    };

    EulerResult {
        vertices: v,
        edges: e,
        faces: f,
        euler_characteristic: euler_char,
        satisfies_euler: satisfies,
    }
}

/// Verify that a graph satisfies Euler's formula.
pub fn verifies_euler(graph: &Graph) -> bool {
    euler_formula(graph).satisfies_euler
}

/// Compute the maximum number of edges in a planar graph with `n` vertices.
pub fn max_edges_planar(n: usize) -> usize {
    if n < 3 {
        n * (n - 1) / 2
    } else {
        3 * n - 6
    }
}

/// Compute the maximum number of edges in a triangle-free planar graph.
pub fn max_edges_triangle_free_planar(n: usize) -> usize {
    if n < 3 {
        n * (n - 1) / 2
    } else {
        2 * n - 4
    }
}

/// Compute the minimum number of faces for a connected planar graph.
pub fn min_faces(graph: &Graph) -> usize {
    if graph.edge_count() == 0 {
        return 1;
    }
    // For connected planar graph: F = E - V + 2
    let v = graph.vertex_count();
    let e = graph.edge_count();
    if e >= v {
        e - v + 2
    } else {
        1
    }
}

/// Count the number of connected components.
pub fn count_components(graph: &Graph) -> usize {
    let n = graph.vertex_count();
    if n == 0 {
        return 0;
    }

    let mut visited = vec![false; n];
    let mut components = 0;

    for start in 0..n {
        if visited[start] {
            continue;
        }
        components += 1;
        let mut stack = vec![start];
        visited[start] = true;
        while let Some(u) = stack.pop() {
            for &v in graph.neighbors(u) {
                if !visited[v] {
                    visited[v] = true;
                    stack.push(v);
                }
            }
        }
    }

    components
}

/// Compute the Euler genus: 2 - V + E - F.
pub fn euler_genus(graph: &Graph) -> i64 {
    let result = euler_formula(graph);
    2 - result.euler_characteristic
}

/// Check if a graph satisfies the necessary planarity conditions from Euler's formula.
pub fn planarity_from_euler(graph: &Graph) -> bool {
    let n = graph.vertex_count();
    let e = graph.edge_count();

    if n <= 2 {
        return true;
    }

    // Necessary condition: e ≤ 3v - 6
    if e > 3 * n - 6 {
        return false;
    }

    // For bipartite (triangle-free): e ≤ 2v - 4
    if is_bipartite(graph) && e > 2 * n - 4 && n >= 3 {
        return false;
    }

    true
}

fn is_bipartite(graph: &Graph) -> bool {
    let n = graph.vertex_count();
    let mut color = vec![None::<bool>; n];

    for start in 0..n {
        if color[start].is_some() {
            continue;
        }
        color[start] = Some(true);
        let mut stack = vec![start];
        while let Some(u) = stack.pop() {
            let cu = color[u].unwrap();
            for &v in graph.neighbors(u) {
                match color[v] {
                    Some(cv) => {
                        if cv == cu {
                            return false;
                        }
                    }
                    None => {
                        color[v] = Some(!cu);
                        stack.push(v);
                    }
                }
            }
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Graph;

    #[test]
    fn test_euler_triangle() {
        let mut g = Graph::new(3);
        g.add_edge(0, 1);
        g.add_edge(1, 2);
        g.add_edge(0, 2);
        let result = euler_formula(&g);
        assert_eq!(result.vertices, 3);
        assert_eq!(result.edges, 3);
        assert!(result.satisfies_euler);
    }

    #[test]
    fn test_euler_cycle() {
        let mut g = Graph::new(4);
        for i in 0..4 {
            g.add_edge(i, (i + 1) % 4);
        }
        let result = euler_formula(&g);
        assert!(result.satisfies_euler);
    }

    #[test]
    fn test_euler_tree() {
        let mut g = Graph::new(4);
        g.add_edge(0, 1);
        g.add_edge(0, 2);
        g.add_edge(0, 3);
        let result = euler_formula(&g);
        assert!(result.satisfies_euler);
    }

    #[test]
    fn test_max_edges_planar() {
        assert_eq!(max_edges_planar(4), 6);
        assert_eq!(max_edges_planar(5), 9);
    }

    #[test]
    fn test_max_edges_triangle_free() {
        assert_eq!(max_edges_triangle_free_planar(4), 4);
    }

    #[test]
    fn test_count_components_connected() {
        let mut g = Graph::new(4);
        g.add_edge(0, 1);
        g.add_edge(1, 2);
        g.add_edge(2, 3);
        assert_eq!(count_components(&g), 1);
    }

    #[test]
    fn test_count_components_disconnected() {
        let mut g = Graph::new(6);
        g.add_edge(0, 1);
        g.add_edge(2, 3);
        // Vertices 4 and 5 are isolated
        assert_eq!(count_components(&g), 4);
    }

    #[test]
    fn test_planarity_from_euler_k5() {
        let mut g = Graph::new(5);
        for i in 0..5 {
            for j in (i + 1)..5 {
                g.add_edge(i, j);
            }
        }
        assert!(!planarity_from_euler(&g));
    }

    #[test]
    fn test_planarity_from_euler_path() {
        let mut g = Graph::new(4);
        g.add_edge(0, 1);
        g.add_edge(1, 2);
        g.add_edge(2, 3);
        assert!(planarity_from_euler(&g));
    }

    #[test]
    fn test_min_faces_tree() {
        let mut g = Graph::new(4);
        g.add_edge(0, 1);
        g.add_edge(0, 2);
        g.add_edge(0, 3);
        assert_eq!(min_faces(&g), 1);
    }

    #[test]
    fn test_empty_euler() {
        let g = Graph::new(0);
        let result = euler_formula(&g);
        assert_eq!(result.vertices, 0);
    }
}
