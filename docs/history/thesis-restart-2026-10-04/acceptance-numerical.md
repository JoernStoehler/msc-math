# Independent numerical and flow-graph acceptance — 4 October 2026

Owners: implementation `/root/acceptance/numerical_contract`; independent acceptance `/root/acceptance`; independent flow mathematics/code review `/root/acceptance/mathematical_review`. Operational integration accepted both bounded packages. No commits were made by these workers; checkpoint work and unrelated edits were preserved. The candidate manifest records source identity; this review does not require or claim a new commit.

## Repaired high-severity API contract defect

`general_qp_action_window` promises one exact positive KKT witness per word in the requested inclusive action window within the transition-pruned general stream. Before correction it reused direct and inherited positive-curvature pruning, which only excludes supports of maxima. A stationary point can have positive beta and Q while failing that curvature test.

The current hypercube control has exact capacity 4 and 142 positive words in this stream: 2 at action 4 and 140 at action 8. At multiplier 2 the old API returned 2, omitting all 140 action-8 words. This is a fresh current-code measurement. The historical development packet's 128 discarded positive candidates belongs to a different audit and is not this reproduction.

One explicit omitted word in `known_polytopes::hypercube()` indexing is `(0,2,4,1,5,6,3,7)`. Equal weights `beta_i=1/8`, `mu=(1/8,1/8,-1/8,-1/8)`, and `xi=-1/8` satisfy exact closure, normalization and KKT stationarity, with `Q=1/16` and action 8. The tangent vector `(1,-1,1,1,1,-1,-1,-1)` satisfies `Cz=0` and `z^T H z=8`. The acceptance owner independently verified these equations using Python `Fraction`, so the counterexample does not rely only on the Rust solver used by both comparator and implementation.

The correction passes an explicit `prune_nonmaximizers` choice into the selected route. Scalar capacity and multiplier exactly 1 retain the original curvature paths. Multipliers greater than 1 disable both direct discovery and cyclic-inheritance pruning. Certified beta/Q feasibility decisions, exact fallback and exact inclusive action cutoff remain active. Independent inspection found no other maximum-only reject path: short-word determinants reject only affine inconsistency; rank-deficient short supports fall back to the complete exact solver; verified inverse-defect rejection is based on certified beta or Q signs.

The new regression compares the complete positive hypercube spectrum at multipliers 1, 199/100 and 2, checking exact values, ordered words, positivity and inclusive endpoint behavior. Its singular short-support case tests nonunique multipliers; it is not evidence that continuous beta families are enumerated. The API promises one witness per word.

Validation:

- Before-fix regression failed: expected 142, observed 2.
- Implementer: public API suite 15/15 passed, 19.31 seconds.
- Implementer: selected-route correspondence 4/4 passed, 7.25 seconds, including scalar bounds, exact F5 window, F10 derivatives and product witnesses.
- Acceptance owner independently reran the hypercube regression: passed, 19.08 seconds.
- Acceptance owner independently checked the rational stationary/positive-curvature counterexample and reviewed the narrow code diff.
- Root integration gate on the final numerical edits: formatting and workspace compile passed; the owning release library suite passed 376 tests, with 23 ignored and none failed, in 28.47 seconds. See [repository-checks.json](repository-checks.json) and [library-tests.log](library-tests.log).

Consumer search located tests, an API example, and `experiments/sys-landscape/gradient-ascent-observed-general/main.rs` using multiplier 101/100 then a 1001/1000 filter. That packet explicitly separates its current schema-v2 producer from the retained historical schema-v1 thesis panel, which uses the legacy route. No tracked v2 output was found by the bounded scan. We did not rerun broader producers or declare every cached computation unaffected. There is no demonstrated scalar-capacity error; wider windows can now cost more, with no broad performance benchmark.

## Repaired low-severity singular classifier defect

Fresh review of Chapter 5 and the rational flow implementation found that `solve_singular_fixed_tube` discarded all-zero coefficient rows before checking consistency. The rank-one system

```text
0*x + 0*y = 1
1*x + 0*y = 0
```

has no solution, but could be treated as a fixed line and conservatively reported as an unsupported positive singular case. Both equation orders are covered by the regression, using determinant-one affine shears over a bounded square, together with consistent fixed-line controls. The repair checks contradictory zero rows before selecting a nonzero row. It also subsumes the earlier rank-zero consistency check.

The new test failed before correction and the two focused singular tests passed afterwards. The independent mathematical reviewer inspected code and before/after results and accepted the correction. Reachability of this exact helper input from a genuine polytope tube was not established. The finite-orbit-regular theorem excludes the singular branch; no wrong scalar capacity was shown. The corresponding README now states the short-sublist independence check and links to current thesis paths.

## Fresh Chapter 5 correspondence review

The independent mathematical reviewer checked the conditional correctness proof, simple-minimizer dependency and presentation-chamber genericity. Exact rational calculations independently confirmed the ten symplectic pairings, determinant witnesses `-10643/600` and `1/168`, and repeated-AB specializations through word length 10. Those repeated specializations prove a rational function is nonzero; they are not claimed as genuine simple geometric inputs.

Runtime inspection covered canonical cycles, trusted input incidence/sign matrices, short-sublist independence validation, short-word exclusion, exact bounded polygon feasibility, balanced tube composition, positive-time reconstruction and dynamic action cutoffs. Every temporary cutoff derives from a realized positive orbit, preserving all candidates within the eventual minimum plus threshold. Singular handling is conservative outside the theorem's regular class.

The pre-correction bounded release flow suite passed 39 tests, with 4 explicitly expensive exhaustive F7 tests left ignored; runtime 35.73 seconds. Coverage includes exact F5/F6 capacity comparisons, retained windows, cutoff equality, primitive/composition semantics, selected F7 words, degenerate domains and input rejection. The new correction received its focused tests rather than repeating unrelated expensive checks.

Reviewed identities: Chapter 5 `509dcc77594b8a495e569abf871fe8c44644ffcbb1dc727cbaed4e8e357cc27a`; `formal/flow-graph-real-algorithm.tex` `2ca0dd6d55b515c760d10659ece0624e2332cb74849f6a3821e8e2fc88585138`; `exact_search.rs` `d5e02288ca84c4e55392e79cee607dc567e03d602f5c462b7329e10bee180786`; repaired `exact_tube.rs` `1e796704d345dd4b29df9b47771eb443982ba1b626c2fbdd4a849b3cf6fc6c4a`.

## Evidence and limits

Raw receipts, logs, exact counterexample and patches are retained in [action-window-contract/](action-window-contract/) and [singular-fixed-line/](singular-fixed-line/), with hashes in [verification-manifest.json](verification-manifest.json). The owning-library gate was run after both repairs; its broader pass does not upgrade the stated mathematical or reachability boundaries. No whole Rust implementation proof, all-input numerical equivalence, general raw-input validation, full empirical rerun or human acceptance is asserted. The final PDF manifest separately binds the reader-facing claims and pages.
