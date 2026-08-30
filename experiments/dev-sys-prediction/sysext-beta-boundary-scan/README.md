# Sysext Beta-Boundary Scan

Status: standalone development diagnostic with no retained output in this
directory.

## Question

Across selected rows of a polytope table, how many raw KKT branches solve
numerically, and how close are their minimum beta coordinates to the
admissibility boundary `beta = 0`?

The scan enumerates transition-pruned sigmas, solves the raw KKT system without
requiring positive beta, and reports counts at absolute beta-margin thresholds
from `1e-6` through `1e-2`. This locates rows where beta-domain behavior may
deserve a fixed-branch probe. It does not test whether a near-boundary raw
branch becomes capacity-minimizing after a perturbation.

## Input, output, and command

The default input is the table produced at
`experiments/polytope-invariant-table/polytope-table.jsonl`.

```bash
scripts/artifacts.py materialize polytope-invariant-table

cargo run -p exp-dev-sys-prediction --release \
  --bin dev-sysext-beta-boundary-scan -- \
  --polytope-table experiments/polytope-invariant-table/polytope-table.jsonl \
  --out /tmp/dev-sysext-beta-boundary-scan.jsonl \
  --max-rows 1
```

The output is one JSONL row per selected input row. The command does not write
a summary, provenance record, or retained interpretation.

## Corresponding implementation

`../src/sysext_beta_boundary_scan.rs` is a copy-edited version called by the
config-driven [`../produce/`](../produce/README.md) panel. It is not imported
by this standalone executable. A mathematical or numerical change to either
raw KKT scan therefore requires inspecting the other implementation; textual
similarity is the current maintenance link.

The production panel currently requests zero beta-boundary rows, while its
smoke config requests one. Neither config makes this standalone directory a
retained evidence packet.

## Claim boundary

A successful row establishes only that the implemented f64 raw KKT solve
returned finite margins under its eigenvalue, residual, and positive-`q`
thresholds. It does not establish branch admissibility, branch relevance,
prediction quality, or a theorem about beta-domain boundaries.
