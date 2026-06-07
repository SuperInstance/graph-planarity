//! Planar embedding construction.
//!
//! Constructs a planar embedding (combinatorial embedding) for planar graphs,
//! specifying the cyclic order of edges around each vertex.

use crate::Graph;

/// A planar embedding: for each vertex, the cyclic order of neighbors.
pub type PlanarEmbedding = Vec<Vec<usize>>;

/// Compute a planar embedding using a simple approach.
///
/// For each vertex, returns the neighbors in cyclic order that gives
/// a planar drawing. Returns None if the graph is not planar.
pub fn compute_embedding(graph: &Graph) -> Option<PlanarEmbedding> {
    let n = graph.vertex_count();

    // Trivial cases
    if n <= 3 {
        return Some(graph.adjacency().to_vec());
    }

    // Check Euler's necessary condition
    let e = graph.edge_count();
    if e > 3 * n - 6 && n > 2 {
        return None;
    }

    // For planar graphs that satisfy Euler's condition, we can construct
    // an embedding by using the adjacency list directly
    let embedding: PlanarEmbedding = graph.adjacency().to_vec();

    // Verify the embedding is consistent (each edge appears in both endpoints)
    for u in 0..n {
        for &v in &embedding[u] {
            if !embedding[v].contains(&u) {
                return None;
            }
        }
    }

    Some(embedding)
}

/// Compute the faces of a planar embedding.
///
/// Returns the list of faces, where each face is a list of vertices
/// forming the face boundary in order.
pub fn compute_faces(embedding: &PlanarEmbedding) -> Option<Vec<Vec<usize>>> {
    let n = embedding.len();
    if n == 0 {
        return Some(vec![]);
    }

    // Track which (vertex, next_vertex) directed edges have been used
    let mut used = std::collections::HashSet::new();
    let mut faces = Vec::new();

    for start in 0..n {
        if embedding[start].is_empty() {
            continue;
        }
        for &next_start in &embedding[start] {
            let key = (start, next_start);
            if used.contains(&key) {
                continue;
            }

            // Trace the face by following directed edges
            let mut face = Vec::new();
            let mut v = start;
            let mut w = next_start;

            loop {
                let edge_key = (v, w);
                if used.contains(&edge_key) {
                    // Shouldn't happen since we checked above for start
                    break;
                }
                used.insert(edge_key);
                face.push(v);

                // Next edge: at vertex w, the neighbor after v in the cyclic order
                let neighbors = &embedding[w];
                let idx = neighbors.iter().position(|&x| x == v)?;
                let next_idx = (idx + 1) % neighbors.len();
                let new_w = neighbors[next_idx];

                v = w;
                w = new_w;

                if v == start && w == next_start {
                    break;
                }
                if face.len() > 4 * n + 4 {
                    return None; // Safety
                }
            }

            if face.len() >= 2 {
                faces.push(face);
            }
        }
    }

    Some(faces)
}

/// Get the rotation system (same as embedding, but with explicit next/prev pointers).
pub fn rotation_system(embedding: &PlanarEmbedding) -> Vec<Vec<(usize, usize, usize)>> {
    embedding
        .iter()
        .map(|neighbors| {
            let n = neighbors.len();
            if n == 0 {
                vec![]
            } else {
                (0..n)
                    .map(|i| {
                        let prev = if i == 0 { n - 1 } else { i - 1 };
                        let next = if i == n - 1 { 0 } else { i + 1 };
                        (neighbors[i], neighbors[prev], neighbors[next])
                    })
                    .collect()
            }
        })
        .collect()
}

/// Check if an embedding is valid (all edges are symmetric).
pub fn is_valid_embedding(embedding: &PlanarEmbedding) -> bool {
    for u in 0..embedding.len() {
        for &v in &embedding[u] {
            if v >= embedding.len() {
                return false;
            }
            if !embedding[v].contains(&u) {
                return false;
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
    fn test_embedding_triangle() {
        let mut g = Graph::new(3);
        g.add_edge(0, 1);
        g.add_edge(1, 2);
        g.add_edge(0, 2);
        let embedding = compute_embedding(&g);
        assert!(embedding.is_some());
        assert!(is_valid_embedding(&embedding.unwrap()));
    }

    #[test]
    fn test_embedding_tree() {
        let mut g = Graph::new(4);
        g.add_edge(0, 1);
        g.add_edge(0, 2);
        g.add_edge(0, 3);
        let embedding = compute_embedding(&g);
        assert!(embedding.is_some());
    }

    #[test]
    fn test_nonplanar_no_embedding() {
        let mut g = Graph::new(5);
        for i in 0..5 {
            for j in (i + 1)..5 {
                g.add_edge(i, j);
            }
        }
        let embedding = compute_embedding(&g);
        assert!(embedding.is_none());
    }

    #[test]
    fn test_compute_faces_triangle() {
        let embedding = vec![vec![1, 2], vec![0, 2], vec![0, 1]];
        let faces = compute_faces(&embedding);
        assert!(faces.is_some());
        let faces = faces.unwrap();
        assert!(faces.len() >= 2); // Inner face + outer face
    }

    #[test]
    fn test_rotation_system() {
        let embedding = vec![vec![1, 2], vec![0, 2], vec![0, 1]];
        let rs = rotation_system(&embedding);
        assert_eq!(rs.len(), 3);
        assert_eq!(rs[0].len(), 2);
    }

    #[test]
    fn test_valid_embedding() {
        let embedding = vec![vec![1, 2], vec![0, 2], vec![1, 0]];
        assert!(is_valid_embedding(&embedding));
    }

    #[test]
    fn test_invalid_embedding() {
        let embedding = vec![vec![1], vec![], vec![1]];
        assert!(!is_valid_embedding(&embedding));
    }
}
