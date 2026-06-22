# Branch Cartography

Status: early reference surface for studying `sys(a)` and HK branch behavior
across perturbation scales. This is not the final datascience architecture and
not thesis evidence by itself.

## Scope

This subpacket asks how much information at a base point `a0` remains
predictive for nearby, semi-local, or effectively unrelated points `a`.

This is a prototype and reference surface for optimizer-facing branch
cartography. Source-stratified local/semi-local evidence belongs in
`experiments/sys-datascience/methods/local-behavior-prediction/`.

The point shape is:

```text
(a0, data(a0), a, data(a), relation_to_a0)
```

The relevant data currently includes `sys(a)`, best and near-active sigma
sets, branch candidate-window membership, transition changes, finite
directional predictions, observed finite deltas, and orbit-search cost.
Relative `sys` change is not stored as a separate field; compute it from
`base_sys` and `target_sys` in `branch-cartography-samples.jsonl`.

Gradient ascent is a downstream consumer: a fixed optimizer needs this
knowledge because steps may cross branch-domain boundaries, ridges, cusps, or
regions where the local branch model stops predicting the target point. Do not
interpret this subpacket as only endpoint diagnostics or method promotion.

## Perturbation Scales

The code does not yet attach a formal `scale_regime` field. Interpret runs by
their explicit `--steps`, direction design, and layer policy.

Current working taxonomy:

- **Local:** small finite steps where branch values and active sets should often
  remain predictable from `a0`.
- **Semi-local:** steps large enough that branch switches, opened transitions,
  or candidate-window misses may occur but still connected to the same base
  point or short trace.
- **Effectively global:** points far enough apart that branch values or minimum
  sigma sets are treated as weakly related unless data shows otherwise.

The initial checked runs only test small finite steps (`1e-4`, sometimes
`1e-3`). They do not locate the local-to-global transition.

## Current Binary

Run after a branch degeneracy diagnostic:

```bash
cargo run -p exp-dev-gradient-ascent \
  --bin dev-gradient-ascent-branch-diagnostic -- \
  --out-dir /tmp/dev-gradient-ascent-branch-diagnostic-check \
  --max-rows 8

cargo run -p exp-dev-gradient-ascent \
  --bin dev-gradient-ascent-branch-cartography -- \
  --diagnostic-dir /tmp/dev-gradient-ascent-branch-diagnostic-check \
  --out-dir /tmp/dev-gradient-ascent-branch-cartography-check \
  --steps 1e-4 \
  --max-fixtures-per-label 1 \
  --random-directions 1
```

Most worktrees are created with LFS smudge skipped. If
`experiments/sys-datascience/tables/*.jsonl` is an LFS pointer file in the
current worktree, either check out those table files with LFS for this worktree
or pass `--polytope-table` and `--provenance-table` to the diagnostic command
using a worktree that has the real table data. Pass the same real
`--polytope-table` path to the cartography command.

Defaults:

- `--selection-threshold-relative 1e-3`
- `--action-window-relative 1e-2`
- `--steps 1e-4,1e-3`
- `--layers 1`
- `--random-directions 2`
- `--max-fixtures-per-label 1`
- `--degeneracy-labels large_gap,narrow_gap,high_degeneracy`

Outputs:

- `fixture-selection.jsonl`
- `branch-cartography-points.jsonl`
- `branch-cartography-samples.jsonl`
- `compute-budget-report.json`
- `summary.json`

The summary and budget report include the run parameters and input file
metadata. `fixture-selection.jsonl` copies provenance fields from the
diagnostic fixture selection when available.

## Caveats

Classification uses raw sign:

```text
observed_delta_sys > 0.0
```

There is no positive-delta tolerance yet. Treat `improving_*` as raw-sign
finite samples until a threshold is added and checked.

Layer expansion is ascent-biased: non-improving targets are recorded but only
raw-sign improving targets are expanded to the next layer. Direction design is
also not neutral; it uses local branch-gradient directions, maximin directions,
and deterministic random unit directions. This is useful for quickly finding
branch-model failures and optimizer-relevant behavior, but it is not an
unbiased map of perturbation space.

The row store is not a resumable `(a, data(a))` cache. A datascience-oriented
successor should probably split the producer into point rows, pair/query rows,
radius-summary rows, and a cache keyed by a stable point identifier.

## Useful Next Questions

- At which step sizes do target best sigmas stop belonging to the base
  near-active set?
- At which step sizes do candidate-window misses or transition-opened samples
  appear?
- How do those answers vary between large-gap, narrow-gap, and high-degeneracy
  base points?
- How quickly does single-branch finite prediction degrade compared with
  near-active multi-branch prediction?
- Which point sources give the highest value base points: random retained
  polytopes, intermediate ascent states, post-stop endpoints, or symmetry-made
  high-degeneracy points?

These questions are useful because they determine what a fixed ascent method
has to handle and which claims can later be supported by datascience reruns.
