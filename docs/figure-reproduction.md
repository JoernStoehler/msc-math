# Reproduce the selected thesis figures

The selected thesis uses thirteen external graphics. The table below maps
every one to its producer, input and deliberate publication-copy name. Run
from the repository root. These commands regenerate figures from the named
retained data; they do not rerun every historical capacity evaluation.

Use a fresh directory so that a failed or different-version run cannot replace
retained evidence or the selected thesis assets:

```bash
figure_run=$(mktemp -d /tmp/msc-thesis-figures.XXXXXX)
```

The [4 October continuation packet](history/thesis-continuation-2026-10-04/)
records actual commands, input/output hashes, tool versions and comparisons.
Its separate PDF check stages regenerated graphics into a fresh source copy.
The original reviewed PDF and its first source-only build remain preserved in
[the preceding packet](history/thesis-restart-2026-10-04/).

## Complete figure map

Paths in the selected-file column are relative to `thesis/figures/`.

| Selected file | Thesis location | Producer and input |
| --- | --- | --- |
| `foundations/characteristic-normalization.pdf` | §2, Hamiltonian language | `thesis/figures/foundations/generate.py`; explanatory construction in source |
| `foundations/facet-polarity.pdf` | §2, polytope input | Same producer; planar geometric construction in source |
| `flow-graph/flow-graph-f6-tube-sequence.pdf` | §5 | `experiments/dev-flow-graph/visualize-tube/{main.rs,render.py}`; exact F6 fixture, attempt3, word1,2,4,5,3 |
| `upper-bound-mechanisms.pdf` | §7 | `thesis/figures/make_hko_figures.py`; explanatory toy functions |
| `symmetry-extension.pdf` | §7 | Same producer; explanatory toy function |
| `conditional-ridge-correlation.pdf` | §8 | `methods/selection-mechanism/analyze.py`; registered invariant table and tracked provenance |
| `association-generic.pdf` | §8 | `thesis/figures/plot_association.py`; same table and provenance |
| `association-products.pdf` | §8 | Same producer and inputs |
| `rotation-profile.png` | §9 | `experiments/regular-products/rotated-regular-products/analyze.py`; tracked `lagrangian-products-5x5.jsonl` |
| `visualization/viz-hypercube-ridges.png` | §12 | `experiments/visualization/`; Rust hypercube export and pinned browser screenshot |
| `visualization/viz-hko-pentagon-min-orbit.png` | §12 | Same route with `hko_pentagon` export |
| `hko-recovery-by-source-distance.pdf` | Appendix A | `experiments/dev-gradient-ascent/ascent-continuation/analyze_hko_calibration.py`; tracked HKO one-step panel |
| `derivative-and-kkt-scale.pdf` | Appendix A | `experiments/dev-gradient-ascent/endpoint-model-audit/analyze.py`; tracked directional audit |

Here `methods/` abbreviates `experiments/sys-datascience/methods/` only in
the table. The commands below use full paths.

## Empirical plots from retained inputs

The correlation and association plots use the historical 14,336-row table,
not the reconstructed 45-feature P2 modeling table. First materialize the
registered snapshot if it is absent (authenticated R2 access is required),
or verify the existing cache offline:

```bash
python3 scripts/artifacts.py verify-cache polytope-invariant-table > "$figure_run/table-cache.json"
figure_table=$(python3 -c 'import json,sys; print(json.load(open(sys.argv[1]))["directory"] + "/files/polytope-table.jsonl")' "$figure_run/table-cache.json")
```

For an absent cache use
`python3 scripts/artifacts.py materialize polytope-invariant-table --no-link`
first. `verify-cache` checks the registry's snapshot identity, manifest,
complete inventory, sizes and hashes without networking or writes; it does
not assert remote availability. See [artifact contracts](artifacts.md).
The two plotters also reject inputs whose expected historical table or
provenance hashes differ. Keep Python optimization disabled.

```bash
PYTHONOPTIMIZE=0 uv run --script thesis/figures/plot_association.py \
  --table "$figure_table" \
  --provenance experiments/polytope-invariant-table/polytope-provenance-table.jsonl \
  --out "$figure_run/association"
PYTHONOPTIMIZE=0 uv run --script experiments/sys-datascience/methods/selection-mechanism/analyze.py \
  --table "$figure_table" \
  --provenance experiments/polytope-invariant-table/polytope-provenance-table.jsonl \
  --out "$figure_run/conditional"
uv run --script experiments/dev-gradient-ascent/ascent-continuation/analyze_hko_calibration.py \
  experiments/dev-gradient-ascent/ascent-continuation/artifacts/hko-one-step-development-panel-20260729/raw \
  "$figure_run/hko-calibration"
uv run --script experiments/dev-gradient-ascent/endpoint-model-audit/analyze.py \
  experiments/dev-gradient-ascent/endpoint-model-audit/artifacts/directional-decomposition-20260729/raw/audit.json \
  "$figure_run/derivative"
uv run --script experiments/regular-products/rotated-regular-products/analyze.py \
  --only-pentagon --out-dir "$figure_run/rotation"
```

The first two commands recompute descriptive summaries from the retained
ratios. The calibration and derivative commands analyze recorded optimizer
and finite-difference traces. The rotation plot combines recorded samples
with the analytic formula. None independently certifies the historical
floating-point volumes, statistical generalization or complete Reeb dynamics.

## Structural figures and visualization

The explanatory producers write beside themselves, so copy their unchanged
source to the fresh output directory. These diagrams illustrate definitions
and proof mechanisms; they are not additional theorem evidence.

```bash
mkdir -p "$figure_run/foundations" "$figure_run/hko" "$figure_run/flow"
cp thesis/figures/foundations/generate.py "$figure_run/foundations/"
cp thesis/figures/make_hko_figures.py "$figure_run/hko/"
uv run --script "$figure_run/foundations/generate.py"
uv run --with matplotlib==3.11.2 --with numpy==2.5.3 "$figure_run/hko/make_hko_figures.py"
cargo run -p exp-dev-flow-graph --release \
  --bin flow-graph-visualize-tube-data -- \
  --facet-count 6 --master-seed 20260605 --attempt 3 --sigma 1,2,4,5,3 \
  --output "$figure_run/flow/flow-graph-f6-tube.json"
uv run --script experiments/dev-flow-graph/visualize-tube/render.py \
  --layout sequence --input "$figure_run/flow/flow-graph-f6-tube.json" \
  --output "$figure_run/flow/flow-graph-f6-tube-sequence.pdf"
```

The explicit HKO plotting versions above are those observed in the dated
check, not a claim that other Matplotlib versions have identical PDF bytes.
The exact flow producer freshly reconstructs the selected fixture and orbit;
its retained JSON and fresh JSON agreed byte for byte in that check.

For the two selected browser images, stage the viewer without copying an
existing `node_modules`, refresh its two selected JSON inputs, and embed them:

```bash
mkdir -p "$figure_run/visualization/viewer/data"
cp experiments/visualization/viewer/{index.html,viz.js,projection.js,screenshot-figures.mjs,embed-data.sh} \
  "$figure_run/visualization/viewer/"
cp experiments/visualization/viewer/data/*.json "$figure_run/visualization/viewer/data/"
for figure_name in hypercube hko_pentagon; do
  cargo run --release --manifest-path experiments/visualization/Cargo.toml \
    --bin visualization -- "$figure_name" \
    "$figure_run/visualization/viewer/data/$figure_name.json"
done
bash "$figure_run/visualization/viewer/embed-data.sh" > "$figure_run/visualization/viewer/data.js"
(cd "$figure_run/visualization/viewer" && \
  npm install --no-save --no-package-lock playwright@1.61.1 three@0.128.0 && \
  npx playwright@1.61.1 install chromium-headless-shell)
```

The other six viewer datasets are retained inputs, unused by the selected
figures. In the same shell, use an unused local port and stop only the server
started by this block:

```bash
(
  python3 -m http.server 18764 --bind 127.0.0.1 \
    --directory "$figure_run/visualization/viewer" > "$figure_run/viewer-server.log" 2>&1 &
  figure_server_pid=$!
  trap 'kill "$figure_server_pid" 2>/dev/null || true; wait "$figure_server_pid" 2>/dev/null || true' EXIT
  sleep 1
  kill -0 "$figure_server_pid" || exit 1
  VIZ_BASE_URL=http://127.0.0.1:18764 node "$figure_run/visualization/viewer/screenshot-figures.mjs"
)
```

Browser setup may require the
system dependencies described in the [visualization runbook](../experiments/visualization/README.md).
The producer uses SwiftShader explicitly and rejects lost WebGL contexts or
an all-white rendered canvas. This was added after a real run exited zero
with a blank hypercube screenshot. Both corrected screenshots matched the
retained PNG bytes; an injected lost context failed before saving any PNG.

## Copy contract and isolated PDF check

Publication copies are deliberate: the ordinary thesis build does not run
plotters or download data. To inspect regenerated plots in context, copy the
tracked thesis into a new directory and replace only its figure inputs.
Never use a successful command's exit code as the sole image check; inspect
nonempty content, labels and crops before selecting new graphics.

| Fresh output | Destination relative to `thesis/figures/` |
| --- | --- |
| `association/association-generic.pdf` | `association-generic.pdf` |
| `association/association-products.pdf` | `association-products.pdf` |
| `conditional/conditional-ranks.pdf` | `conditional-ridge-correlation.pdf` |
| `hko-calibration/recovery-by-source-distance.pdf` | `hko-recovery-by-source-distance.pdf` |
| `derivative/derivative-and-kkt-scale.pdf` | `derivative-and-kkt-scale.pdf` |
| `rotation/lagrangian_products_5x5.png` | `rotation-profile.png` |
| `foundations/characteristic-normalization.pdf` | `foundations/characteristic-normalization.pdf` |
| `foundations/facet-polarity.pdf` | `foundations/facet-polarity.pdf` |
| `hko/upper-bound-mechanisms.pdf` | `upper-bound-mechanisms.pdf` |
| `hko/symmetry-extension.pdf` | `symmetry-extension.pdf` |
| `flow/flow-graph-f6-tube-sequence.pdf` | `flow-graph/flow-graph-f6-tube-sequence.pdf` |
| `visualization/figures/viz-hypercube-ridges.png` | `visualization/viz-hypercube-ridges.png` |
| `visualization/figures/viz-hko-pentagon-min-orbit.png` | `visualization/viz-hko-pentagon-min-orbit.png` |

The following stages all thirteen outputs without touching the selected assets:

```bash
figure_source_dir=$(mktemp -d /tmp/msc-thesis-regenerated.XXXXXX)
FIGURE_RUN="$figure_run" FIGURE_SOURCE="$figure_source_dir" python3 - <<'PYTHON'
import os, pathlib, shutil, subprocess
source = pathlib.Path(os.environ['FIGURE_SOURCE'])
figures = pathlib.Path(os.environ['FIGURE_RUN'])
for name in subprocess.check_output(['git', 'ls-files', '-z', 'thesis']).decode().split('\0'):
    if name:
        target = source / name
        target.parent.mkdir(parents=True, exist_ok=True)
        shutil.copy2(name, target)
pairs = {
    'association/association-generic.pdf': 'association-generic.pdf',
    'association/association-products.pdf': 'association-products.pdf',
    'conditional/conditional-ranks.pdf': 'conditional-ridge-correlation.pdf',
    'hko-calibration/recovery-by-source-distance.pdf': 'hko-recovery-by-source-distance.pdf',
    'derivative/derivative-and-kkt-scale.pdf': 'derivative-and-kkt-scale.pdf',
    'rotation/lagrangian_products_5x5.png': 'rotation-profile.png',
    'hko/upper-bound-mechanisms.pdf': 'upper-bound-mechanisms.pdf',
    'hko/symmetry-extension.pdf': 'symmetry-extension.pdf',
    'flow/flow-graph-f6-tube-sequence.pdf': 'flow-graph/flow-graph-f6-tube-sequence.pdf',
}
for name in ['characteristic-normalization.pdf', 'facet-polarity.pdf']:
    pairs['foundations/' + name] = 'foundations/' + name
for name in ['viz-hypercube-ridges.png', 'viz-hko-pentagon-min-orbit.png']:
    pairs['visualization/figures/' + name] = 'visualization/' + name
assert len(pairs) == 13
for fresh, selected in pairs.items():
    shutil.copy2(figures / fresh, source / 'thesis/figures' / selected)
PYTHON
(cd "$figure_source_dir" && bash thesis/check-build.sh)
```

Inspect the resulting `thesis/build/main.pdf`, including figure labels and
crops. The build checker rejects overfull boxes and undefined references.
The dated [rebuild experiment](history/thesis-continuation-2026-10-04/regenerated-assets-rebuild.json)
used the same thirteen-copy mapping and found all 101 page rasters at 96 dpi
and extracted layout text identical to its separately built candidate.
PDF metadata can change bytes without changing figures. The dated comparison
states its raster resolution rather than claiming cross-version byte identity.

The models' missing bulk input and the selected figures' available plot inputs
are distinct. The P2 machine-learning experiment requires the reconstructed
current-schema table described in its [runbook](../experiments/sys-datascience/methods/standard-baseline-p2/README.md);
substituting the historical plotting table drops six ridge descriptors.
