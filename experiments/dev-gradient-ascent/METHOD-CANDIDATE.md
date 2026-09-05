# Candidate Observed Multi-Direction Ascent

Status: named development candidate, not promoted library code and not thesis
evidence by itself. This describes the algorithm, not an assignment to develop
or promote it. [Current work and ownership](../../memories/todos.md) records
the live scope. For perturbation-scale behavior, see
[branch-cartography/](branch-cartography/README.md).

## Algorithm

The candidate is `iterative_observed_multi_direction_probe`, implemented by the
[local geometry probe](local-geometry-probe/README.md).

At each trace state:

1. Recompute the active `sys` state and collect near-active HK sigma branches.
2. Build local directions from the current branch model:
   - `single_near_active_gradient`;
   - `negative_single_near_active_gradient`;
   - `near_active_maximin_direction` when more than one near-active branch is
     present.
3. Sort generated directions by predicted directional derivative, descending.
4. Try each generated direction and configured trace step.
5. Accept the first finite step whose recomputed `sys` improvement exceeds
   `max(min_observed_delta, min_observed_relative_delta * abs(base_sys))`.
6. Stop when all generated direction/step pairs fail that test, or when the
   trace iteration cap is reached.

Negative-predicted directions are still tested: the local prediction does not
exclude a finite improvement after recomputation. Later generated directions
are tried if earlier ones fail the observed-improvement test.

## Development Parameters

The documented candidate configuration is:

- trace steps: `1e-3,1e-4`;
- endpoint scan steps: `1e-3,1e-4,1e-5,1e-6`;
- relative improvement threshold: `min_observed_relative_delta = 1e-3`;
- absolute improvement threshold: `min_observed_delta = 0`;
- branch selection threshold: `1e-3` for large-gap and narrow-gap selections,
  and `0.01` for high-degeneracy selections;
- trace iteration cap: `8`.

These are development parameters, not approved thesis constants. Comparisons
across regimes must state the differing branch selection thresholds.

## Finite Endpoint Condition and Limits

The endpoint condition is that every generated post-stop direction and step in
the endpoint grid has recomputed `sys` improvement at most

```text
max(min_observed_delta, min_observed_relative_delta * abs(endpoint_sys)).
```

For the configuration above, this is `1e-3 * abs(endpoint_sys)`. It must be
evaluated from the endpoint direction scan; stopping the trace alone does not
establish it. The trace and endpoint grids can differ, and reaching the
iteration cap does not imply that available improvements were exhausted.

This finite check permits positive improvements below the threshold. It does
not certify local maximality: smaller steps, ungenerated directions, omitted
branches, or nearby branch-domain effects can remain unchecked. Branch/germ
completeness is heuristic. Results from a selected fixture panel do not establish
behavior from all starts.

The [probe README](local-geometry-probe/README.md) documents producer commands,
split-run selection, candidate-window variants, ranking audits, reporting tools,
and run provenance. Reports summarize their input observations; they do not
recompute `sys(a)` or supply missing observations.

## Recovery and Support

Earlier versions of this note and the removed `PROMOTION-READINESS.md` are
recoverable in Git. Commit `f3c924d4` contains historical source and input tables
and command fragments; `6a5ee97d` removed unretained numerical claims from the
promotion packet. At `f3c924d4`, the inputs are
`experiments/sys-landscape/datascience/tables/polytope-table.jsonl` and
`polytope-provenance-table.jsonl` in that same directory; command fragments are
in `experiments/dev-gradient-ascent/README.md`. Use `git show <commit>:<path>`
to inspect them without restoring a checkout.

No complete execution, input-selection, and raw-output chain
for the old numerical claims was recovered in the cleanup review. This is a
support gap, not proof that the chain is irrecoverable elsewhere.

Absence from the current checkout is only retrieval overhead when exact source,
input, and command recovery exists. The earlier fixture counts, success claims,
and timings are omitted here because their complete support was not recovered.
This cleanup did not execute or test the historical code.
