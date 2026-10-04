# Seven selected structural figures: fresh reproduction

Measured receipt time: 2026-10-04 01:35:17 UTC.
Owner: `/root/integration/thesis_layout`; receiving owner: `/root/integration`.
Starting source revision: `2704f054` (full revision in the machine receipt).

All seven selected figures were regenerated from their producers into isolated
directories. All three required data exports were also regenerated and are
byte-identical to their retained JSON. The two final viewer PNGs are
byte-identical to the selected thesis PNGs. The five PDFs have different bytes,
but their extracted text and their complete RGB renders at 144 dpi are
identical to the selected thesis PDFs. All seven fresh figures were visually
inspected. No tracked thesis figure or previous delivery bundle was overwritten.

## Results and assets

Fresh outputs are retained under
`figure-reproduction/structural/outputs/` beside this report, with exactly the
same relative paths as beneath `thesis/figures/`:

| Relative path | Producer | Observed comparison |
| --- | --- | --- |
| `foundations/characteristic-normalization.pdf` | `thesis/figures/foundations/generate.py` | Identical text and pixels at 144 dpi |
| `foundations/facet-polarity.pdf` | same | Identical text and pixels at 144 dpi |
| `upper-bound-mechanisms.pdf` | `thesis/figures/make_hko_figures.py` | Identical text and pixels at 144 dpi |
| `symmetry-extension.pdf` | same | Identical text and pixels at 144 dpi |
| `flow-graph/flow-graph-f6-tube-sequence.pdf` | `experiments/dev-flow-graph/visualize-tube/main.rs` and `render.py` | Identical fresh JSON; identical PDF text and pixels at 144 dpi |
| `visualization/viz-hypercube-ridges.png` | `experiments/visualization/` exporter and viewer screenshot producer | Identical fresh JSON and final PNG bytes |
| `visualization/viz-hko-pentagon-min-orbit.png` | same | Identical fresh JSON and final PNG bytes |

`figure-reproduction/structural/receipt.json` records all selected/fresh asset
hashes, producer hashes, command results, versions, comparisons, and limitations.
Its `fresh-data/` directory retains gzip copies of the three fresh JSON exports;
hash comparisons refer to the uncompressed bytes. Its `logs/` preserves both
failed and successful screenshot attempts and the producer logs.

The observed Python environment was Python 3.13.3, Matplotlib 3.11.2 and NumPy
2.5.3, using uv 0.12.11. The retained foundations PDF metadata names Matplotlib
3.11.0; that differs from this run, and no PDF byte-identity promise is made.
Rust and Cargo were 1.94.0. The viewer used Node 22.23.2, Playwright 1.61.1,
Three.js 0.128.0, and Chromium Headless Shell 149.0.7827.55, revision 1228.

## Executed production route

The commands below show the actual isolated destination. Run from the repository
root except where the command names a subshell directory. The two plotting
scripts write beside themselves, so their byte-identical copies were used.
The foundations script also generated two unselected figures; the HKO script
also generated two PNG companions. Those were not substituted into the thesis.

```sh
structural_dir=/tmp/msc-structural-figures-20261004
mkdir -p "$structural_dir/foundations" "$structural_dir/hko" "$structural_dir/flow"
cp thesis/figures/foundations/generate.py "$structural_dir/foundations/"
cp thesis/figures/make_hko_figures.py "$structural_dir/hko/"
uv run --script "$structural_dir/foundations/generate.py"
uv run --with matplotlib --with numpy "$structural_dir/hko/make_hko_figures.py"

cargo run -p exp-dev-flow-graph --release \
  --bin flow-graph-visualize-tube-data -- \
  --facet-count 6 --master-seed 20260605 --attempt 3 --sigma 1,2,4,5,3 \
  --output "$structural_dir/flow/flow-graph-f6-tube.json"
uv run --script experiments/dev-flow-graph/visualize-tube/render.py \
  --layout sequence --input "$structural_dir/flow/flow-graph-f6-tube.json" \
  --output "$structural_dir/flow/flow-graph-f6-tube-sequence.pdf"
```

The viewer directory was copied to `$structural_dir/viewer-packet/viewer/`,
excluding any `node_modules`. The Rust exporter writes only the named JSON;
its Cargo build products are ordinary ignored build outputs. Only the two
selected polytope exports were rerun. The other six inputs were retained copies
used to populate the viewer's menu; they are not shown in these screenshots.

```sh
viewer_dir="$structural_dir/viewer-packet/viewer"
mkdir -p "$viewer_dir"
tar -C experiments/visualization/viewer --exclude=node_modules -cf - . \
  | tar -C "$viewer_dir" -xf -
cargo run --release --manifest-path experiments/visualization/Cargo.toml \
  --bin visualization -- hypercube "$viewer_dir/data/hypercube.json"
cargo run --release --manifest-path experiments/visualization/Cargo.toml \
  --bin visualization -- hko_pentagon "$viewer_dir/data/hko_pentagon.json"
bash "$viewer_dir/embed-data.sh" > "$viewer_dir/data.js"
(cd "$viewer_dir" && npm install --no-save --no-package-lock playwright@1.61.1 three@0.128.0)
(cd "$viewer_dir" && npx playwright@1.61.1 install chromium-headless-shell)
```

Start the copied viewer server in a separate shell:

```sh
cd /tmp/msc-structural-figures-20261004/viewer-packet/viewer
python3 -m http.server 18764 --bind 127.0.0.1
```

Then run the corrected screenshot producer, which writes only the copied
packet's `figures/` directory:

```sh
VIZ_BASE_URL=http://127.0.0.1:18764 node \
  /tmp/msc-structural-figures-20261004/viewer-packet/viewer/screenshot-figures.mjs
```

For an isolated thesis source tree, copy the seven files listed in the result
table from this packet's `outputs/` to the same relative paths beneath that
tree's `figures/`. Integration owns the resulting complete PDF build. The
repository's selected assets remain unchanged.

## Screenshot defect found and repaired

The unmodified producer initially failed because the exact Chromium revision
required by Playwright 1.61.1 was absent. Installing that revision completed.
The subsequent producer exited successfully but saved an entirely white
hypercube PNG. Instrumentation of a second run observed a WebGL context-loss
message, `isContextLost() == true`, and zero nonwhite framebuffer pixels while
the scene groups were populated. Its HKO image was already byte-identical.

Selecting SwiftShader explicitly with `--use-angle=swiftshader` and
`--enable-unsafe-swiftshader` then reproduced both retained PNGs byte for byte.
The maintained `screenshot-figures.mjs` now selects this renderer and checks
an actual rendered frame before saving, rejecting a lost context or an entirely
white frame. The experiment README records that requirement. Two fresh runs of
the corrected producer passed, with 252,970 and 13,568 nonwhite canvas pixels
for the hypercube and HKO respectively, and both final PNGs matched retained
bytes. This is observed behavior on this host, not a guarantee across every
browser, driver, or operating system.

A scratch fault-injection copy called `WEBGL_lose_context.loseContext()` before
the guard. It exited 1 with `WebGL context lost before screenshot` and wrote
zero PNGs. The copy and its log are retained in this packet. To replay that
diagnostic, place `context-loss-test.mjs` beside the copied viewer's
`screenshot-figures.mjs`, so its relative Playwright, Three.js and output paths
resolve as in the recorded run. No artificial fault was added to maintained
producer code.

## Verification limits

Pixel identity at one resolution and identical extracted PDF text do not
establish identical vector internals. These figures are explanatory assets;
their successful reproduction is not a theorem check or validation of every
trajectory export. The fixed screenshot checks detect the observed blank-output
failure but do not replace visual comparison of a figure's intended content.

The existing PDF skill was used; its marker succeeded once for this fresh
seven-PDF production operation (including the two unselected foundations
companions). `git diff --check` passed. This worker made no thesis prose edits,
no selected-asset replacement, and no commit. The integration owner receives
the final output mapping and decides the assembled candidate disposition.
