//! # graph-planarity
//!
//! Planar graph algorithms for Rust. Pure `std` — no external dependencies.
//!
//! ## Modules
//!
//! - [`planarity`] — Planarity testing (simplified Hopcroft-Tarjan approach)
//! - [`kuratowski`] — Kuratowski subgraph detection (K₅ and K₃,₃)
//! - [`embedding`] — Planar embedding construction
//! - [`dual`] — Dual graph construction from planar embeddings
//! - [`euler`] — Euler's formula verification and face counting

pub mod planarity;
pub mod kuratowski;
pub mod embedding;
pub mod dual;
pub mod euler;

/// A graph for planarity algorithms.
#[derive(Clone, Debug)]
pub struct Graph {
    n: usize,
    adj: Vec<Vec<usize>>,
}

impl Graph {
    /// Create a new graph with `n` vertices.
    pub fn new(n: usize) -> Self {
        Self {
            n,
            adj: vec![vec![]; n],
        }
    }

    /// Number of vertices.
    pub fn vertex_count(&self) -> usize {
        self.n
    }

    /// Add an undirected edge.
    pub fn add_edge(&mut self, u: usize, v: usize) {
        assert!(u < self.n && v < self.n, "Vertex index out of bounds");
        if !self.adj[u].contains(&v) {
            self.adj[u].push(v);
        }
        if u != v && !self.adj[v].contains(&u) {
            self.adj[v].push(u);
        }
    }

    /// Check if edge (u, v) exists.
    pub fn has_edge(&self, u: usize, v: usize) -> bool {
        self.adj[u].contains(&v)
    }

    /// Get neighbors.
    pub fn neighbors(&self, v: usize) -> &[usize] {
        &self.adj[v]
    }

    /// Get the degree.
    pub fn degree(&self, v: usize) -> usize {
        self.adj[v].len()
    }

    /// Get all edges (u, v) with u < v.
    pub fn edges(&self) -> Vec<(usize, usize)> {
        let mut edges = Vec::new();
        for u in 0..self.n {
            for &v in &self.adj[u] {
                if u < v {
                    edges.push((u, v));
                }
            }
        }
        edges
    }

    /// Number of edges.
    pub fn edge_count(&self) -> usize {
        self.edges().len()
    }

    /// Maximum degree.
    pub fn max_degree(&self) -> usize {
        (0..self.n).map(|v| self.degree(v)).max().unwrap_or(0)
    }

    /// Get the adjacency list.
    pub fn adjacency(&self) -> &[Vec<usize>] {
        &self.adj
    }
}
