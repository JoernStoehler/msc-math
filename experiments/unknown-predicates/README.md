# UNKNOWN Predicate Survey

Do UNKNOWN predicate states (inconclusive numerical evidence) actually appear in practice on our polytope datasets? If not, the certified capacity value is the answer and no further work is needed.

## Status
Complete (Phase 1). Phase 2 not needed in practice.

## Design

- Input datasets: random-sweep (70 polytopes, F=5..12) and lagrangian-products (92 polytopes, 10 regular polygon pairs)
- Total: 162 polytopes run through production algorithms with UNKNOWN logging
- Tracks certified vs uncertain capacity and beta_min per polytope

## Key findings

- **29 UNKNOWNs found, all in Lagrangian products, all f64 rounding noise**
- Numerical gaps range from 4.44e-16 to 4.93e-12, all below 1e-10
- Random-sweep polytopes: zero UNKNOWNs
- Beta_min well above epsilon=1e-12 for all polytopes (random median 4.5e-2, Lagrangian median 1.7e-1)
- Algorithm is empirically exact up to machine-precision rounding at f64

## Files

| File | Purpose |
|------|---------|
| `unknown_predicates.rs` | Rust binary: runs datasets with UNKNOWN logging |
| `unknown_predicates.py` | Python: beta_min histogram |
| `unknown-predicates.jsonl` | Dataset (162 rows: certified vs uncertain capacity per polytope) |
| `unknown-predicates.tex` | Thesis writeup |
| `unknown_predicates_beta_min.png` | Figure: beta_min distribution histogram |

## Run

```bash
cd experiments/
cargo run --bin unknown_predicates --release    # -> unknown-predicates.jsonl
python3 unknown-predicates/unknown_predicates.py  # -> beta_min histogram
```

## Known limitations

- Only tested on polytopes with F <= 12; higher facet counts untested
- Phase 2 (re-solve at higher precision) not performed for the 29 UNKNOWN cases
- The 29 UNKNOWNs appear benign but are not individually confirmed resolved

## Phase 2: Resolution strategies (only if Phase 1 finds UNKNOWNs)

Each strategy targets a different source of UNKNOWN verdicts.
They are independent and can be tried in any combination.

### Strategy A: Higher-precision floats for affected code paths

Use `f128` or a multi-precision library (e.g. `rug`) only for the
code paths that produce UNKNOWN verdicts:

- **Adjacency:** Recompute vertex-facet incidence at higher precision.
  Only needed if adjacency UNKNOWNs appear. The adjacency
  precomputation is done once per polytope and is not performance
  critical.
- **Positivity filter (beta admissibility):** Recompute the KKT solve
  at higher precision for the specific (S, sigma) pairs that got
  UNKNOWN. For hk2017, this can be done at the very end: collect all
  UNKNOWN candidates, then re-solve only those with higher precision.

### Strategy B: Alternative to SVD for the KKT solve

The SVD accumulates floating-point error from the input.
An alternative solver (e.g. QR with iterative refinement, or a
direct LU solve with residual correction) might produce a beta
with smaller error, shrinking the UNKNOWN band around zero.

### Strategy C: Check if beta corresponds to a Reeb orbit

**Status: probably useless, listed for completeness.**

### Strategy D: Perturb the polytope to break degeneracy

If UNKNOWN verdicts arise from polytope degeneracy (e.g. beta_i
exactly zero due to symmetry), slightly perturbing the polytope
might cause spurious Q values to vanish.

### Strategy E: Rerun with tighter tolerance

Recompute the KKT solve for UNKNOWN pairs using a much smaller
epsilon tolerance. A beta that was UNKNOWN with epsilon = 1e-10 might become
clearly TRUE or FALSE with epsilon = 1e-14.

## Decision tree

```
Phase 1: Run datasets with UNKNOWN logging
    |
    +-- No UNKNOWNs found --> DONE (algorithm is empirically exact)
    |
    +-- UNKNOWNs found
        |
        +-- From adjacency --> Strategy A (higher-precision incidence)
        |
        +-- From positivity (beta near zero)
            |
            +-- Due to degeneracy (beta_i = 0 exactly)
            |   --> Already handled by dismissal machinery
            |   --> If not: Strategy D (perturbation, with caveats)
            |
            +-- Due to numerical noise (beta_i near but not at 0)
                --> Strategy A (higher-precision solve) or
                    Strategy B (better solver) or
                    Strategy E (tighter tolerance)
```

## Open mathematical questions

1. Is the Q-maximization problem exactly the dual of the Reeb orbit
   variational problem, or only related?

2. Does "critical admissible w.r.t. Q" imply "corresponds to a Reeb
   orbit"? If yes, Strategy C is provably useless.

3. For the Q-maximization: do non-maximal critical points also
   correspond to orbits? Do non-critical feasible points ever
   correspond to orbits?
