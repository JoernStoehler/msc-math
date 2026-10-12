# Searching the systolic-ratio landscape

The landscape experiments ask how numerical systolic ratios change when one
samples new bodies, moves existing facets, adds a facet, or changes symplectic
alignment. They use

\[
\widehat{\operatorname{sys}}(K)=\frac{c_{\rm num}(K)^2}{2V_{\rm num}(K)}.
\]

The threshold one is the volume–capacity conjecture's threshold in dimension
four. A finite search below it answers only what that search found. It cannot
establish the inequality for an unrestricted family, and an ascent endpoint
is not a local maximum merely because a chosen search stopped.

The physical source owner is
[`topics/empirical-studies/sys-landscape/`](sys-landscape/README.md).
That package also supplies maintained geometry/cache/ascent support to other
experiments; the package name does not make all its old producer results
current. The maintained random-data and feature/method pipeline instead starts
at [`polytope-datasets`](polytope-datasets/README.md),
[`polytope-invariant-table`](polytope-invariant-table/README.md)
and [`sys-datascience`](sys-datascience/README.md).

## What the different searches actually establish

| Question | Preserved answer | Evidence boundary |
| --- | --- | --- |
| How large were the original randomly sampled ratios? | Generic: 70 rows, maximum 0.739. Products: 100 rows, maximum 0.794. Neither panel crossed one. | Historical summary and figures; original raw panels are gone. These are distinct from the enlarged maintained datasets. |
| Can moving ten existing facets raise the ratio? | General ascent: ten seeds, maximum final 0.9030. Product ascent: twelve seeds, maximum final 0.8727. Neither panel crossed one. | Historical summary and figures; summary/trace JSONL removed. Search termination does not certify a local maximum. |
| Can an observed branch-derived direction policy repeatedly improve a body? | All twelve retained F10 runs accepted eight moves; mean gain 0.011565. | Full historical v1 rows remain. Every run hit cap eight and every endpoint scan still found a finite improving move. |
| Can adding a facet improve an F10 endpoint? | Ninety legacy continuation trials; F10-to-F11 gains were common but final values stayed below one. | Historical summary and two figures; raw rows and required general-ascent input removed. |
| Does orientation matter after selecting a strong Euclidean body? | Selected product: 0.862586 to 0.878308. Selected generic: no sampled improvement. | Full compact scan: two post-selected bodies, 164 evaluations each; no population frequency or exhaustive optimum. |
| Does a sparse global linear-distortion sample improve the compact result? | No, in 216 noncompact evaluations per body. | Full stopped pilot; angular coverage and radial scales are sparse. |
| How efficient was random-body rejection sampling? | Eighteen retained cells, 1,000 attempts each; narrow-height F10 acceptance was 72.9%. | Generator/grid/seed-specific calibration, with historical timings and no rejection-reason labels. |

The legacy numerical summaries above are owned by
[`legacy-ascent-continuation-debt.md`](sys-landscape/legacy-ascent-continuation-debt.md).
They are preserved context, not newly reconstructed row-level results. The
retained observed-ascent, orientation and calibration outputs below permit
stronger inspection of their particular finite panels.

## Sampling new bodies and moving facets

The [generic random producer](sys-landscape/random-sample/README.md)
requests ten accepted bodies at each F=5 through 10 and five at each F=11,12,
with master seed 42 and heights in `[0.8,1.2)`. Candidate dual vertices encode
normalized halfspaces `a_i·x <= 1`. Unit normals are sampled on `S^3`, with
independent uniform heights, then divided by those heights. Candidate validity
conditions affect the accepted distribution: this is not a uniform measure on
all convex polytopes. Generic attempts use Blake3-derived individual seeds.
The [product producer](sys-landscape/random-product-sample/README.md)
requests ten products for each of the ten pairs `3 <= k <= m <= 6`, using a
shared ChaCha8 stream and random planar polygons. Its population is restricted
to products in the fixed q/p Lagrangian splitting.

The legacy generic producer uses the orbit-producing auto capacity helper;
the product producer uses the billiard route to retain iteration and bounce
diagnostics. Both use rounded exact reference volume. Their source remains,
but the two PNGs depict old removed raw panels. They must not be relabeled as
plots of the larger maintained random datasets.

[General fixed-F ascent](sys-landscape/gradient-ascent-general/README.md)
moves the normalized dual vertices in the full `R^(4F)` parameter space.
[Product ascent](sys-landscape/gradient-ascent-products/README.md)
restricts directions to preserve product structure, cycling starts among the
3×7, 4×6 and 5×5 splits, all with F=10. The producer sources build active-orbit
first-order models: one branch gives its gradient; multiple selected branches
give a maximin direction. Finite line searches check fractions of a geometric
step bound and overshoots beyond it. Random multiplicative perturbations supply
additional escape attempts. These mechanisms explain what was tested; they do
not imply that all nearby directions or all relevant branches were exhausted.

The old panels retain six PNGs each but no local summary/trace inputs. The
sources remain executable, with per-seed budgets and temporary output defaults.
Changed capacity, geometry, active-branch selection or wall-clock budgets can
change fresh trajectories. Per-seed RNG independence is weaker than byte
reproduction: elapsed times vary, and budget-triggered stopping can alter the
mathematical endpoint itself.

## Observed finite-step ascent has a retained panel

The [observed-general packet](sys-landscape/gradient-ascent-observed-general/README.md)
retains twelve F10 rows for seeds 42–53 in
[`retained-panel.jsonl`](sys-landscape/gradient-ascent-observed-general/artifacts/retained-panel.jsonl),
with a validated [`summary.json`](sys-landscape/gradient-ascent-observed-general/artifacts/summary.json).
The candidate recomputes branch-derived directions after each accepted move.
It checks trace steps `1e-3,1e-4` and, at the final state, all generated
directions against `1e-3,1e-4,1e-5,1e-6`. The accepted improvement threshold
is `max(0,1e-3*abs(base_sys))`.

Every retained run accepted all eight trace moves. Gains range from 0.003579
to 0.042455, with mean 0.011565; measured compute totals 400.889 seconds,
204 finite-step evaluations and 405,772 capacity-orbit iterations. The
endpoint condition is `not_evaluable_trace_did_not_stop` in every row:
the iteration cap stopped each trace before its candidate-specific stop
condition, and every endpoint scan still found an above-threshold move.
This is evidence of systematic finite progress and observed cost. It is
adverse evidence for interpreting cap-eight endpoints as optimized.

These rows belong to the historical v1 `OrbitSearchResult` implementation.
The current v2 producer uses exact action-window KKT witnesses at basepoints
and certified scalar capacity bounds at finite targets. No replacement v2
panel is retained here. The v1 analyzer explicitly rejects other schemas;
its agreement with the historical rows cannot validate the new algorithm.
The packet pins the historical source commit and provides the full producer
reproduction route. An analysis-only check can recover the twelve named
one-row inputs from the retained concatenation and run the analyzer in a
temporary directory; that validates retained rows, not producer reproduction.
For the distinct current optimizer comparisons, start at
[`dev-gradient-ascent`](../../experiments/dev-gradient-ascent/README.md).

## Adding a facet is a different continuation question

[Variable-F ascent](sys-landscape/variable-f-ascent/README.md)
tries random facet placements at selected F10 endpoints, then compares four
routes from random starts: F10 ascent alone, F10 ascent followed by facet
addition and F11 ascent, facet addition before ascent, and fresh F11 ascent.
The fresh F11 control is a separate random start, rather than the identical
body embedded by the other routes. The old comparison comprises fifty
endpoint-placement trials and forty random-start/control trials.

The raw output, cache and required general-ascent panel are removed. Its
source's `local_maxima` terminology denotes selected legacy ascent endpoints,
not certified maxima. The surviving figures and debt note preserve the
bounded observation that continuation often improved endpoints below one.
Full mode cannot reproduce that comparison from the current tree. A future
witness-guided continuation programme remains a design in the debt note, with
exact-check fallback and its own evidence/return contract; it is not a retained
positive result or an active data-science method row.

## Shape and symplectic alignment can be separated

The [fixed-shape packet](sys-landscape/fixed-shape-orientation-search/README.md)
selects the highest stored ratio in each of the 4,096-row generic and
10,240-row product source populations. It holds each Euclidean body fixed and
scans representatives of `SO(4)/U(2)`: non-symplectic rotations can change
capacity while retaining Euclidean volume and shape. A coarse grid followed
by two refinement rounds produced 164 evaluations per body.

[`evaluations.jsonl`](sys-landscape/fixed-shape-orientation-search/evaluations.jsonl)
and [`analysis.json`](sys-landscape/fixed-shape-orientation-search/analysis.json)
record the improvement of `random_5x6_s3_168` from 0.862586 to 0.878308,
above every stored ratio in both source populations. `random_F10_s3_104`
stays at its identity value 0.859560 in the sampled scan. This witnessed
product improvement shows that the selected Euclidean candidate and its
symplectic alignment carry separate information. It does not estimate how
frequently a new body improves or prove the displayed angle optimal.

The companion determinant-one linear scan varies Euclidean shape as well.
Its normalized skew form `M^T J M = cosh(r)U + sinh(r)V` uses the two unit
spheres in the self-dual/anti-self-dual splitting; `r=0` recovers the compact
orientation stratum and the representative map's condition number is `exp(r)`.
Each body receives a compact-best control and 216 noncompact evaluations:
24 global and twelve compact-anchored angular directions at each of six
scales from `r=0.125` to 4. The retained
[`global-evaluations.jsonl`](sys-landscape/fixed-shape-orientation-search/global-evaluations.jsonl)
and [`global-analysis.json`](sys-landscape/fixed-shape-orientation-search/global-analysis.json)
show no sampled improvement. The best noncompact values are 0.804071
(generic) and 0.836228 (product), both at the smallest radius. This sparse
negative pilot stopped the naive random-transform route. A targeted geometric
mechanism, optimizer or materially different body would be grounds to revisit
it; the result does not place a global maximum on the compact stratum.

Both retained scans are historical legacy-capacity evidence. Fresh compact
runs default to production capacity and v2 rows, while the stopped global
producer deliberately retains its legacy default. Both use f64 volume with
exact-derived incidence. The summaries compare numerical estimates; they do
not convert production capacity intervals into certified optimality or
ordering statements. Source champions are selected from current dataset
contents on rerun, so preserving the same filenames is insufficient to pin
the original population. Historical summary path strings retain the former
`sys-datascience/produce/` location; current producer-owned files are under
`polytope-datasets/`.

## Generator calibration and safe verification

[Rejection calibration](sys-landscape/rejection-calibration/README.md)
retains [`acceptance.jsonl`](sys-landscape/rejection-calibration/acceptance.jsonl)
for F=5–10 and three height intervals. It times candidate generation and
geometry acceptance, including primal construction for accepted bodies;
it does not time capacity. Each grid cell starts again from seed 42, rather
than constituting an independent seeded repetition. The narrow-height panel's
acceptance rises from 5.7% at F5 to 72.9% at F10; the widest-height panel peaks
at 37.1% at F8 and falls to 28.3% at F10. The artifact does not identify which
geometric test rejected each body, and its machine-specific timing rows do
not pin a source revision.

Verification must follow retention boundaries. The rejection binary always
rewrites the tracked artifact and has no smoke mode. Both orientation
producers also default to tracked output paths. Use their explicit temporary
output options or reanalyze existing JSONL into `/tmp`; do not run them simply
to check command syntax. The legacy sampling binaries default to temporary
paths but still request their entire 70/100-body plans; their READMEs provide
one-body checks. The ascent smoke scripts use temporary outputs but invoke
real optimization with per-seed budgets, rather than an instantaneous CLI
check. Variable-F smoke falls back to a new random start when the removed
ascent input is unavailable and therefore cannot verify the old continuation
comparison.
