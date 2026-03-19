# Unknown Predicates: Logbook

## Motivation

The capacity algorithm uses a three-valued admissibility predicate: certified, rejected, or uncertain (UNKNOWN). If UNKNOWN predicates affect the capacity result -- i.e., an uncertain orbit achieves lower action than any certified orbit -- then the algorithm's output depends on floating-point precision and Phase 2 (high-precision re-solve) would be needed. This experiment checks whether UNKNOWNs appear in practice on our datasets.

## Status

**Complete (Phase 1).** Phase 2 not needed: all 29 observed UNKNOWNs are f64 rounding noise.

## How to run

```bash
# Generate dataset
cd experiments/ && cargo run --bin unknown_predicates --release

# Plot beta_min histogram
python3 experiments/unknown-predicates/analyze.py
```

### Files

| File | Role |
|------|------|
| `run.rs` | Rust binary: runs datasets with UNKNOWN logging |
| `analyze.py` | Python: beta_min distribution histogram |
| `math.tex` | Thesis writeup (experiment, results, conclusion) |
| `unknown-predicates.jsonl` | Dataset (162 rows: certified vs uncertain capacity per polytope) |
| `unknown_predicates_beta_min.png` | Figure: beta_min distribution histogram |

## Design

- **Input datasets:** random-sweep (70 polytopes, F=5..12) and lagrangian-products (92 polytopes, 10 regular polygon pairs).
- **Total:** 162 polytopes run through production algorithms with UNKNOWN logging.
- **Metrics per polytope:** certified capacity, uncertain capacity, numerical gap, beta_min.
- **Threshold:** epsilon = 1e-12 for the admissibility predicate.

## Findings

1. **29 UNKNOWNs found, all in Lagrangian products, all f64 rounding noise.** Numerical gaps range from 4.44e-16 to 4.93e-12, all below 1e-10.
2. **Random-sweep polytopes: zero UNKNOWNs.**
3. Beta_min (margin from the uncertain threshold) is well above epsilon=1e-12 for all polytopes:
   - Random: median 4.54e-2, range [6.74e-4, 1.21e-1]
   - Lagrangian: median 1.71e-1, range [5.46e-2, 3.54e-1]
4. The algorithm is empirically exact up to machine-precision rounding at f64 on our datasets.
5. In all 29 UNKNOWN cases, the certified and uncertain capacities agree to full f64 precision.

## Known limitations

- Only tested on polytopes with F <= 12; higher facet counts untested.
- Phase 2 (re-solve at higher precision) not performed for the 29 UNKNOWN cases.
- The 29 UNKNOWNs appear benign but are not individually confirmed resolved.

## Dead ends / deferred directions

Five Phase 2 strategies were designed but not needed in practice:

- **Strategy A:** Higher-precision floats for affected code paths (adjacency, positivity filter).
- **Strategy B:** Alternative solver to SVD (QR with iterative refinement, direct LU).
- **Strategy C:** Check if beta corresponds to a Reeb orbit (probably useless).
- **Strategy D:** Perturb the polytope to break degeneracy.
- **Strategy E:** Rerun with tighter tolerance.

## Open questions

1. Is the Q-maximization problem exactly the dual of the Reeb orbit variational problem, or only related?
2. Does "critical admissible w.r.t. Q" imply "corresponds to a Reeb orbit"?
3. For Q-maximization: do non-maximal critical points also correspond to orbits?

## Related experiments

- `random-sweep`: Source of the 70 random polytopes used as input.
- `lagrangian-products`: Source of the 92 Lagrangian product polytopes used as input.
