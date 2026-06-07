# graph-planarity

Planar graph algorithms for Rust. Pure `std` — no external dependencies.

## Features

- **Planarity testing** — Euler's condition, K₅/K₃,₃ subdivision detection
- **Kuratowski subgraph detection** — Identifies K₅ and K₃,₃ obstructions
- **Planar embedding** — Combinatorial embedding with face traversal
- **Dual graph** — Construct dual graph from planar embeddings
- **Euler's formula** — Verification, face counting, component analysis

## Usage

```rust
use graph_planarity::{Graph, planarity, kuratowski, euler, dual};

let mut g = Graph::new(4);
for i in 0..4 { g.add_edge(i, (i + 1) % 4); }

let result = planarity::is_planar(&g);
println!("Is planar: {}", result.is_planar);

let euler = euler::euler_formula(&g);
println!("V={}, E={}, F={}", euler.vertices, euler.edges, euler.faces);

let kuratowski = kuratowski::detect_kuratowski(&g);
println!("Has obstruction: {}", kuratowski.found);
```

## License

MIT
