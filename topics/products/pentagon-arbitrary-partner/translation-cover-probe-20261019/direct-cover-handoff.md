# Bounded unrestricted translation-cover rehearsal

19 October 2026. Owner `/root/partner_geometry`, receiving owner `/root`.
All probe work stopped; no jobs or follow-up optimization remain owned here.
Everything in this packet is scratch. Root chooses retention, review and
integration; no domain-source or manuscript files were changed for this probe.

## Actual question and source inspection

The actual finite unrestricted model is
`area(conv(union_j(T_j+t_j)))`, with all ten fixed oriented unit normal
triangles, overlaps allowed, `t_0=0`, and eighteen free relative translation
coordinates. A numerical area improvement below the exact HKO area would be a
candidate for exact reconstruction, not an accepted counterexample.

My bounded search and reading covered the retained exact local-rigidity
certificate/fixture and the recovered restricted-ten-direction source. The
algorithm owner additionally reported a historical codimension-one contact
slide. None of these identified sources was an unrestricted 18-variable
local optimizer under this contract. This was a bounded source search, not an
exhaustive archive inventory. The already solved fan model must not be mistaken
for an unrestricted search. The global compact translation-domain argument in
`RESIDUAL_AND_OBSTRUCTIONS.md` was inspected but is not used as a numerical
branch-and-bound certificate here.

The algorithm owner supplied Jung–Ahn, *Smallest Convex Hulls of Polygons*,
ESA 2026, Section 5.1 Lemma 6: fixing a common slide direction makes area convex
piecewise linear in all translation scalars. Its primary-source PDF is
https://drops.dagstuhl.de/storage/00lipics/lipics-vol388-esa2026/LIPIcs.ESA.2026.44/LIPIcs.ESA.2026.44.pdf .
This packet tests that primitive, not the paper's full global algorithm.

## Implemented oracle and bounded search

`direct-cover-probe.py` imports the exact retained
`local-cover/pentagon_fixture.py`, constructs all ten templates and known HKO
placements, converts them to floats, anchors the first translation, and evaluates
the 30-point hull with SciPy ConvexHull. The input JSON retains every template,
initial placement, seed, fixture hash and settings; the baseline area is
`0.20081141588622722` in this fixture's physical coordinates.

For each fixed unit direction d, the scalar variables are the nine shifts of
triangles 1–9. At a CCW hull with labelled vertices p_i, the supporting-plane
coefficient for label j is

```
g_j = (1/2) sum_{i:label(i)=j} det(p_(i-1)-p_(i+1),d).
```

The shoelace area of this fixed hull cycle is affine in parallel shifts and
is a global lower supporting plane: in coordinates with d horizontal, its two
chains remain y-monotone, and their signed slice width never exceeds the new
hull's true width. This also explains the contact/degeneracy case without
assuming the full eighteen-variable objective is convex. Floating arithmetic
and LP solves remain uncertified.

A cutting-plane epigraph LP minimizes the accumulated planes inside an
explicit numerical shift box `[-2,2]^9`, retaining every best evaluated
placement. Alternating directions is a heuristic for the full nonconvex
problem. The box is a search restriction, not a physical contact or a proved
bound on every optimal scalar slide. No selected incumbent hit this box;
primary-query box contacts were not individually logged. The refinement trace
retains every queried scalar position.

Twelve deterministic starts were used: HKO, reflected HKO, common-centered
triangles, three HKO perturbation scales, and six remote seeded placements.
Each ran twelve direction updates, capped at eighteen oracle evaluations per
update, for 217 evaluations/start. All 144 updates hit that cut cap; none
closed its inner LP gap. Seed: 20261019. Objective-evaluation counts include
initial incumbents. All caps and unfinished inner gaps are retained.

## Results and diagnostic refinement

No evaluated hull had area below the HKO baseline. Both HKO controls retained
area ratio `1.0000000000000002`. The best non-control after the short schedule
was a perturbed-HKO start at `1.0845596447402013` times HKO. One remote start
fell from `5.767376196858145` to `1.1875206831058924` times HKO. These decreases
show candidate generation works; they are not convergence evidence.

One fixed-direction refinement was then performed on three existing starts;
no new start or random campaign was added:

| Existing start | Additional evaluations | Cumulative start evaluations | Best area / HKO | Numerical gap / HKO inside shift box |
| --- | ---: | ---: | ---: | ---: |
| HKO | 23 | 240 | 1.0000000000000002 | −2.8e−16 |
| Common center | 80 | 297 | 1.2230509022726799 | 1.65e−14 |
| Existing remote | 71 | 288 | 1.1482892279878345 | 1.11e−14 |

The third refinement had already completed when root's subsequent steering
specified exactly the first two controls. This is reported explicitly rather
than omitted; every start remained within the original 300-evaluation cap.
No further probe followed that steering.

These results discriminate the main implementation issue: eighteen cuts were
too few to solve even these cheap convex inner problems, whereas 23–80 cuts
closed the numerical box gaps in the refinement. Closing one inner LP gap is
neither local nor global convergence for the unrestricted 18-variable model.
The common-center and remote results remain well above HKO. There is no
mathematical minimum-area conclusion from this rehearsal.

No accumulated supporting cut exceeded any subsequently queried objective
value in these runs (maximum observed violation zero). This is a finite
implementation check, not a proof of numerical reliability or LP lower bounds.

## Evidence, resources and stops

- `direct-cover-results-inputs.json`: full exact-fixture-derived floating
  triangles, all initial translations, seed, source hash and settings.
- `direct-cover-results.json`: per-start incumbents, full best translations,
  hull labels, all inner gaps and cut-cap terminations.
- `direct-cover-refine-results.json`: full query traces and final translations
  for the three existing-start refinements.
- `direct-cover-manifest.json`: SHA-256 and sizes for the code/input/output
  packet and owned process identities.

Primary process PID 322258 exited 0: 7.36 seconds wall and 8.46 seconds observed
process CPU. Refinement PID 322870 exited 0: 0.40 seconds wall and 1.60 seconds
observed process CPU. CPU hard limits and external wall timeouts were set;
neither fired. All 12 starts completed, maximum cumulative per-start count 297.
No failure, budget overshoot, apparent HKO improvement or running subprocess
remains. No unrelated process was stopped.

A reusable tool would need a separate code review, cleaner relative-path
interface and complete primary query logging before retention. Increasing
starts or direction sweeps is not selected by this handoff. Any later larger
campaign needs a concrete scientific discrimination target and its own scope.
