# Five-Case Local-Maxima Check

This root topic compares five deliberately different evidence statuses.  Its
producer asks whether a finite local search finds an increasing direction from
three selected equality candidates and uses the rotated-pentagon crossing as a
known-positive control. HKO enters through its exact theorem packet, not as
another probe seed. The selected bodies and their evidence ladder, rather than
the hostile-search data-science pipeline, organize this packet.

## Five statuses

| Body | Value status | Local status |
| --- | --- | --- |
| regular triangle--hexagon, angle `0` | numerical reconstruction gives `sys ≈ 1`; no exact equality proof was recovered | conjectured local maximum in its fixed nine-row labelled chart modulo the `sys` symmetries |
| regular square--square, angle `pi/4` | numerical reconstruction gives `sys ≈ 1`; no exact equality proof was recovered | conjectured local maximum in its fixed eight-row labelled chart modulo the same symmetries |
| CH2021 displayed six-vertex body | exact `sys = 1`, certified by `experiments/verification/ch2021-six-vertex/` | conjectured local maximum among fixed-nine-facet nearby bodies, across adjacent combinatorial cells, modulo the same symmetries |
| HKO | exact `sys = (3+sqrt(5))/5 > 1` with theorem-facing certificate | proved locally maximal in the ten-facet space, with equality on the local symmetry orbit |
| rotated-pentagon equality crossing | exact `sys = 1` from the proved profile | exactly not locally maximal because the family continues into `sys > 1` |

The relevant symmetries are translations, positive scaling, and the
identity-component linear symplectic action. None of the conjectures concerns
adding facets or all convex bodies.

## Line status

**Decision `LMC-D1`, 2026-07-22:** close the current allocation of research
effort to the line “start from a known `sys >= 1` body, find that it is not a
local maximum, and thereby obtain a new `sys > 1` example or family not already
explained by the rotated-pentagon/HKO family.”  This is a portfolio decision,
not a mathematical claim that the three conjectural bodies are local maxima
and not a prohibition on reconsidering the line.

The reasoning is:

1. Result `LMC-R1` is the finite screen documented below.  It finds no material
   nominal or interval-separated improvement at the triangle--hexagon,
   square--square, or CH2021 targets.  Its structured probes do recover the
   known improvement at the rotated-pentagon positive control.  Thus it leaves
   three cases as conjectural local maxima, but supplies no new `sys > 1`
   route.
2. The same control also shows why `LMC-R1` cannot close the mathematical
   question: the generic quotient-basis and random probes miss a real thin
   improving set which the theory-derived structured directions find.  More
   finite misses would therefore remain weak evidence unless a new method
   addresses branch completeness or supplies a theorem-grade neighborhood
   argument.
3. The most immediate continuations do not currently justify their opportunity
   cost:
   - Densifying or rerunning the same fixed-facet probes has no identified
     claim-changing outcome.
   - Adding-facet probes have no clean negative outcome: failure to sample an
     improvement would remain inconclusive.  Jörn's planning prior for such a
     run was 2% for a new `sys > 1` case, 80% for a technically completed but
     mathematically inconclusive result, and 18% for technical abandonment.
   - A gap-aware local-linear model already exists under
     `experiments/dev-sys-prediction/`; replaying these probes through it could
     help choose a target for exact branch analysis, but would not itself prove
     local maximality or reliably produce a new positive example.  It is a
     conditional triage tool, not an unperformed default successor.
   - Exact branch-aware analysis could turn one conjecture into a theorem and
     may have direct thesis value, but that is a separately assessed proof
     project rather than a necessary continuation of this search.
4. At closeout, these continuations did not beat the available alternatives:
   reviewing/integrating existing curvature-certificate work, deciding whether
   the triangle-product theorem should enter the thesis, and thesis
   readiness/demonstration.  This comparison is time-sensitive; a future agent
   should redo it against the then-current portfolio rather than inherit its
   ranking as a permanent fact.

Reconsider `LMC-D1` when new evidence changes the scientific or portfolio
comparison, for example:

- a new, independently motivated `sys >= 1` candidate has a concrete reason to
  be non-maximal and its improving continuation would not merely recover the
  known rotated-pentagon/HKO family;
- branch data, exact formulas, or a completeness argument make a target's
  local status resolvable at competitive cost;
- a method can turn a negative outcome into materially stronger evidence than
  another finite directional miss; or
- a named thesis claim makes classification of one target valuable enough to
  beat the current alternatives.

The mere availability of more samples, another perturbation basis, a routine
rerun, or a possible implementation does not by itself defeat `LMC-D1`.

## Question and decision

For each selected equality representative, does a nearby fixed-facet polytope
have larger `sys`? A positive result selects a path for exact analysis. A
negative finite screen only prioritizes a body for branch-aware or exact
local-maximum work; it does not establish local maximality.

The current panel is:

- the regular pentagon-product equality crossing, which is known from the
  exact profile to continue into `sys > 1` and is the expected-positive
  control;
- the regular triangle--hexagon Lagrangian product at relative angle zero;
- the regular square--square Lagrangian product at relative angle `pi/4`;
- the rational CH2021 six-vertex body, reconstructed from the displayed
  vertices after exact centroid translation and polarization.

The control is not new landscape evidence. Failure to recover it blocks the
interpretation of target misses.

## Interventions

At each base, the producer recomputes the full current `MinimaSafe` capacity
scalar and then evaluates:

1. both signs of a deterministic orthonormal basis for the Euclidean
   complement of the translation, scaling, and `Sp(4,R)` tangent orbit;
2. deterministic antipodal Gaussian directions in the same quotient tangent;
3. sixteen angular directions in the two-dimensional `so(4)/u(2)` tangent,
   exponentiated to orthogonal one-parameter maps;
4. both relative-rotation directions for every regular-product seed.

The fixed-row probes use relative step norms `1e-3`, `1e-4`, and `1e-5`.
Orientation and family probes use angular radii `1e-2`, `1e-3`, and `1e-4`
radians. All directions and all finite perturbed states are retained. The
producer records exact-incidence geometry, nominal capacity and `sys`, the
capacity interval propagated to a `sys` interval, and whether the lower bound
is above one or above the base upper bound.

Raw signed changes are retained. The compact summaries call a nominal change
material only when `delta_sys > 1e-12`; this excludes binary64 equality noise
without hiding the raw value. Capacity-interval separation is reported
separately and is the stronger finite-point outcome.

This screen keeps the facet count fixed. New-facet and facet-deletion
directions are a separate stratum and are not tested here.

## Frozen predictions

These predictions were written before running the target panel:

- at least 98% probability that the structured pentagon control increases at
  all three angular scales in the direction toward the known positive region;
- 20% probability that at least one of the three non-control bodies has an
  interval-separated improving probe at all three scales of some probe family;
- 55% probability that no non-control body has such a pattern, while the
  control succeeds;
- 25% probability that nominal improvements, capacity-interval ambiguity,
  changing incidence, or strong directional dependence makes at least one
  target inconclusive.

The probabilities concern this finite diagnostic, not the mathematical truth
of local maximality.

## Claim boundary

An interval-separated point proves only an inequality for the exact rational
polytope obtained by rationalizing that finite binary64 perturbation, subject
to the current capacity-bound contract. Repeated improving points at shrinking
radii are empirical evidence for a local increasing germ, not by themselves a
proof that arbitrarily small improvements exist.

A miss is basis-, sample-, radius-, evaluator-, and fixed-stratum-dependent.
For the nonsimple CH body, generic fixed-row perturbations can enter adjacent
combinatorial cells; those are part of the local neighborhood and are retained
with an explicit incidence-change flag rather than rejected.
Theorem-grade local maximality additionally requires a genuine neighborhood
slice, complete control of right-active/singular branch germs, and an exact
upper-branch certificate or equivalent argument.

## Reproduction

The Rust producer is in `src/main.rs`; `src/seeds.rs` constructs the selected
bodies and positive control, while `src/directions.rs` constructs the quotient
and structured probe directions.

From the repository root:

```bash
cargo run --release -p exp-local-maxima-check --bin sys1-local-maxima -- --canonical
uv run --script experiments/local-maxima-check/analyze.py
```

Use `--smoke` for the pentagon control with one quotient basis pair and its two
structured-family probes at one radius. Smoke output goes to a temporary
directory and is plumbing evidence only.

Canonical artifacts are written under `artifacts/`. `probes.jsonl` is the raw
finite-evaluation evidence; `bases.jsonl`, `radius-summaries.jsonl`,
`summary.json`, and `REPORT.md` are generated views. `run-provenance.json`
records the command, Git state, constants, and source paths.
