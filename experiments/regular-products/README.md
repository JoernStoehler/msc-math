# Regular Products Slice

This is the experiment entry point for Lagrangian products of rotated regular
polygons. The selected mathematical proofs are in
`thesis/chapters/09-rotated-regular-polygons.tex` and
`thesis/chapters/10-affine-pentagons.tex`.

The main theorem-strength result is the exact formula for

```text
sys(P_5 x_L R(theta)P_5)
```

on the full rotation domain. The selected chapter uses endpoint interpolation
from the published HKO capacity. The earlier exhaustive Sage proof is retained
here as a separate route. Broad regular-product sweeps and pentagon figures
are supporting context, not proof input.

This package is separate from `experiments/sys-landscape/` because it has a
different thesis role. `experiments/regular-products/` owns the regular-product
experiments and retained certificate; `experiments/sys-datascience/` owns current
random/product data-science consumer work, and `experiments/sys-landscape/`
contains retained search implementations and historical context.

## Start Here

Paths beginning `thesis/`, `formal/` or `experiments/` are repository-root
relative; other paths below are relative to this directory.

| Task | Relevant sources |
| --- | --- |
| Read or update the selected rotation proof | `thesis/chapters/09-rotated-regular-polygons.tex` |
| Read the independent linear-deformation extension | `thesis/chapters/10-affine-pentagons.tex`, with maintained development at `formal/pentagon-affine-products/README.md` |
| Inspect the recorded certificate run | `pentagon-rotation-formula-proof/README.md` and `pentagon-rotation-formula-proof/executable_proof.full.stdout.txt` |
| Review the earlier executable certificate | `pentagon-rotation-formula-proof/executable_proof.sage.py`, with the packet README and the endpoint/symmetry boundary below |
| Choose figures | `thesis/chapters/09-rotated-regular-polygons.tex`, `rotated-regular-products/README.md`, `pentagon-rotation-empirics/README.md` |
| Understand broad regular-product context | `rotated-regular-products/README.md` |
| Recover old calculation details | `formal/legacy/pentagon-rotation-capacity.tex` and relevant Git history; see its limitations below |

Follow the dependencies of the claim being checked. The selected analytic
proof does not use the Sage classification; a claim about that classification
requires its code and output. A successful recorded run is not by itself a
review of what the program proves.

## Who Says What

| File or folder | Role | Current value | Maintenance risk |
| --- | --- | --- | --- |
| `thesis/chapters/09-rotated-regular-polygons.tex` | Selected thesis section | Analytic endpoint-interpolation proof and selected empirical figure | Inclusion does not establish final human mathematical or presentation acceptance |
| `formal/pentagon-affine-products/README.md` | Proof-route map | Maintained independent linear-deformation proof, source contract and links to the selected chapters | Agent-reviewed extension; human acceptance and novelty are separate questions |
| `pentagon-rotation-formula-proof/executable_proof.sage.py` | Exact proof source | Source truth for the open half-domain executable certificate | If edited, rerun the full proof and refresh stdout |
| `pentagon-rotation-formula-proof/executable_proof.full.stdout.txt` | Full proof run output | Source truth for exact run output, status counts, and runtime | Do not hand-edit |
| `pentagon-rotation-formula-proof/README.md` | Earlier proof packet runbook | Entry point for reproducing the exhaustive certificate | Distinguish it from the selected analytic proof |
| `pentagon-rotation-empirics/` | Sampled pentagon artifacts | Figures, sampled sweep, and orbit viewer for exposition | Not proof input; avoid overclaiming |
| `rotated-regular-products/` | Broad regular-pair sweeps | Context for tested regular polygon products | Empirical only; not a classification theorem |
| `src/` | Shared Rust helpers | Product cache, capacity wrapper, volume helper, package paths | Ordinary code source; keep comments near code |
| `formal/lagrangian-product-rotation-symmetry.tex` | Formal symmetry source | Current rotation/reflection and factor-swap lemmas | Developer-facing proof text, not thesis prose |
| `formal/combinatorial-boundary-regularity.tex` | Formal continuity source | Supports the endpoint-continuity step of the earlier exhaustive route | The selected interpolation proof includes its endpoints directly |
| `formal/legacy/pentagon-rotation-capacity.tex` | Old formal proof draft | Useful for notation and active-branch derivation | Stale body text includes old paths and deleted `cas_witnesses.py` references; no longer input by `formal/main.tex` |
| `experiments/sys-datascience/README.md` and `experiments/sys-datascience/methods/README.md` | Search/data-science context | Explain regular products as structured contrast in the hostile-search story | Do not use it as a proof source for the formula |

## Current Proof Status

The selected rotation chapter proves the formula at every angle using the
HKO endpoint values, positive trigonometric interpolation, an attaining
feasible word and symmetry. The independent affine extension gives a second
analytic route without importing the HKO endpoint value.

The earlier exact executable proof covers the open half-domain

```text
0 < theta < pi/10.
```

Its endpoint and mirror steps are mathematical arguments outside the run:

1. **Endpoints:** use EHZ Hausdorff continuity and constant volume.
2. **Mirror:** use the equal odd-pentagon factor-swap symmetry.

The full proof run is recorded in

```text
pentagon-rotation-formula-proof/executable_proof.full.stdout.txt
```

Use that stdout file for exact status counts and runtime. Neither selected
analytic proof requires rerunning this certificate.

## Folder Layout

```text
rotated-regular-products/
```

Broad empirical sweeps over regular polygon pairs.

```text
pentagon-rotation-empirics/
```

Sampled pentagon data, static figures, and the standalone orbit-projection
viewer. These artifacts are empirical and illustrative.

```text
pentagon-rotation-formula-proof/
```

Exact SageMath executable proof and code-audit notes for the pentagon
formula. The proof does not depend on the empirical JSONL or figures.

```text
src/
```

Small Rust helpers shared by the regular-product producers:

1. `product_polytope_cache.rs`: neutral product-polytope cache construction.
2. `capacity.rs`: production product-minimizer adapter returning exact capacity,
   outward bounds, and one deterministic minimizing word for bounce counting.
3. `volume.rs`: exact-incidence volume converted to `f64`.
4. `paths.rs`: package-relative output paths.

## Thesis Boundaries

Use this split while writing:

1. `thesis/chapters/09-rotated-regular-polygons.tex` owns the rotation formula,
   its analytic proof and selected empirical figure.
   `thesis/chapters/10-affine-pentagons.tex` owns the independent linear-factor
   extension; its development source is `formal/pentagon-affine-products/`.
2. `thesis/chapters/08-data-science.tex` may mention product samples and broad
   regular-product sweeps as hostile-search context. It should not own the
   pentagon formula theorem.
3. `thesis/chapters/14-code-data.tex` owns the compact code/data and reproduction
   pointers. The retained Sage script and stdout reproduce the earlier route,
   not a computational dependency of the selected rotation proof.
4. The selected manuscript has no pentagon Sage appendix. Its HKO certificate
   appendix concerns a different theorem.

## Knowledge-Base Notes

1. **Current entry point:** this README for inventory, then
   `thesis/chapters/09-rotated-regular-polygons.tex` for thesis wording.
2. **Historical formal draft:** `formal/legacy/pentagon-rotation-capacity.tex`.
   Its header marks it stale, but the body still contains historical labels,
   old `experiments/sys-landscape/...` paths, and deleted
   `cas_witnesses.py` references. Use it only to recover specific historical
   calculations, checking them against current sources.
3. **Avoid hidden source truth:** if a claim is about code behavior, check the
   producer script or exact proof script. If a claim is about final thesis
   wording, check `thesis/chapters/09-rotated-regular-polygons.tex`.
4. **Generated artifacts:** do not patch-edit JSONL, HTML, or PNG outputs.
   Regenerate them with the commands below when source behavior changes.

## Commands

These are reproduction/producer commands, not a thesis-build checklist. The
Rust sweeps and viewer command write generated artifacts; inspect their local
READMEs before replacing retained evidence. The full Sage run can be expensive.

Broad regular-product sweeps:

```bash
cargo run -p exp-regular-products --release --bin regular-rotated-products
```

Pentagon empirical minima sweep:

```bash
cargo run -p exp-regular-products --release --bin regular-pentagon-rotation-empirics -- --canonical
```

Pentagon sampled KKT-branch landscape:

```bash
cargo run -p exp-regular-products --release --bin regular-pentagon-rotation-empirics -- --branch-landscape --canonical
uv run --script experiments/regular-products/pentagon-rotation-empirics/analyze.py landscape \
  --input experiments/regular-products/pentagon-rotation-empirics/kkt-branch-landscape.jsonl
```

The packet README records the bounded spike and the explicit-input command for
the retained legacy figures.

Pentagon orbit viewer:

```bash
uv run --script experiments/regular-products/pentagon-rotation-empirics/build_interactive_orbit_viewer.py
```

Pentagon exact proof prefix:

```bash
conda run --name sage python experiments/regular-products/pentagon-rotation-formula-proof/executable_proof.sage.py --limit 50
```

Pentagon exact full proof:

```bash
conda run --name sage python experiments/regular-products/pentagon-rotation-formula-proof/executable_proof.sage.py --progress-every 500
```
