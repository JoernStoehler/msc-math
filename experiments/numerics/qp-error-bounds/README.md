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

## Retained-exact route evaluation

Run the focused route packet with:

```bash
bash experiments/numerics/qp-error-bounds/run_retained_exact.sh
```

It writes `artifacts/retained-exact/raw_rows.jsonl` (source evidence),
`analysis.json` (derived strata/cost report), and `summary.md`. The Rust
producer evaluates ordinary generated F5, pinned q4:p5, triangle×square tie,
and pruning-roundoff. Each row records the supplied transition/product-block
stream, f64 True/Indeterminate/rejected strata, current `MinimaSafe`, exact
rechecking of every retained candidate, and an exact-all reference where
available. The fixed action window is exact `[minimum, 21/20 · minimum]`.

Exact values use the rational target represented by stored binary64 bits. No
algebraic HKO oracle is available. Exactness is over the f64-retained set;
exact-all is completeness only relative to the named supplied stream. Active
words are candidate words, not proven distinct physical orbits. This packet
does not provide exact multipliers, derivatives, recovery data, global HK
candidate recall, or a production-consumer migration.

The runner requires a clean git tree before deleting/regenerating outputs. It
records the reachable source commit and full git tree object in `manifest.json`;
the generated artifact may then be committed as a separate child commit. The
source tree snapshot is the producer provenance and does not recursively hash
the generated artifact.

Timing fields are scoped: candidate generation includes route enumeration and
f64 solves but excludes fixture/exact-geometry setup and compilation;
`MinimaSafe` includes ordinary aggregation/fallback only; retained exact
includes exact resolution of every retained candidate; exact-all includes
complete-stream exact enumeration, solving, and sorting. Fixture construction,
Python analysis/validation, and compilation are excluded from all row timers.

## v2 multi-centre soundness trial

Run the separate v2 surface with:

```bash
bash experiments/numerics/qp-error-bounds/run_soundness_v2.sh
```

It writes `artifacts/soundness-v2/`.  Each raw lifecycle row names its solver
centre explicitly (current saddle/eigen accepted centre, unconditional SVD,
projected critical and max-margin, local LU/QR, and one QR refinement), then
compares only against the same-word stored-rational exact positive witness.
The formula registry, derived formula observations, and policy rows are
separate artifacts.  Policies distinguish the unchecked current f64 heuristic,
exact retained candidates, one-shot f64-anchored selective windows, and exact
supplied-stream replay.  The heuristic policy reports only its own f64 scalar,
minimizers, and window; its exact fields are unavailable. Exact policies record
every attempted resolution separately from positive-Q accepts. A policy's
active words are not physical-orbit sets.

The packet includes ordinary F5/F8 controls, a product tie, the pinned q4:p5
stream, the four named HKO stress words, and the hypercube boundary word. HKO
exactness is only for the stored binary64 rational target; algebraic HKO
transfer is explicitly unavailable. Small streams are deliberately capped and
their policy exactness scope stays stream-relative.
