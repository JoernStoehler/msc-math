# Rotated-Pentagon Chapter Companion

Status: chapter-local maintenance and review map, not mathematical source
truth. The active reading surface is `thesis/09-rotated-regular-polygons*.tex`.

Purpose: preserve the source chain, computation boundary, reviewed
interpretation, and concrete reopen conditions for the rotated-pentagon side
result without making a later owner reconstruct the proof packet.

## Result and proof spine

For a centered circumradius-one regular pentagon `P_5`, the chapter proves

```text
sys(P_5 x_L R(theta)P_5)
  = ((5 + 2 sqrt(5))/10) sec(d(theta))^2,
d(theta) = distance(theta, (pi/5) Z).
```

The proof has four load-bearing moves:

1. rotation, reflection, and equal-factor swap reduce to
   `0 <= theta <= pi/10`;
2. the word `(3,8,1,0,5,6)` with its displayed positive weights gives the
   candidate action and hence the upper bound;
3. the product finite-enumeration theorem supplies at least one capacity
   maximizer in the alternating family with two or three blocks of each factor
   type, and the exact Sage certificate excludes every generated positive KKT
   competitor below the candidate on a dense open subset;
4. Hausdorff continuity fills all algebraic specialization parameters and the
   endpoints, after which symmetry covers every real angle.

The finite-enumeration theorem is existential. Neither the theorem nor this
chapter claims that every minimizing orbit has two or three bounces.

## Source hierarchy and routes

1. Active theorem/proof:
   `thesis/09-rotated-regular-polygons*.tex`.
   The Sage-source subsection contains cleaned proof-facing excerpts for
   comparison with the formulas; it is an audit surface, not a second
   executable owner.
2. Product finite enumeration and QP interface:
   `thesis/04-haim-kislev-quadratic-program.tex`, especially
   `thm:lagrangian-product-finite-enumeration` and the active-word/KKT bridge.
3. Exact proof source and run:
   `experiments/regular-products/pentagon-rotation-formula-proof/`.
   Read its `README.md`, executable source, and full stdout in that order.
4. Literature:
   `papers/hko2024/counterexample.tex` for the HKO pentagon/value;
   `papers/hk2017/EHZ-polytopes.tex` for the finite QP;
   `papers/ch2021/s1_introduction_and_main_results.tex` for Hausdorff
   continuity. Cached paper sources are immutable.
5. Symmetry and older derivation material:
   `formal/lagrangian-product-rotation-symmetry.tex` is useful source;
   `formal/pentagon-rotation-capacity.tex` is explicitly stale and is not proof
   status.
6. Empirical figure owner:
   `experiments/regular-products/rotated-regular-products/analyze.py` and
   `lagrangian-products-5x5.jsonl`. The thesis copy is
   `thesis/working/rotated-regular-polygons/lagrangian-products-5x5.png`.

Active prose, the QP theorem, executable predicates, current full run, and
primary papers outrank this companion.

## Exact-certificate contract

The verifier constructs the pentagon and every candidate itself; it reads no
Rust witness or empirical data. Its default unlimited run checks:

- exact geometry in `(q1,q2,p1,p2)` order;
- active-branch action, positive weights, and systolic prefactor;
- 7,200 two-bounce and 43,200 three-bounce cyclic representatives before
  pruning;
- exact constancy of every mixed transition sign on
  `0 < theta < pi/10`;
- 3,340 distinct surviving raw words;
- exact KKT/sign-cell classification into six accepted outcomes;
- rejection of every manual-review outcome.

The run header records SageMath version, assertion state, source SHA-256, and
arguments. The program aborts if assertions are disabled. A full source change
requires regeneration of `executable_proof.full.stdout.txt`; a prefix run is
only a smoke check.

The classifier works generically over `K(t)`. KKT rank changes, rational roots,
and poles contribute only finitely many exceptional parameters. The thesis,
not the executable, closes them and the endpoints by capacity continuity.

## Empirical boundary and figure decision

The retained chapter figure overlays independently computed floating-point QP
samples with the proved exact profile. It shows the discovery pattern and the
sample/theorem agreement. It does not exclude branch crossings and is not a
proof input.

The chapter also retains the sampled enumerated KKT-branch landscape from
`experiments/regular-products/pentagon-rotation-empirics/`. Its canonical Rust
artifact freezes 3340 raw words at 9 degrees and solves them on a 73-angle
grid. The Rust and Sage families agree by count, not by an independently
retained set comparison. Every word/angle pair records one of admissible,
numerically inadmissible, indeterminate, or solve failure. Whole-profile
grouping collapses words only when all 73 sampled statuses and all actions
rounded to ten decimal places agree. One member's whole sampled sequence is
then drawn for each group; there is no angle-wise selection or splicing.
The thesis copy is
`thesis/working/rotated-regular-polygons/enumerated-kkt-branch-landscape.pdf`.
This figure and its sampled presence table explain the finite problem but do
not prove ordering or exact feasibility-set topology.

The broad polygon-pair sweep, labeled-pentagon diagram, orbit projection, and
sampled three-bounce action plot were removed from the chapter because they did
not reduce the audit burden for this theorem enough to justify their page and
attention cost. Their producer artifacts remain available under
`experiments/regular-products/`.

## Reviewed findings

- HKO use `sum dp_i wedge dq_i = -omega_0`. Translating conventions by
  `(q,p) -> (q,-p)` changes their raw `R(-pi/2)P_5` to the thesis-convention
  `R(pi/2)P_5`. The chapter now states this rather than silently identifying
  the raw angle.
- The product finite-enumeration dependency is sufficient. The generator fixes
  one selected q-block only to choose a cyclic starting representative; it
  does not quotient by reversal or pentagon dihedral symmetry.
- The full certificate proves the comparison away from finitely many algebraic
  specialization parameters. Continuity, not the code, fills those parameters
  and the endpoints.
- The previous stdout did not identify source/environment and Python
  optimization could disable proof checks. The verifier now fails fast when
  assertions are disabled and prints a run-identity header.
- The previous non-runnable excerpt file quoted stale implementation names. It
  is now a short function-level audit route without duplicated code.
- A final cold-reader pass found an imprecise periodicity justification, an
  understated implementation-audit surface, undersized figure typography, and
  a table interrupting that explanation.  All four were repaired and rechecked
  on the rendered PDF.
- The final mathematical/source review independently confirmed the symmetry,
  constants, active branch, generator coverage, fail-closed classifier, and
  continuity close.  It found a blocks-per-factor wording error and the stale
  retained transcript; the wording was repaired and the current unlimited run
  replaced the transcript with matching source digest and 3340/3340 success.
- That retained run records the installed SageMath 10.7, while the current
  devcontainer Dockerfile pins 10.8.  On 2026-07-14 Jörn classified this as a
  nonblocking reproducibility follow-up, not a mathematics or Kai-review gate.
  The later reproducibility pass may either rerun under the release image or
  align the Dockerfile with the retained environment.
- Read-only inspection of the active bounce-mechanism, active-resampling,
  width-shortcut, bounce-distribution, and alternative-generator worktrees
  found no accepted theorem evidence that changes this chapter. The
  difference-body shortcut is an unmerged conditional theorem target and is
  not used here.
- Main advanced during final verification.  Its accepted HKO correction uses
  the raw \(-\pi/2\) representative; the chapter now states explicitly that
  convention translation gives \(+\pi/2\) and simultaneous reflection
  identifies the two representatives.  The intervening archive/licensing
  commits and HKO correction do not alter the finite-enumeration dependency.
- On 2026-07-14 Jörn accepted the finite-enumeration dependency as proved:
  the classical two-/three-bounce result for the planar product billiard,
  together with the simple active-orbit reduction, places a capacity minimizer
  in the enumerated family.  The extensive empirical agreement is corroboration
  only and is not needed for this implication.
- Jörn rejected language that describes proved/cited mathematical inputs as
  merely “trusted”.  The chapter now distinguishes proof and citation from the
  theorem-specific code-to-mathematics audit and from ordinary reliance on
  SageMath as a standard exact computer-algebra system.  At his request it
  copies cleaned Sage excerpts into the thesis so Kai can inspect the
  correspondence directly.
- Final focused re-audits after adding those excerpts found the mathematical
  enumeration interface clean.  The verifier audit requested the previously
  hidden geometry, disjointness, transition-sign, and singular forced-zero
  predicates; all are now displayed, together with the accepted-status set.
  A rebuilt rendered-reader pass found the expanded listing readable and the
  proof/software boundary clear.  SageMath 10.7 now has its standard software
  citation.
- Independent inspection of the full executable still needs the stable
  repository or Zenodo locator once the thesis release is fixed.  This is a
  publication-provenance dependency owned by the Published Code and Data
  chapter, not a current mathematical or chapter-review blocker.
- The example competitor `(0,5,3,8,1,7)` has constant positive weights and an
  exact positive action gap on `0 <= theta < pi/10`, with equality only at the
  right endpoint. Its branch has a strict internal action minimum at
  `tan(theta) = tan(pi/5)/3`, illustrating branch geometry hidden by the final
  lower envelope.
- Independent review checked the example competitor's block convention, KKT
  weights, `Q`, action, both gap formulas, endpoint tie, and internal minimum.
  It found only an ambiguity in the classifier-path sentence; the repaired
  wording no longer suggests that every other word takes an early exit.
- The empirical packet's bounded spike retained all 10020 word/angle outcomes
  in 0.815 seconds. The canonical run retained 243820 outcomes in 13.397
  seconds: 29164 admissible, 214367 numerically inadmissible, 289
  indeterminate, and zero solve failures. The 289 indeterminate outcomes all
  occur at the right endpoint.
- A read-only packet review rejected the first draft's use of the shared
  solver's uncertified action-bound diagnostics and its self-attested
  canonical label. The final analyzer makes no bound-derived ordering/tie
  claim and requires the exact 73-angle canonical grid; the repaired packet
  passed re-review.
- Cold review of the integrated pages initially found that display clipping
  could be mistaken for sampled inadmissibility and that 40 profile groups were
  not visibly reconciled with 3340 raw words. Jörn then rejected the repaired
  draft because its "fixed representative" explanation remained opaque and
  the crowded status-count panel had visible overlaps. The final two-page unit
  gives the two action panels the full figure, states the 73-status/ten-decimal
  grouping rule in prose, uses uniform lines rather than multiplicity styling,
  and moves sampled-presence and right-endpoint statuses to a separate table.
  Focused rereview found no remaining overlap, semantic, or pagination defect
  at whole-page scale.

Independent review used for the candidate: primary-paper/citation audit,
finite-enumeration coverage audit, exact-verifier audit, mathematical/source
review, worked-branch mathematical review, empirical-packet/provenance review,
and cold-reader/rendered-PDF review. These reviews are agent evidence; Jörn's
acceptance of the finite-enumeration dependency is recorded separately above
and does not by itself record Kai's acceptance.

## Reopen conditions

Reopen the mathematics or certificate explanation if any of the following
occurs:

- `thm:lagrangian-product-finite-enumeration`, the QP word convention, or the
  active-word KKT implication changes;
- the Sage block generator, transition predicate, KKT classifier, active word,
  coefficient field, or expected counts change;
- the executable source digest and retained full-run header disagree;
- the thesis release is fixed but the exact packet is still identified only by
  a repository-relative path rather than its stable archive locator;
- a review finds a non-finite specialization set or a failure of the
  continuity close;
- Jörn revises the accepted finite-enumeration decision, or Kai rejects that
  route or requests a different proof-by-computation audit surface;
- an accepted and merged bounce/difference-body result materially simplifies
  the active-branch derivation or changes the theorem statement.

Reopen either empirical figure if its data/producer changes, a thesis copy
ceases to match, or rendered review shows that the sample/exact distinction is
not perceptible at normal page scale. Reopen the branch landscape specifically
if the Rust/Sage word-family relationship, four-way status semantics,
whole-profile grouping rule, or shared solver's admissibility behavior changes.

## Remaining stakeholder gate

Jörn has accepted the theorem-strength finite-enumeration dependency.  The
active chapter now makes the exact, source-identified Sage classification
visibly correspond to the mathematical predicates and supplements it with an
exact example competitor plus a separately labelled empirical landscape.  The
chapter remains subject to his writing review and, ultimately, Kai's review.
No current agent audit identified a theorem gap requiring a weaker statement.
