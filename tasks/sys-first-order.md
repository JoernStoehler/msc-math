<!--
Purpose: arbitrary-polytope first-order behavior roadmap for sys.
Context: durable cache for the missing theorem/evaluator/proof/algorithm/test
surface after the 2026-04-29 failed smooth-branch draft.
-->

# Sys First-Order Roadmap

## Status

- State: map-input.
- Last updated: 2026-04-29.
- Source surfaces: `papers/hk2017/EHZ-polytopes.tex`,
  `formal/numerics/gradient/numerics.tex`,
  `formal/numerics/gradient/numerics-subdifferential.tex`,
  `formal/hko-local-maximum/second-order.tex`,
  `formal/library/algorithms.tex`,
  `formal/numerics/error-bounds.tex`.
- Refresh when: an agent proves, refutes, or sharply weakens the
  compute-once first-order evaluator theorem; HKO proof wording starts relying
  on arbitrary-polytope first-order behavior; or an implementation claims exact
  first-order `sys` behavior beyond generic smooth branches.

## Steering Cache

- [accepted 2026-04-29] The target theorem is not mere existence of directional
  derivatives. The target is a reusable first-order object:

  ```text
  compute D(a) once;
  evaluate Dsys_a(h) = Eval(D(a), h) for arbitrary directions h;
  sys(a+h) = sys(a) + Dsys_a(h) + o(||h||).
  ```

  `Eval(D(a), h)` may be direction-dependent and only positively homogeneous in
  `h`; it does not need to be linear. Why it matters: gradient-ascent and local
  maximality work need repeated direction queries without re-solving the
  original local problem for every `h`.
- [accepted 2026-04-29] Generic smooth-branch or Danskin-envelope statements
  are not acceptable as the main theorem. A theorem whose key hypothesis is
  "each active branch is `C^1`/`C^2` near `a`" assumes away the boundary and
  singular cases this task exists to handle. Such results may appear only as
  later specializations.
- [observed 2026-04-29] A weaker broad theorem appears available from the
  closed Haim--Kislev value formula plus semialgebraic local Lipschitzness:
  `sys` has finite Hadamard directional derivatives in the fixed row chart.
  Why it matters: this is background regularity, not the desired compute-once
  evaluator.
- [accepted 2026-04-29] Numerical behavior must be separated from real/exact
  claims. `f64` tests can find regressions, diagnose active-set changes, and
  calibrate heuristics, but they do not prove the real first-order theorem.

## Goal Contract

The next agent's goal is to notice the missing theorem and either fix it or
report exactly why it cannot be fixed yet. A successful answer must classify
the theorem status below. A chapter, implementation, or polished proof that
does not classify the theorem status is not a successful answer.

Accepted theorem statuses:

- `PROVED`: includes a theorem with `D(a)` computed once, `Eval(D(a), h)` for
  arbitrary directions, stated degeneracy coverage, proof obligations discharged,
  and an algorithm contract.
- `ONLY-HEAVY`: the theorem is available only through a broad finite
  semialgebraic or quantifier-elimination construction; include the complexity
  reason it is not a simple gradient-ascent object.
- `BLOCKED`: the route is not currently proved; give the smallest missing lemma
  or obstruction, and name the exact tempting substitute that failed.
- `NO-GO`: give a concrete counterexample pattern or theorem-level reason the
  desired compute-once evaluator cannot exist in the stated form.

Any `PROVED` or `ONLY-HEAVY` verdict must include a hard-case table with rows
for `beta_i=0`, limits with `beta_i^n>0 -> beta_i=0`, ray feasibility versus
linearized feasibility, singular KKT or active continua, repeated/redundant
listed rows, volume combinatorics, and exact-real versus `f64` behavior. If a
row is not handled, the verdict is not `PROVED`.

## Work Map

| item | state | value class | owner/gate | next action | source |
| --- | --- | --- | --- | --- | --- |
| Theorem feasibility split | `[map-input]` | mainline thesis if HKO proof route depends on it | agents then Jorn math | Decide which theorem surface is actually available: strong active-germ evaluator, heavy semialgebraic cell-decomposition evaluator, or no-go/complexity obstruction. The output must include exact proof obligations, not polished prose. | this file |
| Closed HK row-chart formalization | `[map-input]` | mainline thesis if theorem route retained | agents | State the closed Haim--Kislev capacity value problem in the row chart, including `beta >= 0`, actual facets versus listed rows, repeated/redundant inequalities, and the normalization between `(n_i,h_i)` and rows `a_i`. | `papers/hk2017/EHZ-polytopes.tex`; `formal/numerics/error-bounds.tex` |
| Active-germ evaluator proof attempt | `[blocked]` | mainline thesis if feasible | theorem feasibility split | Try to prove a finite object `D(a)` that stores active closed optimizer/germ data and evaluates every direction `h` without re-solving the original HK problem. Do not insert smooth-branch assumptions as the main proof. | closed HK formalization |
| Semialgebraic cell-decomposition fallback | `[map-input]` | contingent during writing or future/follow-up | agents then Jorn | Check whether quantifier elimination/cell decomposition gives a mathematically finite compute-once evaluator for `h -> Dsys_a(h)`, and record the complexity cost. This may be true but too large for a readable thesis theorem or usable algorithm. | semialgebraic/Lipschitz proof route |
| Volume first-order object | `[map-input]` | support for any theorem route | agents | Specify the matching compute-once first-order object for `V(a)=vol(K(a))`, including vertex-combinatorics changes and redundant row behavior. | `formal/library/algorithms.tex`; volume code/formal notes |
| Algorithm contract | `[blocked]` | contingent during writing | accepted theorem surface | Turn the theorem into an algorithm contract: inputs, exact arithmetic domain, stored object, evaluation cost, and failure/undecided modes if the theorem is conditional. | theorem feasibility split |
| Implementation and test suite | `[blocked]` | future/follow-up unless thesis route needs it | algorithm contract | Implement only after the theorem/contract is accepted. Tests must separate exact-real checks from `f64` diagnostics and include boundary `beta_i=0`, singular active continua, redundant rows, generic smooth points, and directions where first-order feasibility is inconclusive. | `crates/symplectic/`; `experiments/numerics/`; `experiments/hko-local-maximum/` |
| HKO dependency audit | `[map-input]` | mainline thesis | agents then Jorn | Decide whether the HKO local-max proof needs the full arbitrary-polytope theorem, only a verified HKO-specific finite evaluator, or can be worded as conditional/supporting evidence. | `tasks/hko.md`; `formal/hko-local-maximum/second-order.tex`; `research/hko-local-maximum*.md` |

## Agent Cache

- [fresh 2026-04-29] Rejected substitutes: Hadamard differentiability alone,
  raywise limit definitions, generic unique-minimizer calculus, finite minimum
  of smooth branches, and any evaluator that re-solves the original HK value
  problem for every direction.
  Refresh by: checking whether a proposed theorem literally contains `D(a)`
  computed once and `Eval(D(a), h)` applied afterward.
- [fresh 2026-04-29] Hard cases that must be addressed or explicitly excluded:
  `beta_i^n > 0` limiting to `beta_i = 0`; `beta_i = 0` with first-order
  feasibility pairing zero; active optimizer continua; singular KKT systems;
  hidden optimizer germs; listed rows that are repeated or redundant.
  Refresh by: reading the closed HK formalization and any candidate proof.
- [fresh 2026-04-29] Candidate routes:
  strong active-germ evaluator, semialgebraic cell-decomposition evaluator,
  or precise no-go/complexity obstruction. The next agent should reason through
  these routes; the cache speeds that reasoning but does not license skipping
  it after the failed smooth-branch session.
- [fresh 2026-04-29] Existing proof fragments are local and narrow:
  `formal/numerics/gradient/numerics.tex` handles generic/non-generic smoothness
  classifications with gaps; `formal/numerics/gradient/numerics-subdifferential.tex`
  discusses directionally feasible orbit subsets; `formal/hko-local-maximum/second-order.tex`
  uses a min-envelope/Danskin argument that is explicitly not yet the arbitrary
  closed-value theorem.
- [fresh 2026-04-29] Review gate before showing Jorn a PDF: a reviewer should
  try to downgrade the result to `BLOCKED` by checking whether the main theorem
  secretly substitutes Hadamard differentiability, smooth branches, ray limits,
  or per-direction optimization for the compute-once evaluator.
- [fresh 2026-04-29] This task bundle is intended to be self-contained. Do not
  open scratch artifacts to decide the theorem shape unless this file is missing
  a specific fact needed for provenance.

## Pruned / Stale

- [stale 2026-04-29] Failure provenance only:
  `scratch/sys-first-order-chapter.pdf` used a `C^2` branch hypothesis as the
  main theorem. Do not use it as source truth or as the requested chapter. That
  route assumes away the arbitrary-polytope degeneracies this bundle exists to
  handle.
