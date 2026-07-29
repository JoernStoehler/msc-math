# Local-Maxima Classification Pilot Report

Status: bounded source audit plus a retained 83-probe control-calibration
panel. This report does not select the general optimizer and does not make a
new theorem claim.

## Evidence ladder used here

The classifications below keep these levels separate:

1. an exact theorem gives or excludes a local maximum in a named chart or
   family;
2. a finite recomputed probe finds a better point;
3. a body survives a declared finite falsification suite;
4. a heuristic positive certificate controls a declared local model;
5. a theorem controls every nearby transverse direction;
6. a finite named facet-addition family finds or misses an improvement; and
7. a theorem controls changing facet counts.

A result at one level is not promoted to the next.

## Selected-body source and status audit

This is the selected panel, not a project-wide inventory.

| Body/state | Chart and normalization | Current strongest status | Sources |
| --- | --- | --- | --- |
| HKO | Exactly ten facets; labelled dual-vertex chart; quotient by translations, positive scaling, and the identity-component linear symplectic action | Proved locally maximal in the ten-facet Hausdorff model. The exact finite predicate uses 26 feasible upper sections, quotient covector rank 25, and a positive exact relation. It does not control adding facets. | `thesis/07-hko-local-maximum.tex`; `experiments/hko-local-maximum/theorem/README.md`; `formal/hko-feasible-section-upper-branches.tex` |
| Controlled HKO quotient perturbations | Ten facets; deterministic Euclidean quotient-basis directions; relative HKO distance `1e-3` | This pilot finds nominally improving return moves at fractions `1e-1`, `1e-2`, and `1e-3` of the HKO distance for two perturbations. This is a finite improving pattern, not a proof that the perturbed points are not local maxima. | `artifacts/rows.jsonl`; `artifacts/summary.json` |
| Rotated-pentagon equality crossing | Ten-facet `P5 x_L R(theta)P5` family; crossing angle determined by the exact profile | Exactly not locally maximal: the proved profile continues from `sys=1` into `sys>1`. The pilot recovers the improving sign at all three tested angular radii. | `thesis/09-rotated-regular-polygons-pentagon-profile-theorem.tex`; `artifacts/summary.json` |
| Rotated-pentagon improving-side point | Same family, at crossing angle plus `1e-2` radians, below `pi/10` | Exactly not locally maximal within the named family because the exact profile is strictly increasing toward `pi/10`. The pilot recovers improvement at steps `1e-3`, `1e-4`, and `1e-5`. | same exact-profile theorem; `artifacts/summary.json` |
| Regular triangle--hexagon at angle zero | Nine-row labelled Lagrangian-product chart modulo the same continuous symmetries | Numerical reconstruction gives `sys` approximately one; no exact equality proof was recovered by the existing packet. It survived 372 declared finite probes with no material nominal improvement, but the capacity intervals were broad. Conjectural fixed-facet local maximum. | `experiments/local-maxima-check/README.md`; `experiments/local-maxima-check/artifacts/REPORT.md`; `experiments/regular-products/rotated-regular-products/lagrangian-products-3x6-6deg.jsonl` |
| Regular square--square at angle `pi/4` | Eight-row labelled Lagrangian-product chart modulo the same continuous symmetries | Numerical reconstruction gives `sys` approximately one; no exact equality proof was recovered. It survived 348 declared finite probes with no material nominal improvement, but the capacity intervals were broad. Conjectural fixed-facet local maximum. | same packet and `experiments/regular-products/rotated-regular-products/lagrangian-products-4x4-6deg.jsonl` |
| CH2021 displayed six-vertex body | Exact rational six-vertex, nine-facet body; fixed-nine-facet dual chart across adjacent combinatorial cells | Exactly `sys=1` by exhaustive rational HK2017 evaluation. It survived 366 finite probes, all of which entered adjacent combinatorial cells and decreased nominal `sys`; the capacity intervals were broad. The packet does not prove local maximality. | `experiments/verification/ch2021-six-vertex/README.md`; its `report.json`; `experiments/local-maxima-check/artifacts/REPORT.md` |
| Ordinary random `F=6` control | Deterministic accepted random dual-vertex state; master seed `20260729`, attempt `3`; fixed six-facet quotient chart | This pilot finds 9 improving directions among the 18 signed quotient-basis probes at relative radius `1e-4`; all 9 improvements are interval-separated. This is a finite rejection of stationarity under the declared poll. | `artifacts/rows.jsonl`; `artifacts/summary.json`; `artifacts/run-provenance.json` |
| Two archived high `F=6` optimizer states | Outcome-selected frozen states; fixed six-facet quotient chart | Both have positive signed quotient-basis directions at every tested relative radius `1e-3`, `1e-4`, and `1e-5`. They are explicitly rejected by that finite stationarity diagnostic and are not endpoint candidates. | `experiments/dev-gradient-ascent/quotient-endpoint-diagnostic/artifacts/DISCUSSION.md`; its `summary.json` |
| Ten legacy `F=10` states formerly called “local maxima” | Old `gradient-ascent-general` endpoints, used as starts for the former variable-facet packet | The historical design called `general_0` through `general_9` local maxima, with source values up to `0.9030`, but supplied only stopped legacy ascent states. The current debt note rejects endpoint/local-maximality use. The old `F=11` continuation improved 45 of 50 named placements. These states plausibly explain Jörn's recollection, but no candidate-specific fixed-`F` certificate was recovered, so that identification remains uncertain. | `experiments/sys-landscape/legacy-ascent-continuation-debt.md`; provenance-only historical source `git show 2ea00eb1a:research/sys-landscape/design/variable-f-ascent.md` |

Search scope for the remembered fixed-`F` maxima included current experiment,
formal, thesis, documentation, recovery-planning, optimizer worktrees, Git
commit subjects, historical variable-facet design, and `/tmp/joern` planning
material under the terms `local max`, `local-max`, `maxima`, `maximizer`,
`endpoint`, and the named bodies. No current candidate-specific proof or
stationarity packet beyond HKO and the three conjectural equality bodies was
found. This is a reported search result, not proof of absence.

## Facet-addition status

Facet addition is a separate classification axis.

- HKO has a retained 20-placement cut-then-ascent `F=11` panel. None exceeded
  the HKO value and every final state retained the added facet. This is a
  finite negative for that sampler and search policy, not changing-facet local
  maximality.
- The HKO neighborhood packet also has a named random cutting-facet sampler.
  It is an implementation/control source, not a general answer.
- Legacy continuation from the ten old gradient-ascent states commonly
  improved them after adding one facet, but every final value remained below
  one and the starting states were not credible local maxima.
- No selected-body facet-addition result was recovered for triangle--hexagon,
  square--square, or CH2021.

Sources:
`experiments/hko-local-maximum/empirical/m11-ascent/README.md`,
`experiments/hko-local-maximum/empirical/neighborhood-sampling/README.md`,
`experiments/sys-landscape/variable-f-ascent/README.md`, and
`experiments/sys-landscape/legacy-ascent-continuation-debt.md`.

## Broad falsifier and positive-check comparison

This is the broad set considered in the pilot.

| Check | Positive value | Negative/miss value | Main risk or prerequisite | Shared surfaces |
| --- | --- | --- | --- | --- |
| Exact structured family direction | Can prove a body is not locally maximal when a formula gives an improving germ | Only controls the named family | Needs a body-specific exact profile or chart | Product constructors, full evaluator, exact family source |
| Signed quotient-basis poll | A valid recomputed improving point cheaply rejects the finite stationarity condition without active-set completeness | Basis- and radius-dependent miss; oblique or thin cones can be missed | Quotient gauge, valid geometry, full scalar route | `directions.rs`, raw row schema, evaluator |
| Seeded richer quotient directions or MADS-like positive spanning polls | Better directional coverage; positive still immediately useful | Finite miss remains heuristic | Costs roughly dimension times full evaluations; metric dependent | Same quotient and evaluator code |
| Return tests after controlled perturbation | Tests attraction/local recovery and can find small finite improving moves | Finite return miss does not establish a basin or local maximum | Needs frozen reference/equivalence rule and several shrinking scales | Same raw state/probe schema |
| Active/right-active gradient convex-hull separation | A separating direction can propose a strong first-order falsifier; an interior positive relation can become a positive certificate | Passing a sampled gradient test is weak | Requires complete right-active, singular, and zero-weight germ control | Branch rows, quotient projection, convex solver |
| Active-facet coverage/support move | An omitted facet proposes a cheap inward move; full scalar validation can falsify a candidate | Coverage is only necessary and does not certify a maximum | Numerical active data may be incomplete; current derivation is inside `unverified` | Active words, support chart, volume row, full evaluator |
| Branch-informed finite continuation and finite-gap models | Can follow a thin improving ridge missed by generic directions | A miss depends on branch proposer, gap, and domain coverage | Branch changes and singular domains; model must be validated by full `sys` | Existing prediction/continuation packets |
| Higher-order probes along first-order-flat directions | Can falsify a putative maximum or prioritize an exact flat-direction analysis | Sampled negative curvature is not negative definiteness | First-order model and numerical scale must be credible | HKO second-order packet pattern |
| Feasible touching upper functions with rank and positive relation | With exact data and neighborhood validity, can prove fixed-chart local maximality; HKO is the working example | Failure of one row bank does not reject the body | Body-specific feasible sections, exact evaluation, symmetry slice, uniform neighborhood argument | HKO theorem architecture |
| Named facet insertion and local reoptimization | An improvement can falsify changing-facet local maximality and may produce a new high body | Finite miss is weak without a detection argument | Placement families, effect scale, larger-chart optimizer | HKO `m11`, legacy variable-`F` code |

The active-facet derivation in
`formal/active-orbit-facet-coverage.tex` is a proposal source only: it is
inside `unverified`, excludes nonsimple support-wall points from its
unconditional chart, and explicitly requires complete active limiting data
before numerical use.

## Ranked short list

The current priority order for classification, independent of general
optimizer selection, is:

1. **Body-specific structured continuation.** It is cheapest and has the
   strongest possible negative outcome. The pentagon control shows why it must
   be included whenever a family is known.
2. **Full-scalar signed quotient poll plus return tests at shrinking radii.**
   It is active-set-agnostic, shared across all fixed-facet bodies, and this
   pilot shows clean control separation. Add deterministic symmetry/family
   directions before adding more random directions.
3. **Complete right-active/upper-branch recovery for a survivor.** This is the
   first route with meaningful positive fixed-`F` certification potential.
   Numerical convex-hull conditioning can triage, but theorem use requires
   exact rows and neighborhood validity like the HKO packet.
4. **Facet-coverage support moves as proposers, followed by full-scalar
   validation.** Cheap and explanatory when they succeed; do not treat
   numerical coverage as a positive certificate.
5. **Named facet insertion only after a credible fixed-`F` survivor is
   selected.** Positive value is high, but finite negative value is poor.

Generic extra random directions rank below structured and branch-informed
directions because the existing rotated-pentagon control's quotient/random
polls missed its real thin improving family.

## Retained pilot result

Command:

```bash
cargo run --release -p exp-local-maxima-check \
  --bin local-maxima-control-calibration -- --canonical
```

The producer wrote 83 valid probes across six cases in 13.97 seconds. No probe
changed incidence.

| Case | Declared probes | Improving | Interval-separated | Result at exact strength |
| --- | ---: | ---: | ---: | --- |
| HKO reference | 50 signed quotient axes at relative radius `1e-4` | 0 | 0 | Survives this finite suite; separate theorem remains authority |
| HKO perturbation, quotient axis 0 | 3 return fractions | 3 | 0 | Finite nominal return improvement at every tested fraction |
| HKO perturbation, quotient axis 1 | 3 return fractions | 3 | 0 | Finite nominal return improvement at every tested fraction |
| ordinary random `F=6` | 18 signed quotient axes at relative radius `1e-4` | 9 | 9 | Finite interval-separated improvement found |
| pentagon equality crossing | 6 signed structured rotations | 3 | 3 | Exact profile proves non-local-maximality; detector recovers all improving signs |
| pentagon improving-side point | 3 forward structured steps | 3 | 3 | Exact profile proves non-local-maximality; detector recovers all three |

For the two HKO perturbations, the smallest tested return fraction `1e-3`
changed nominal `sys` by `3.743e-6` and `3.813e-6`, respectively. Their
capacity intervals do not separate, so these are central-scalar observations.
For HKO itself, the largest signed-basis change was
`-7.443e-5`. For the random control, the best interval-separated change was
`+7.844e-5`.

The first new result is therefore modest but useful: the same cheap
active-set-agnostic machinery separates a theorem-positive control, two
controlled nearby states, a structured exact nonmaximum, and an ordinary
random state. It does not yet strengthen the three equality-body survivor
claims.

## Verification and cost

- `--smoke`: 8 probes across five cases; 8/8 valid; 2.26 seconds producer
  execution. The first clean release build took about 74 seconds and is not
  part of smoke execution time.
- retained `--canonical` pilot: 83 probes across six cases; 83/83 valid; 13.97
  seconds producer execution and 16.00 seconds including the incremental
  build.
- original parent-packet `sys1-local-maxima --smoke`: 4/4 probes valid; 0.58
  seconds producer execution.
- `cargo test -p exp-local-maxima-check`: 3/3 unit tests passed across the two
  binaries.
- `cargo fmt --check`: passed.
- package-only Clippy with dependencies excluded: passed with warnings denied.
  The first strict dependency-inclusive command stopped on six pre-existing
  `symplectic` lints (`result_large_err`, `new_ret_no_self`, and
  `large_enum_variant`); no pilot lint was reported.
- an independent `jq` aggregation reconstructed all case counts, improvement
  counts, extrema, validity, and incidence-change counts from `rows.jsonl` and
  agreed with `summary.json`.

No run approached the ten-minute experiment stop condition. No exact
arithmetic was added: the pilot uses binary64 exploration and cites existing
exact theorem packets for theorem-strength statements.

## Next positive route for survivors

The next experiment is better determined only for one survivor at a time:
recover a complete candidate set of touching/right-active germs in its actual
chart, project their derivative rows to the symmetry quotient, and measure
rank, convex-hull margin, singularity, and exact-reconstruction cost before
attempting a theorem packet.

Among triangle--hexagon, square--square, and CH2021, CH2021 has the best exact
base arithmetic but the hardest local chart because every existing finite
probe enters an adjacent combinatorial cell. Triangle--hexagon has the cleanest
observed quadratic signed-basis behavior but lacks a recovered exact equality
proof. Therefore no production follow-up is launched here. The next cheap
source check should compare the exact active maximizer sets and local chart
complexity for triangle--hexagon and CH2021, with a stop after numerical
rank/conditioning and exact-data availability are known. Expected cost is
minutes of source work plus at most a few minutes of binary64 diagnostics; a
theorem attempt is a separate decision.
