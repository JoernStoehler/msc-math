# Empirical research desk: first target-free scout

**Historical checkpoint, superseded as a work recommendation.** See the
[current research agenda](research-agenda.md) for the subsequent audit and
active pentagon-partner line. The statistics below remain exploratory evidence;
the proposed outlier inspection is no longer the recommended next task.

Date: 2026-09-18. This desk owns scoped empirical discussion and experiments,
not thesis prose. The production budget remains paused.

## Decision and result

The first useful move was a zero-capacity, target-free relation scout on the
already available join between the historical invariant table and canonical
extreme-vertex covariance measurements. It examined 14,334 rows in all 18
generator buckets and deliberately never loaded `sys` into the analysis.

The clearest candidate relation is between covariance symplectic eccentricity
`rho = nu2 / nu1` and the normalized sum of unsigned symplectic two-face areas.
Their within-bucket Spearman coefficient is positive in every bucket: median
0.961, generic-bucket median 0.939, product-bucket median 0.975, and range
0.844--0.997. The nearly identical mean-area result is not an independent
discovery; within exact buckets it largely differs from the sum by the fixed or
nearly fixed number of two-faces. No covariance--combinatorial pair met the
prewritten follow-up rule (at least 0.9 sign consistency, agreeing generic and
product median signs, and median absolute coefficient at least 0.5).

This is an exploratory geometric relation, not yet a conjecture or a capacity
result. It is stronger than a pooled source-composition effect because it holds
within every generator bucket, including theorem-excluded product families.
It may nevertheless be a common response to anisotropy rather than a direct
mathematical constraint. Multiple high-ranked ridge statistics are dependent
summaries of the same two-face-area field and must not be counted as separate
discoveries.

## Concrete next discrimination

The screen also selected, without using `sys`, one body per bucket where the
within-bucket ranks of `rho` and total ridge area disagree most. The strongest
generic discrepancies are `8264c9ce...` in `random:F11` (rank fractions 0.598
versus 0.021) and `a665a799...` in `random:F12` (0.712 versus 0.157). The
strongest product discrepancy is `9ce9bde6...` in `6x6` (0.532 versus 0.064).
These examples show that the association is not a deterministic rank identity.

The recommended next action is a bounded semantic reconstruction on those
three bodies plus a typical matched control from each bucket:

1. recover their exact vertices and two-face decompositions;
2. decompose covariance and ridge-area contributions by direction/factor;
3. test whether one explicit anisotropy quantity predicts why the leading
   relation fails on the selected examples;
4. stop if no common explanation survives both generic and product cases.

This step needs no new capacity evaluation. The exact source-geometry artifact
is not currently materialized in the host cache, so geometry recovery is the
only dependency. If the dependency is cheap, I expect this inspection to take
well under an hour and to have roughly even odds of yielding either a concise
mechanistic explanation or a useful counterexample to an overly simple one.
If it only restates that both summaries encode anisotropy, the lane should stop
and acquisition should move to a substantively different representation.

A separate broad lane should remain alive: the present pilot spans only
combinatorics, symplectic two-face summaries, and covariance geometry. It does
not fulfill the larger request for broad representation discovery by itself.
The next representation should be chosen for genuinely different information
(for example a raw/covariant directional field under matched transformations),
not another scalar transform of the same ridge areas.

## Reproduction and boundaries

Run:

```sh
uv run --with scipy python \
  docs/empirical-viterbo-design/desk/target_free_relation_scout.py \
  --historical /home/joern/.cache/msc-math/artifacts/polytope-invariant-table/c3042846723dbb89db17f2f39d88004d937e1e790949ff6880188fa89fc1900a/files/polytope-table.jsonl \
  --covariance experiments/sys-datascience/methods/canonical-vertex-covariance/artifacts/current/per_polytope.jsonl \
  --out-dir docs/empirical-viterbo-design/desk/target-free-relation-scout
```

The generated [summary](target-free-relation-scout/summary.json) records input
hashes, all bucket coefficients, transformation boundaries, target exclusion,
and discordant exemplars. [All screened pairs](target-free-relation-scout/relations.tsv)
remain available rather than only the leading result.

`rho` is translation- and scale-invariant and is unchanged by exact linear
symplectic transformations of the vertex representative. `nu1` and `nu2` have
degree two under common scaling. `covariance_condition` is Euclidean and
representative-dependent. The historical comparison columns follow their
owning combinatorial and normalized symplectic two-face feature contracts.
None of these empirical transformation statements extends a polytope feature
to invariance under arbitrary nonlinear symplectomorphisms.
