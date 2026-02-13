/// Compute EHZ capacity of the 4D crosspolytope (16 facets) with progress reporting.
///
/// Reports progress to stderr every 10 seconds and at each m-level completion.
/// Final result (if reached) printed to stdout as JSON.
///
/// Usage: crosspolytope-experiment
use geom::known_polytopes;
use hk2017::{ehz_capacity_pruned_with_progress, total_search_space, ProgressReport};

fn report(p: &ProgressReport) {
    let total_processed = p.total_evaluated + p.total_pruned;
    let fraction = if p.grand_total > 0 {
        total_processed as f64 / p.grand_total as f64
    } else {
        0.0
    };
    let elapsed_s = p.elapsed.as_secs_f64();

    // Extrapolate completion time from fraction done
    let eta = if fraction > 1e-15 {
        let total_est = elapsed_s / fraction;
        let remaining = total_est - elapsed_s;
        format!("{:.0}s (total est: {:.0}s)", remaining, total_est)
    } else {
        "N/A".to_string()
    };

    let prune_rate = if p.m_evaluated + p.m_pruned > 0 {
        p.m_pruned as f64 / (p.m_evaluated + p.m_pruned) as f64 * 100.0
    } else {
        0.0
    };

    let status = if p.m_completed { "DONE" } else { "..." };
    let best_str = match p.best_action {
        Some(a) => format!("{a:.6}"),
        None => "none".to_string(),
    };

    eprintln!(
        "[{elapsed_s:>7.1}s] m={:>2}/{} {status:<4} | \
         this_m: {:>12} eval, {:>12} pruned ({prune_rate:>5.1}%) of {:>12} | \
         cumul: {:>12} eval, {:>12} pruned | \
         progress: {:.6}% | best: {best_str} | ETA: {eta}",
        p.m,
        p.m_max,
        p.m_evaluated,
        p.m_pruned,
        p.m_theoretical,
        p.total_evaluated,
        p.total_pruned,
        fraction * 100.0,
    );
}

fn main() {
    let kp = known_polytopes::crosspolytope();
    let f = kp.polytope.facet_count();

    eprintln!("=== Crosspolytope EHZ Capacity Experiment ===");
    eprintln!("Facets: {f}");
    eprintln!("Total search space: {} (S,σ) pairs", total_search_space(f));
    eprintln!("Starting computation...\n");

    let result = ehz_capacity_pruned_with_progress(&kp.polytope, |p| report(p));

    eprintln!();
    match result {
        Some(r) => {
            eprintln!("=== RESULT ===");
            eprintln!("Capacity: {:.10}", r.capacity);
            eprintln!("Best subset: {:?}", r.best_subset);
            eprintln!("Best permutation: {:?}", r.best_permutation);
            eprintln!("Iterations (KKT solves): {}", r.iterations);
            // JSON to stdout for programmatic consumption
            println!(
                "{{\"capacity\":{},\"iterations\":{},\"subset\":{:?},\"permutation\":{:?}}}",
                r.capacity, r.iterations, r.best_subset, r.best_permutation
            );
        }
        None => {
            eprintln!("=== NO RESULT (no valid candidate found) ===");
        }
    }
}
