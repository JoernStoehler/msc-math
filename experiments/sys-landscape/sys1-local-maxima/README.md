# Empirical Local-Maximum Screen at Known `sys = 1` Bodies

This experiment asks whether a finite local search finds an increasing
direction from each currently identified `sys = 1` representative. It belongs
to `sys-landscape`, rather than `sys-datascience`, because the objects are a
small theory-selected panel and the output is a local geometric diagnostic,
not a learned or population-level model.

## Question and decision

For each known equality representative, does a nearby fixed-facet polytope
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

From the repository root:

```bash
cargo run --release -p exp-sys-landscape --bin sys1-local-maxima -- --canonical
uv run --script experiments/sys-landscape/sys1-local-maxima/analyze.py
```

Use `--smoke` for the pentagon control with one quotient basis pair and its two
structured-family probes at one radius. Smoke output goes to a temporary
directory and is plumbing evidence only.

Canonical artifacts are written under `artifacts/`. `probes.jsonl` is the raw
finite-evaluation evidence; `bases.jsonl`, `radius-summaries.jsonl`,
`summary.json`, and `REPORT.md` are generated views. `run-provenance.json`
records the command, Git state, constants, and source paths.
