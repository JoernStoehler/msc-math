# Quotient-Aware Endpoint Diagnostic Discussion

## Mathematical heuristic stationarity definition

For a valid labelled fixed-`F` dual-vertex state `a`, the diagnostic forms the tangent span of the four translations, positive scaling, and ten `sp(4,R)` generators. All five retained states have tangent rank `15`. The slice is the Euclidean orthogonal complement at the base state, of dimension `4F-15`: `9` for the generic six-facet states and `25` for HKO.

At each relative radius `r` in `1e-3, 1e-4, 1e-5`, the producer polls both signs of every vector in a deterministic orthonormal basis of that slice, with absolute step norm `r ||a||_2`. A state passes the packet's finite stationarity condition at one radius only when all `2(4F-15)` recomputed states are valid, keep the same facet incidence signature, and none has positive raw `Δsys`. There is no relative-gain cutoff: the table reports the best raw change and change per step. Passing this condition is only finite, basis-dependent evidence.

## Direct control outcomes

The two retained negative controls were selected because their next literal-gradient update has positive full-`sys` change. Both show positive quotient-basis directions at all three radii, so the diagnostic does not confuse these two ordinary improvable states with endpoints. HKO2024, the exact-theorem positive control, has no positive nominal-scalar direction among all `50` directions at any radius. Its least-negative margin is `-7.44336e-06`. This is an operational nominal-scalar consistency check, not a successful capacity-sign discriminator and not evidence for the HKO theorem, whose exact certificate remains authoritative.

| State role | Relative radius | Directions | Improving | Max Δsys | Max Δsys / step |
| --- | ---: | ---: | ---: | ---: | ---: |
| Negative: initial | 1e-03 | 18 | 9 | 0.00103917 | 0.428012 |
| Negative: initial | 1e-04 | 18 | 9 | 0.000104238 | 0.429333 |
| Negative: initial | 1e-05 | 18 | 9 | 1.0427e-05 | 0.429466 |
| Negative: mid-trajectory | 1e-03 | 18 | 9 | 0.00381477 | 1.52278 |
| Negative: mid-trajectory | 1e-04 | 18 | 9 | 0.000381536 | 1.52302 |
| Negative: mid-trajectory | 1e-05 | 18 | 9 | 3.81541e-05 | 1.52304 |
| Unknown: global best | 1e-03 | 18 | 5 | 0.00281924 | 1.01783 |
| Unknown: global best | 1e-04 | 18 | 9 | 0.000392008 | 1.41527 |
| Unknown: global best | 1e-05 | 18 | 9 | 4.35145e-05 | 1.571 |
| Unknown: terminal best | 1e-03 | 18 | 8 | 0.0021967 | 0.872662 |
| Unknown: terminal best | 1e-04 | 18 | 9 | 0.000451386 | 1.79318 |
| Unknown: terminal best | 1e-05 | 18 | 9 | 4.51549e-05 | 1.79383 |
| Positive: HKO2024 | 1e-03 | 50 | 0 | -0.000744066 | -0.190357 |
| Positive: HKO2024 | 1e-04 | 50 | 0 | -7.44312e-05 | -0.19042 |
| Positive: HKO2024 | 1e-05 | 50 | 0 | -7.44336e-06 | -0.190426 |

Across the packet, `366/366` probes were valid, every listed dual point stayed extreme, and no probe changed the base incidence signature. All trajectory targets agree exactly under the current scalar route; the HKO recomputation differs from its known-capacity target by `1.33e-15`. `analyze.py` independently reconstructs unknown selection, negative-control witnesses, raw direction norms, signed pairs, slice Gram matrices, cross-radius direction identity, row denominators, target-difference arithmetic, and compact summaries. It range-checks the producer's orbit-projection and geometry/capacity diagnostics rather than independently recomputing them.

## Unknown-state outcomes

- `unknown_global_best_so_far` is the highest valid `sys` row across the frozen 3,142-row six-start evaluation. It has positive directions at all three radii (respectively `5/18`, `9/18`, and `9/18`) and therefore fails the finite stationarity condition at every tested resolution.
- `unknown_terminal_best_so_far` is the highest complete iteration-100 state whose trajectory best occurs at iteration 100. It also has positive directions at all three radii (`8/18`, `9/18`, and `9/18`) and fails at every tested resolution.

These outcome-selected states are diagnostic unknowns, not independent optimizer validation. The first is explicitly a best point on an oscillatory trajectory; the second tests whether a high terminal best is any closer to stationarity. Neither may be called a heuristic local maximum from this packet.

## Quotient and branch-completeness limitations

The derivative-free poll was chosen because HKO's `44` nonsingular active KKT rows span only rank `23` of its `25`-dimensional quotient; the exact theorem needs singular feasible upper sections. The poll therefore does not assume that the base active sigma list contains every right-active or singular germ. It does, however, rely on the current `MinimaSafe` full-capacity scalar at each finite perturbed state and does not establish limiting branch-germ completeness.

Capacity-bound audit: all 216 generic probe minimum-action intervals collapse; 136/150 HKO intervals do not, with maximum width 3.44094. The broad HKO bounds come from ill-conditioned returned candidates near the singular control. Consequently the HKO finite-poll signs are operational central-scalar observations, not independently certified capacity inequalities. This numerical limitation does not weaken the generic unknown-state failures, but it prevents using the HKO poll as new mathematical support. A diagnostic disagreement with HKO would have to be treated first as an evaluator/branch-completeness failure.

The Euclidean slice and its coordinate-ordered Gram-Schmidt basis are one local gauge. The signed basis is positive spanning but is not dense on the quotient sphere; nonsmooth directional ascent can exist between tested axes. The affine slice is tangent-transverse only at the base. The finite radii do not prove behavior below `1e-5 ||a||_2`, and f64 state coordinates are rationalized exactly rather than representing unknown exact optimizer endpoints. Discrete facet relabellings and HKO's finite symmetry group create no extra tangent directions.

## Evidence thresholds

Calling a future state a **heuristic local maximum** should require at least: successful negative controls; nominal consistency with HKO plus explicit disposition of its capacity intervals; valid fixed-facet geometry; a complete signed quotient-basis poll with no improvement at several shrinking radii; a materially richer deterministic or seeded quotient-direction cover (or branch-aware gradient sampling) that also finds no improvement; exact raw margins and direction coverage; and repetition after the polisher's stopping state is frozen. A finite no-improvement scan remains heuristic.

**Theorem-grade local maximality** requires a local chart and an exact certificate controlling every transverse direction, such as HKO's feasible upper branches with exact rank and positive convex relation. No amount of finite polling alone supplies that implication.

## Decision and next optimizer experiment

Stop treating these two frozen high states as endpoint candidates. The next useful experiment is a safeguarded quotient-basis polisher seeded at them: at each iteration evaluate the signed quotient basis, accept the largest positive full-`sys` move, and shrink the radius only when a complete poll has no improvement. Retain every raw poll and stop after no improvement at three declared radii. Then rerun this endpoint packet plus a richer direction cover on the frozen polished states. This directly tests whether the parallel optimizer policies merely reach high values or can actually remove the ascent directions observed here.

## Reproduction

From the repository root; the frozen compact trajectories are ordinary tracked files:

```bash
cargo run --release -p exp-dev-gradient-ascent \
  --bin dev-gradient-ascent-quotient-endpoint-diagnostic -- \
  --out-dir experiments/dev-gradient-ascent/quotient-endpoint-diagnostic/artifacts \
  --threads 8

uv run --script \
  experiments/dev-gradient-ascent/quotient-endpoint-diagnostic/analyze.py
```

`run-provenance.json` hashes all `36` selection inputs, the producer, analyzer,
and the generation-time Cargo manifest. The analyzer verifies the exact
manifest hash when unchanged; after unrelated binaries are integrated, it
instead verifies the endpoint binary and dependency contract so those additions
do not invalidate retained evidence. `poll-directions.jsonl` is the raw
evidence; `states.jsonl` and `radius-summaries.jsonl` are compact generated
views. The figures are generated directly from the validated rows.
