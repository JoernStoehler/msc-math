# Coverage-directed novelty search

This packet evaluates a target-free discovery policy, not generator quality.
It uses the existing cheap factor-only generator path on the fixed six-sided
stratum, with balanced rows for the eight explicit planar populations and two
independent master seeds.  The first seed is the training candidate pool; the
second is an independent holdout pool.  Every retained row keeps its producer
`sample_id`, law/parameter, seed, row, attempt, and deterministic witness ID.

`analyze.py` generates the complete balanced train and holdout pools first,
then runs a deterministic passive hash-ranked retained-witness coreset and
three offline greedy coresets: `offline_greedy_max` (max-normalized gain across
two views), and view-specific `offline_greedy_frame`/`offline_greedy_chord`.
These are offline coresets, not online adaptive generator allocation. The frame view quotients cyclic
starting point, reversal, translation, positive scale, and a local frame
rotation.  The lossy chord view sorts all pairwise chord lengths, removing
vertex order.  The views remain separate in `coreset-yield.tsv`; disagreement
rows are retained in `view-disagreement.tsv` rather than collapsed into a law
score.  Holdout nearest-cover max, mean, and q90 are measured at matched
retained-witness budgets. Full-pool producer generation counts/costs are copied
from the producer reports into `generation-cost.tsv`; offline selection cost is
measured separately. No per-generated-row or online sample-efficiency claim is
made.

The frozen reference set is the lowest four deterministic witness IDs from the
current-baseline population. Train/holdout master seeds and sample IDs must be
disjoint, and all population/side-count strata must remain balanced; the
analyzer fails closed otherwise. A selected witness is authorized only for later
target-free geometry follow-up. No `sys`, target, density, support, law-quality,
or post-selection inferential claim is permitted. Geometry alone cannot
distinguish a tiny remote population mode from a contaminated outlier; the
synthetic calibration retains that limitation explicitly.

## Reproduce

From this worktree, build the existing producer and run two independent pools:

```bash
cargo build --release --locked --package exp-sys-landscape --bin sys-datascience-generator-zoo-smoke
PACKET=experiments/sys-datascience/methods/generator-coverage-directed-novelty
PRODUCER=target/release/sys-datascience-generator-zoo-smoke
COMMON=(--factor-only --attempts 256 --factor-rows-per-population 12 --factor-side-counts 6
  --factor-population 'current-baseline|delta=0.2'
  --factor-population 'primal-hull-uniform-disk|points=n+4,origin=interior'
  --factor-population 'repulsive-gap|alpha=1'
  --factor-population 'repulsive-gap|alpha=4'
  --factor-population 'repulsive-gap|alpha=16'
  --factor-population 'repulsive-gap|regular'
  --factor-population 'regular-mutation|steps=4,scale=0.03'
  --factor-population 'zonogon|lengths=uniform(0.5,1.5)')
"$PRODUCER" "${COMMON[@]}" --seed 20260716 --factor-out-dir "$PACKET/artifacts/train"
"$PRODUCER" "${COMMON[@]}" --seed 20260717 --factor-out-dir "$PACKET/artifacts/holdout"
python3 "$PACKET/analyze.py" --train "$PACKET/artifacts/train/factor-shapes.jsonl" \
  --holdout "$PACKET/artifacts/holdout/factor-shapes.jsonl" \
  --producer-report "$PACKET/artifacts/train/factor-only-report.json" \
  --producer-report "$PACKET/artifacts/holdout/factor-only-report.json" \
  --out-dir "$PACKET/artifacts/analysis"
uv run --with pytest --with numpy python -m pytest -q "$PACKET/test_packet.py"
```

The checked-in artifacts are a bounded finite-panel result.  Rebuild the
producer from source rather than relying on a disposable absolute binary path;
the analysis report binds exact input/report/analyzer hashes and the repository
revision. A future packet should first check whether the bulk (mean/q90)
holdout cover reductions survive another pair of seeds and side-count strata;
do not select a target from this packet.
