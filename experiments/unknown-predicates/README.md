# Experiment: UNKNOWN predicate survey

Status: **Phase 1 complete** — negative result, zero UNKNOWNs found across 162 polytopes.

## Motivation

The numerical algorithm (Appendix A) uses three-valued predicates
(TRUE / FALSE / UNKNOWN) for adjacency and positivity checks.
UNKNOWN means the numerical evidence is inconclusive.
The certified capacity value is a lower bound on the true maximum;
UNKNOWN candidates above it represent unresolved uncertainty.

**Key question:** Do UNKNOWN predicate states actually appear in
practice on our polytope datasets? If not, the certified value *is*
the answer and no further work is needed.

## Phase 1 results

Ran the random-sweep (70 polytopes, F=5..12) and lagrangian-products
(92 polytopes, 10 regular polygon pairs) through the production
algorithms, tracking certified vs uncertain capacity and β_min.

**Finding: zero UNKNOWNs.** Numerical gap = 0 for all 162 polytopes.

β_min distribution:
- Random polytopes: 6.7e-4 to 1.2e-1 (well above ε = 1e-12)
- Lagrangian products: 6.2e-12 to 3.5e-1
- Near-miss: `pair_4x4_18deg` has β_min = 6.2e-12 (6× above threshold)

Conclusion: algorithm is empirically exact at f64 precision.
Phase 2 is not needed.

## Pipeline

```bash
cd experiments/
cargo run --bin unknown_predicates --release    # → unknown-predicates.jsonl
python3 unknown-predicates/unknown_predicates.py  # → beta_min histogram
```

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
  (Having a higher verified Q value earlier doesn't help cut off
  branches in hk2017, since the search is exhaustive over all pairs.)

### Strategy B: Alternative to SVD for the KKT solve

The SVD accumulates floating-point error from the input.
An alternative solver (e.g. QR with iterative refinement, or a
direct LU solve with residual correction) might produce a beta
with smaller error, shrinking the UNKNOWN band around zero.

Open question: which solvers are less sensitive to incoming
floating-point errors in the system matrix? The system is small
(|S|+5 square, typically 10-20), so cost is irrelevant.

### Strategy C: Check if beta corresponds to a Reeb orbit

**Status: probably useless, listed for completeness.**

The Q-maximization problem relates to the Reeb orbit problem as
follows. The parametrization is:

| Q-problem | Reeb orbit problem |
|---|---|
| Reeb velocities scaled to 1x | Reeb velocities = 1x |
| Period = 1 | Period = T |
| Action = Q = 1/(2T) | Action = T |
| Objective = 1/(2Q) | Objective = I_K = T for pure Reeb velocity curves |

The Q-problem is (related to) the dual problem, but we never check
translation of the curve, nor that it lies on the surface of
partial K. So:

Open question: Does "critical w.r.t. Q" imply the time-lengthened
curve z is critical w.r.t. I_K?

- If yes: all critical admissible beta correspond to a Reeb orbit,
  and all inadmissible ones don't. But then a barely-inadmissible
  beta corresponds to a barely-non-Reeb orbit, so the Reeb orbit
  check doesn't give us more discriminating power than the
  positivity check already does.
- If no: the relationship is more complex and needs further analysis.

**Worry:** Checking whether beta corresponds to a Reeb orbit
literally buys us nothing in practice. A spurious beta (UNKNOWN
admissibility) would correspond to a curve that's almost-but-not-
quite Reeb, which is just as hard to distinguish numerically as
the beta itself.

The check *might* sometimes detect a beta that very clearly has no
corresponding Reeb orbit, but it's unclear when this would happen
if the positivity check is already inconclusive.

### Strategy D: Perturb the polytope to break degeneracy

If UNKNOWN verdicts arise from polytope degeneracy (e.g. beta_i
exactly zero due to symmetry), slightly perturbing the polytope
might cause:
- Spurious Q values to vanish (beta becomes clearly inadmissible)
- Non-spurious Q values to persist (beta stays clearly admissible)

**Problem:** This only works if the original polytope is non-degenerate
and the UNKNOWN arose from near-degeneracy. If the polytope IS
degenerate (e.g. symmetric), perturbation changes the problem.
A degenerate polytope's capacity genuinely involves the degenerate
pair, and perturbation gives the capacity of a *different* polytope.

Could still be useful as a heuristic: if perturbing in multiple
random directions consistently gives the same capacity, the
original UNKNOWN was probably spurious.

### Strategy E: Rerun with tighter tolerance

Recompute the KKT solve for UNKNOWN pairs using a much smaller
epsilon tolerance. This doesn't eliminate UNKNOWN—it shrinks the
band. A beta that was UNKNOWN with epsilon = 1e-10 might become
clearly TRUE or FALSE with epsilon = 1e-14.

**Limitation:** If beta_i is exactly zero (degeneracy), no tolerance
shrinkage resolves it. The UNKNOWN persists until the degeneracy
is handled explicitly (e.g. by recognizing that the pair is covered
by a smaller pair, which is what the dismissal machinery already does).

## Open mathematical questions

1. Is the Q-maximization problem exactly the dual of the Reeb orbit
   variational problem, or only related? The parametrization
   (vel=1x, period=1, Q=1/(2T)) suggests dual, but we don't check
   curve translation or surface containment.

2. Does "critical admissible w.r.t. Q" imply "corresponds to a Reeb
   orbit"? If yes, Strategy C is provably useless (the orbit check
   has exactly the same numerical sensitivity as the admissibility
   check).

3. For the Q-maximization: do non-maximal critical points also
   correspond to orbits? Do non-critical feasible points ever
   correspond to orbits? (This determines whether the maximum-Q
   orbit is always found by the algorithm, or whether other orbits
   could be missed.)

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
