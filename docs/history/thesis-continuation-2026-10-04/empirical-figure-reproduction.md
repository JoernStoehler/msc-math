# Selected empirical figure reproduction

Checked 2026-10-04, 01:29–01:33 UTC, on source checkpoint `2704f054`
with the two output-routing changes described below. All six requested
figures were regenerated successfully without reevaluating any capacity or
overwriting retained inputs, experiment evidence or selected thesis assets.
The integration owner received the fresh files for its separate assembled-PDF
check. This report does not modify or supersede the immutable prior bundle.

## Outcome and comparison contract

The five PDF figures render pixel-for-pixel identically to their selected
thesis copies with Poppler 24.02.0 at 100 dpi. The rotation PNG has identical
decoded pixels at its native 795 × 510 resolution. The actual PDF/PNG byte
hashes differ. PDF metadata records Matplotlib 3.10.8 for the association
figures and 3.11.2 for the other fresh plots; the selected rotation PNG records
3.11.0. This is a successful rendering and numerical-output reproduction,
not a claim of byte-identical figure files or identical environments.

Eight retained outputs also reproduce byte-for-byte: the three conditional-rank
TSV/checksum tables; calibration state, method and compute CSVs plus its
`analysis.json`; and the derivative audit's generated `REPORT.md`. All five
plotting commands exited zero, in approximately 12.21, 3.62, 4.29, 2.84 and
1.94 seconds respectively. The figure comparison contact sheet was inspected;
no new clipping, overlap, absent panel or label defect was observed.

[The machine receipt](figure-reproduction/receipt.json) binds the exact source
commit, working producer hashes, helper hash, commands, input hashes, output
hashes, actual timestamps, versions and each comparison. It confirms that the
inputs and all six selected thesis assets were unchanged. Command logs and
fresh outputs are retained under [figure-reproduction/](figure-reproduction/).

## Producer, input and publication-copy map

All paths in this table are repository-relative unless identified as a local
cache. `OUT` means a fresh output directory chosen by the caller; the actual
run used the receipt's absolute paths below this report directory.

| Selected asset under `thesis/figures/` | Plot producer and input | Fresh output and deliberate copy step |
| --- | --- | --- |
| `association-generic.pdf` and `association-products.pdf` | `thesis/figures/plot_association.py`; historical invariant-table snapshot plus tracked `experiments/polytope-invariant-table/polytope-provenance-table.jsonl` | `--out OUT/association` writes both names unchanged and `plot-receipt.json`. Review and copy the corresponding PDFs into the selected asset directory only when refreshing publication copies. |
| `conditional-ridge-correlation.pdf` | `experiments/sys-datascience/methods/selection-mechanism/analyze.py`; the same two hash-guarded inputs | `--out OUT/conditional` writes `conditional-ranks.pdf`; its reviewed publication copy is renamed `conditional-ridge-correlation.pdf`. |
| `hko-recovery-by-source-distance.pdf` | `experiments/dev-gradient-ascent/ascent-continuation/analyze_hko_calibration.py`; `artifacts/hko-one-step-development-panel-20260729/raw/{summary.json,steps.jsonl,candidates.jsonl}` relative to that experiment | Positional `RAW OUT/hko-calibration` writes `recovery-by-source-distance.pdf`; its reviewed publication copy adds the `hko-` prefix. |
| `derivative-and-kkt-scale.pdf` | `experiments/dev-gradient-ascent/endpoint-model-audit/analyze.py`; `artifacts/directional-decomposition-20260729/raw/audit.json` relative to that experiment | Positional `AUDIT OUT/derivative` writes the selected filename unchanged. |
| `rotation-profile.png` | `experiments/regular-products/rotated-regular-products/analyze.py`, shared `experiments/figure_config.py`, and local `lagrangian-products-5x5.jsonl` | `--only-pentagon --out-dir OUT/rotation` writes `lagrangian_products_5x5.png`; its reviewed publication copy is renamed `rotation-profile.png`. |

The invariant-table input was the already available local cache file
`/home/joern/.cache/msc-math/artifacts/polytope-invariant-table/c3042846723dbb89db17f2f39d88004d937e1e790949ff6880188fa89fc1900a/files/polytope-table.jsonl`
(24,250,539 bytes, SHA-256
`607c8731fa03d190d497edc3e8f1b4cca88f7d238260cce527680f568bc33d59`).
The provenance table hash is
`6ff88a5accce9a7ec7e5a494107350b0974b2ce0268ea44caae36a18a7494ef2`.
Both DS producers check those hashes. The integration owner separately checked
the cache manifest and payload with its new offline `verify-cache` command;
this worker did not download from R2 or establish remote availability.

## Commands and write effects

Run from the repository root. The exact executed commands are also retained in
the JSON receipt. `TABLE` names the verified invariant table above; `OUT` is a
new directory. Keep `PYTHONOPTIMIZE=0` because the DS input guards use assertions.

```sh
OUT=$(mktemp -d)
TABLE=/home/joern/.cache/msc-math/artifacts/polytope-invariant-table/c3042846723dbb89db17f2f39d88004d937e1e790949ff6880188fa89fc1900a/files/polytope-table.jsonl
export PYTHONOPTIMIZE=0 MPLBACKEND=Agg OPENBLAS_NUM_THREADS=1

uv run --script thesis/figures/plot_association.py \
  --table "$TABLE" \
  --provenance experiments/polytope-invariant-table/polytope-provenance-table.jsonl \
  --out "$OUT/association"
uv run --script experiments/sys-datascience/methods/selection-mechanism/analyze.py \
  --table "$TABLE" \
  --provenance experiments/polytope-invariant-table/polytope-provenance-table.jsonl \
  --out "$OUT/conditional"
uv run --script experiments/dev-gradient-ascent/ascent-continuation/analyze_hko_calibration.py \
  experiments/dev-gradient-ascent/ascent-continuation/artifacts/hko-one-step-development-panel-20260729/raw \
  "$OUT/hko-calibration"
uv run --script experiments/dev-gradient-ascent/endpoint-model-audit/analyze.py \
  experiments/dev-gradient-ascent/endpoint-model-audit/artifacts/directional-decomposition-20260729/raw/audit.json \
  "$OUT/derivative"
uv run --script experiments/regular-products/rotated-regular-products/analyze.py \
  --only-pentagon --out-dir "$OUT/rotation"
```

These commands read retained data and write only their chosen output directories
(plus normal uv dependency caches). They also generate companion summaries or
plots in those directories. The capacity/data-generation commands shown in the
experiment READMEs were inspected but not run. No copying into `thesis/figures/`
was performed by this worker; the integration owner stages reviewed copies in
an isolated source tree for the next check.

## Usability fixes and scope

`plot_association.py` previously always wrote beside its source; it now accepts
`--out` while preserving the previous default. Rotation `analyze.py` now accepts
`--data-dir`, `--out-dir` and `--only-pentagon`, preserving the default full plot
set. Its README documents the safe selected-plot command and exact publication
rename. These options were exercised by the successful runs, and the edited
files pass `git diff --check`.

The integration owner also assigned the obsolete routing in
`docs/ds-evidence-closure/search-account.md`. That page now marks its proposed
organization and open choices as historical, links current Chapter 8 and the
data-science appendix, and distinguishes the former draft's explanatory example
from the selected account. Its scientific results were not changed.

The association/conditional figures use all 14,336 historical numerical targets,
with eight generic and ten product groups. Calibration reanalysis covers 16
selected perturbations plus its HKO control; the derivative audit covers three
selected cases; the rotation plot uses 37 historical samples and overlays the
analytic profile. Plot reproduction does not recover historical execution
receipts, certify historical ratios, rerun geometry/capacity calculations,
establish a general convergence statement or prove either pentagon theorem.
Reproduction on another machine still requires the registered table and suitable
plotting dependencies. Final PDF assembly and acceptance remain separately owned.
