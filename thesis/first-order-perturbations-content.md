# First-Order Variation Content Notes

Status: maintenance companion for
`thesis/06-first-order-perturbations.tex`. The active chapter is the source
candidate; this file records its support routes, accepted boundary, and reopen
conditions rather than duplicating the mathematics.

## Chapter purpose and boundary

The chapter connects the finite capacity algorithms to HKO and search by
distinguishing three objects in a fixed labelled dual-row chart:

1. a nondegenerate positive Haim--Kislev maximizing branch, whose value and
   systolic upper function are smooth and have an envelope-theorem derivative;
2. a finite lower envelope of nondegenerate branches, which gives the actual
   directional derivative only under explicit local branch-coverage and gap
   hypotheses;
3. an arbitrary smooth positive feasible beta section, which gives a smooth
   upper function without claiming that the section remains optimizing.

The third object is the bridge to HKO. The HKO proof uses 26 exact feasible
upper functions touching `sys` at the base point and a first-order spanning /
positive-convex-relation certificate on the transverse slice. It does not
differentiate a complete family of nearby optimizing `sys_sigma` branches.

The chapter does not prove a complete directional theory for arbitrary
non-generic polytopes. Zero weights, appearing supports, singular KKT systems,
optimizer continua, redundant listed rows, and changing face combinatorics
remain outside the nondegenerate theorem unless a separate feasible-upper-function
argument suffices.

## Load-bearing source routes

- Global finite formula, normalization, word convention, and the distinction
  between fixed-word KKT data and a global maximum:
  `thesis/04-haim-kislev-quadratic-program.tex`; primary source
  `papers/hk2017/EHZ-polytopes.tex`, especially Theorem 1.1 and the feasible
  weight upper bound near source lines 487--489.
- Fixed row chart and systolic-ratio symmetries:
  `thesis/02-preliminaries-ehz-capacity.tex` and
  `thesis/07-hko-local-maximum-chart-reduction.tex`.
- Nondegenerate KKT branch and implementation formula background:
  `formal/capacity-derivatives.tex`,
  `formal/capacity-smoothness-classification.tex`, and
  `crates/symplectic/src/derivatives.rs`. The relevant formal blocks remain
  marked `unverified`; the active chapter therefore states and proves the
  needed nondegenerate result directly rather than citing those blocks as accepted
  theorem source.
- Arbitrary non-generic classification and why base support pruning does not
  establish first-order completeness:
  `formal/sys-first-order-local-behavior.md`. Its semialgebraic catalogue route
  is a draft assessment with an unchecked external citation chain, so the
  thesis mentions it only as an in-principle heavier route and does not state
  its candidate theorem.
- Feasible sections and the finite upper-function criterion used by HKO:
  `formal/hko-feasible-section-upper-branches.tex`,
  `thesis/07-hko-local-maximum-exact-certificate.tex`, and
  `experiments/hko-local-maximum/theorem/README.md`.
- Historical comparison and omission check only:
  `thesis/legacy/sys-first-order-regular-case.tex`. No legacy theorem status or
  genericity claim was imported.

## Accepted mathematical choices

- Use the explicit quadratic objective `Q_sigma` from the active QP chapter;
  do not inherit conflicting minimize/maximize wording from old formal notes.
- State the nondegenerate branch condition as full constraint rank, positive weights
  and value, constrained stationarity, and negative definiteness on the
  constraint kernel. This makes the branch the global maximum on that affine
  constraint space and makes the saddle-point system nonsingular.
- Express the derivative through the KKT multiplier and the differential of
  the constraint map. This avoids making the implementation's rowwise formula
  a prerequisite for understanding the theorem.
- Use the direct finite-minimum directional derivative at smooth covered ties.
  Do not call `min_j <grad U_j,h>` a Clarke generalized derivative; the current
  Rust helper name is not thesis terminology.
- Derive the volume row formula directly as
  `DV[h] = -sum_i S_i/||a_i|| <xbar_i,h_i>` on a stable simple chart.
- Present the finite upper-function criterion independently of HKO, then let
  the next chapter instantiate it with the exact 26-row certificate.

## Empirical and implementation status

The chapter retains only evidence whose role is stable:

- `crates/symplectic/src/derivatives.rs` has a volume-gradient central-
  difference regression and an ignored release-mode fixed-word capacity-
  gradient central-difference regression;
- `crates/symplectic/src/exact/derivatives.rs` compares the exact rational and
  f64 capacity-gradient formulas on a rational simplex fixture;
- the HKO theorem verifier checks its feasible-section derivative rows exactly,
  while `experiments/hko-local-maximum/empirical/` remains falsification and
  sanity-check evidence rather than theorem support;
- `experiments/sys-landscape/gradient-ascent-observed-general/` supports
  bounded finite-step progress and cost on twelve deterministic `F=10` starts,
  but not an endpoint or local maximum;
- `experiments/dev-sys-prediction/` and most of
  `experiments/dev-gradient-ascent/` remain active development surfaces and are
  not thesis evidence by themselves.

The 2026-07-14 chapter-owner run executed the three named derivative tests in
release mode, including the ignored fixed-word capacity test; all passed.

## Deliberate omissions and reopen conditions

- Reopen finite-distance branch-window prose only after a promoted,
  reproducible result packet fixes the base-point panel, branch-window policy,
  finite radii, prediction errors, and target-branch miss rate.
- Reopen a theorem for arbitrary non-generic directional behavior only if a
  downstream retained claim requires it, or if the semialgebraic source chain
  and active-germ coverage are proved at thesis cost.
- Reopen explicit per-row capacity-gradient coordinates only if a reader or
  code-facing passage needs them; the multiplier differential currently gives
  the cleaner mathematical statement.
- Reopen a figure only for a concrete reader failure. The current distinction
  is algebraic and is carried by the feasible-section lemma, the envelope
  display, and the upper-function criterion; no existing experiment figure is
  publication evidence for this distinction.

## Review record

The current candidate passed separate mathematical/source-strength and
unprimed reader-path reviews on 2026-07-14.

- The mathematical reviewer found no sign or factor error in the volume,
  feasible-section, KKT-envelope, systolic, finite-minimum, or positive-convex
  formulas. The review did find ambiguous branch coverage, a locally scoped
  chart hypothesis, overbroad finite-step wording, and an unsupported
  semialgebraic existence phrasing. The chapter now assumes the stable chart
  throughout, states `Q_max = max_j q_j` explicitly, names eight accepted trace
  iterations, and presents the semialgebraic catalogue only as a possible
  route.
- The unprimed reader recovered the intended nondegenerate/non-generic/HKO
  distinctions. It found that “regular” collided with the preceding
  flow-graph terminology, that “upper branch” blurred feasible sections with
  optimizing branches, and that an affine symmetry-tangent direction was
  confused with its group orbit. The chapter now says “nondegenerate branch,”
  states the tangent-space conclusion precisely, and the active HKO handoff
  consistently says “feasible section” and “upper function.”
- The integrated PDF was inspected at whole-page scale through the
  flow-graph--first-order--HKO transition. No figure or table was added: the
  central distinctions were legible in the definitions and formulas, and an
  added summary table worsened the final-page break without adding a new
  relation.
