<!--
Purpose: distilled planning note for the witness-search program in the sys>1 search.
Context: replaces the imported memo as the file TASKS.md should cite.
-->

# Witness-Search Program

Distilled from the imported memo.
Use this file for tracker pointers and session prep; do not cite the raw import from
`TASKS.md`.

## 1. Model And Motivation

- Treat the search as a finite minimax problem `sys(K) = min_{c in S_F} s_c(K)`.
- Fixed-witness branch evaluation is cheap; exact witness search is the factorial bottleneck.
- Existing repo evidence already covers the generic-interior regime:
  - random sampling stayed below `sys = 1`: `crates/exp-sys-landscape/random-sample/logbook.md`
  - fixed-F ascent improved tails but found no new `sys > 1`: `crates/exp-sys-landscape/gradient-ascent-general/logbook.md`
  - random `F -> F+1` continuation gave only marginal gains: `crates/exp-sys-landscape/variable-f-ascent/logbook.md`
- The new program focuses on small active witness sets, exact-search warm starts, and structured continuation.

## 2. Witness Oracle Instrumentation

- Goal: upgrade exact witness search from "best permutation only" to a reusable local-structure oracle.
- Add optional outputs:
  - top-`m` witnesses by branch value
  - all witnesses within additive gap `Delta`
  - incumbent list encountered during exact search
- Add optional inputs:
  - one incumbent witness
  - a small incumbent witness set
- Persist per exact-evaluated point:
  - exact `sys(K)`
  - minimizing witness
  - near-active witness set
  - branch values, gradients, and Hessians for the near-active witnesses
  - exact-search runtime diagnostics
- Bundle the benchmark bank into the same session. It is a dependency artifact, not its own tracker item.

## 3. Reuse And Safe Prefilter Calibration

- Goal: quantify how long a local witness cache remains useful under nearby perturbations and how often a partial witness set can safely reject a point.
- Compare witness sets built from:
  - minimizer only
  - top-`m`
  - within-gap `Delta`
  - parent-cache and hybrid caches
- Measure:
  - exact-minimizer hit rate vs step size
  - upper-bound gap `U_A(K) - sys(K)`
  - safe rejection rate from `U_A(K) < 1`
  - exact-call reduction factor
- Fold permutation-neighborhood search and the warm-start benchmark into this program line, not separate tracker headers.

## 4. Reduced-Model Ascent

- Goal: spend many cheap local steps on a reduced witness model and call exact search only when needed.
- First variant:
  - soft-min / log-sum-exp smoothing on a small witness set
  - serious-step / null-step outer loop
- Second variant:
  - min-norm convex-hull QP as a stationarity and support diagnostic
- Acceptance criteria:
  - compare against an exact-evaluate-every-step baseline on the same seeds
  - report best exact `sys`, exact-call count, and wall-clock

## 5. Witness-Guided Continuation

- Goal: replace weak random `F -> F+1` continuation with witness-guided vertex splitting.
- Use branch-gradient disagreement to choose which dual vertex to split and in which direction.
- Lift parent witnesses into the child `F+1` problem instead of restarting from scratch.
- Compare directly against current random-addition baselines:
  - `crates/exp-sys-landscape/variable-f-ascent/`
  - `crates/exp-hko-local-maximum/cut-and-ascent/`
- This is the natural successor to the current continuation baseline, not a side quest.

## 6. Structured Families

- Goal: search symmetry-constrained low-dimensional families that generic iid sampling may miss.
- Start with orbit-union families in labeled dual-vertex coordinates.
- Use the reuse, prefilter, and reduced-model machinery inside those families.
- If this becomes productive, extend it to box-pruning with witness upper bounds over parameter regions.
- Keep combinatorial/order-type diagnostics as supporting logging inside this line rather than a standalone tracker item.

## 7. Bundling Rules For TASKS.md

- Session-sized tracker items:
  - witness oracle instrumentation + benchmark bank
  - witness reuse + safe prefilter calibration
  - reduced-model ascent on witness sets
  - witness-guided `F -> F+1` continuation
  - symmetry-family search
  - box-pruning on structured families
- Leaf-sized extras to fold into neighboring sessions:
  - permutation-neighborhood search
  - exact warm-start benchmark
  - combinatorial diagnostics logging
- Keep this witness-search program separate from:
  - the LICCA density/falsification bundle
  - the HKO local-max evidence bundle
  - numerics and thesis write-up bundles
