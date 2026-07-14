# QP numerical evidence: wide-row Rust producer

The experiment has one observation boundary: Rust `observe(polytope, sigma)`
produces one wide `RawRow`. It calls the current production KKT saddle solver,
exact rational solver, geometry/transition construction, capacity route,
volume/sys, derivative, and orbit-recovery APIs directly. A grouped aggregate
row is emitted per case. Python only computes errors, formula coverage,
predicate categories, and reports.

Run the representative broad packet with:

```bash
cd experiments/numerics/qp-error-bounds
bash run.sh
```

The packet includes known simplex and hypercube complete-small transition
populations, an embedded ordinary seeded original-rational F=5 source record,
the pinned 1,294-row transition stream, four separate HKO stored-dyadic named
contexts, the embedded product tie fixture, and the pruning-roundoff fixture.
Capped populations are labelled as
capped; they do not support global HK-capacity claims. Every row carries target
identity, lifecycle events, route counters, raw/corrected Q, exact values when
the oracle is feasible, derivative and recovery fields when the production API
accepts the candidate, and narrow unavailable reasons otherwise.

`raw_rows.jsonl` and `aggregates.jsonl` are the source evidence. The compact
`formula_inventory.json` is the retained 101-entry source audit; `analyze.py`
also publishes a metadata-complete registry for its 17 packet-local formulas.
It evaluates only formulas whose implementation status and required atoms
support a stated target, recording a narrow unavailable reason for the rest,
alongside exact error coverage and offline volume/sys,
ranking, derivative/recovery, and predictor buckets. Route counters are
explicitly case-population totals repeated on rows, not per-sigma recall.
`validate.py` checks row counts, unique identities, run/source consistency,
exact encodings, predicates, inventory identity, aggregate minima, named
population coverage, target/cohort/source separation, and the full lifecycle.
The compact [`coverage_ledger.json`](coverage_ledger.json) records source
pointer, selection, oracle, completeness, intended question, and status for
each population plus explicit source-audit gaps. Q16 algebraic transfer remains unavailable without a genuine
algebraic/Sage oracle; all feasible rational/dyadic surfaces are produced
directly by Rust rather than represented as packet-local solver code.

The observation boundary is unconditional: rejected or singular f64 rows keep
their least-squares beta/mu/xi proposal, raw Q, residual, SVD rank/singular
values, eigenvalues, inertia, exact assembled matrices, and narrow route state.
Exact row reduction separately records consistency, rank/nullity, and a
particular beta witness. `exact_beta_predicate` is strict-positive feasibility
(`true`/`false`/`unavailable`); the sign classification of a row-reduction
particular vector is retained separately and is never used as a beta-error
reference. Target/center and population-filter joins are enforced by the
schema and validator.

Formula evaluations are atom-matched: the retained `omega0` matrix is the
oriented antisymmetric form (separate from the symmetric QP `H`), and the KKT
correction is the solver's direct residual term. Interval, product-route, and
fallback/timing formulas remain unavailable when their endpoint or route atoms
are not retained; a center plus an error scalar is not treated as an interval.
