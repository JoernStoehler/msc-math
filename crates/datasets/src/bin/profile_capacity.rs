use datasets::known_polytopes::{hypercube, crosspolytope};
use hk2017::ehz_capacity_pruned;

fn main() {
    let polytopes = vec![
        ("hypercube", hypercube().polytope),
        ("crosspolytope", crosspolytope().polytope),
    ];

    for (name, p) in &polytopes {
        eprintln!("Profiling {}...", name);
        for _ in 0..10 {
            let _ = ehz_capacity_pruned(p);
        }
    }
}
