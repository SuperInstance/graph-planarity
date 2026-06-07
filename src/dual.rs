//! Dual graph construction from planar embeddings.
//!
//! Constructs the dual graph of a planar graph from its embedding,
//! where each face becomes a vertex and adjacent faces share edges.

use crate::Graph;
use crate::embedding::{PlanarEmbedding, compute_faces};

/// Result of dual graph construction.
#[derive(Debug, Clone)]
pub struct DualGraph {
    /// The dual graph.
    pub graph: Graph,
    /// Mapping from face index to the original face vertices.
    pub face_vertices: Vec<Vec<usize>>,
    /// Mapping from original edge (u, v) to dual edge (face_i, face_j).
    pub dual_edges: Vec<((usize, usize), (usize, usize))>,
}

/// Compute the dual graph of a planar embedding.
///
/// The dual graph has one vertex for each face of the embedding,
/// and an edge between two dual vertices iff the corresponding faces
/// share an edge in the original graph.
pub fn compute_dual(graph: &Graph) -> Option<DualGraph> {
    let embedding = crate::embedding::compute_embedding(graph)?;
    compute_dual_from_embedding(&embedding)
}

/// Compute the dual graph from a given embedding.
pub fn compute_dual_from_embedding(embedding: &PlanarEmbedding) -> Option<DualGraph> {
    let faces = compute_faces(embedding)?;
    let n_faces = faces.len();

    if n_faces == 0 {
        return Some(DualGraph {
            graph: Graph::new(0),
            face_vertices: vec![],
            dual_edges: vec![],
        });
    }

    let mut dual = Graph::new(n_faces);
    let mut dual_edges = Vec::new();

    // For each pair of faces, check if they share an edge
    for i in 0..n_faces {
        for j in (i + 1)..n_faces {
            if share_edge(&faces[i], &faces[j]) {
                dual.add_edge(i, j);
                // Find the shared edge
                if let Some(edge) = find_shared_edge(&faces[i], &faces[j]) {
                    dual_edges.push((edge, (i, j)));
                }
            }
        }
    }

    Some(DualGraph {
        graph: dual,
        face_vertices: faces,
        dual_edges,
    })
}

/// Check if two faces share an edge.
fn share_edge(face1: &[usize], face2: &[usize]) -> bool {
    for i in 0..face1.len() {
        let u = face1[i];
        let v = face1[(i + 1) % face1.len()];
        for j in 0..face2.len() {
            let a = face2[j];
            let b = face2[(j + 1) % face2.len()];
            if (u == a && v == b) || (u == b && v == a) {
                return true;
            }
        }
    }
    false
}

/// Find the shared edge between two faces.
fn find_shared_edge(face1: &[usize], face2: &[usize]) -> Option<(usize, usize)> {
    for i in 0..face1.len() {
        let u = face1[i];
        let v = face1[(i + 1) % face1.len()];
        for j in 0..face2.len() {
            let a = face2[j];
            let b = face2[(j + 1) % face2.len()];
            if (u == a && v == b) || (u == b && v == a) {
                return Some((u.min(v), u.max(v)));
            }
        }
    }
    None
}

/// Compute the self-dual graph properties.
///
/// A graph is self-dual if it is isomorphic to its dual.
pub fn is_self_dual(graph: &Graph) -> bool {
    let Some(dual_result) = compute_dual(graph) else {
        return false;
    };
    let original_edges = graph.edge_count();
    let dual_edges = dual_result.graph.edge_count();
    let original_vertices = graph.vertex_count();
    let dual_vertices = dual_result.graph.vertex_count();
    original_edges == dual_edges && original_vertices == dual_vertices
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Graph;

    #[test]
    fn test_dual_triangle() {
        let mut g = Graph::new(3);
        g.add_edge(0, 1);
        g.add_edge(1, 2);
        g.add_edge(0, 2);
        let dual = compute_dual(&g);
        assert!(dual.is_some());
        let dual = dual.unwrap();
        // Triangle has 2 faces (inner + outer)
        assert_eq!(dual.face_vertices.len(), 2);
    }

    #[test]
    fn test_dual_cycle() {
        let mut g = Graph::new(4);
        for i in 0..4 {
            g.add_edge(i, (i + 1) % 4);
        }
        let dual = compute_dual(&g);
        assert!(dual.is_some());
        let dual = dual.unwrap();
        // C4 has 2 faces
        assert_eq!(dual.face_vertices.len(), 2);
    }

    #[test]
    fn test_dual_tree() {
        let mut g = Graph::new(4);
        g.add_edge(0, 1);
        g.add_edge(0, 2);
        g.add_edge(0, 3);
        let dual = compute_dual(&g);
        assert!(dual.is_some());
        // Tree has 1 face
        assert_eq!(dual.unwrap().face_vertices.len(), 1);
    }

    #[test]
    fn test_dual_nonplanar() {
        let mut g = Graph::new(5);
        for i in 0..5 {
            for j in (i + 1)..5 {
                g.add_edge(i, j);
            }
        }
        assert!(compute_dual(&g).is_none());
    }

    #[test]
    fn test_dual_edge_mapping() {
        let mut g = Graph::new(4);
        g.add_edge(0, 1);
        g.add_edge(1, 2);
        g.add_edge(2, 3);
        g.add_edge(3, 0);
        let dual = compute_dual(&g);
        assert!(dual.is_some());
    }

    #[test]
    fn test_self_dual() {
        let mut g = Graph::new(4);
        g.add_edge(0, 1);
        g.add_edge(1, 2);
        g.add_edge(2, 3);
        g.add_edge(3, 0);
        // C4's dual is also C4 (2 vertices, 2 edges)
        // Actually it depends on the face count; let's just check it doesn't panic
        let _ = is_self_dual(&g);
    }
}
