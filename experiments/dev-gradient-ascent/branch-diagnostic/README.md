# Branch Degeneracy Diagnostic

Status: development-data producer for the gradient-ascent and branch-behavior
packets. Its outputs are diagnostics, not local-maximality evidence.

## Question

Which retained `sys` input rows can be recomputed successfully, and how does
their near-active HK branch count change across relative action thresholds?

The diagnostic labels successful row-threshold pairs as `large_gap`,
`narrow_gap`, or `high_degeneracy`. Downstream packets use those labels to
select fixtures; the labels do not by themselves establish geometric
degeneracy or endpoint behavior.

## Inputs and outputs

By default the producer reads the polytope and provenance tables in
`experiments/sys-datascience/prepare/`. Most worktrees skip LFS checkout. If
those JSONL files are LFS pointers, check out the two inputs or pass paths to
real tables explicitly.

The output directory contains:

- `fixture-selection.jsonl`: selected source rows and copied provenance;
- `branch-set-diagnostic.jsonl`: one recomputation result per row and
  threshold;
- `compute-budget-report.json`: orbit-search and wall-time accounting;
- `summary.json`: counts, parameters, paths, and the diagnostic caveat.

## Command

```bash
cargo run -p exp-dev-gradient-ascent \
  --bin dev-gradient-ascent-branch-diagnostic -- \
  --out-dir /tmp/dev-gradient-ascent-branch-diagnostic-check \
  --max-rows 8
```

Use `--polytope-table` and `--provenance-table` together when overriding the
default inputs. Consumers normally receive the output directory through
`--diagnostic-dir`.

## Consumers and claim boundary

Current direct consumers are:

- [`../local-geometry-probe/`](../local-geometry-probe/);
- [`../branch-cartography/`](../branch-cartography/README.md).

The producer retains no canonical run in this directory. A successful run
shows only what the implemented finite branch diagnostic recomputed on its
selected input rows.
