# Regular Products Slice

This is the single entry point for the thesis slice about Lagrangian products
of rotated regular polygons.

The main theorem-strength result is the exact formula for

```text
sys(P_5 x_L R(theta)P_5)
```

on the pentagon rotation fundamental domain. Broad regular-product sweeps and
pentagon figures are supporting context, not proof input.

This package is separate from `experiments/sys-landscape/` because it has a
different thesis role. `experiments/regular-products/` owns the structured
regular-product side result; `experiments/sys-landscape/` owns hostile-search
and data-science evidence.

## Start Here

Read only the row that matches your task.

| Task | Minimum read path | Stop before opening |
| --- | --- | --- |
| Write the thesis section | `thesis/rotated-regular-polygons-content.md`, then `thesis/09-rotated-regular-polygons.tex` | Sage source, generated JSONL/PNG/HTML, stale formal draft |
| Check the exact proof result | `pentagon-rotation-formula-proof/README.md`, then `pentagon-rotation-formula-proof/executable_proof.full.stdout.txt` | empirical folders |
| Inspect the proof code | `pentagon-rotation-formula-proof/README.md`, then `pentagon-rotation-formula-proof/executable_proof.sage.py` | generated artifacts |
| Choose figures | `pentagon-rotation-empirics/README.md`, then the ranked figure list in `thesis/rotated-regular-polygons-content.md` | exact proof source |
| Understand broad regular-product context | `rotated-regular-products/README.md` | pentagon proof internals |
| Recover old calculation details | `formal/pentagon-rotation-capacity.tex` | unless a current guide points to a specific calculation |

Default order for a new agent:

1. Read this README.
2. If writing, read `thesis/rotated-regular-polygons-content.md`.
3. If verifying the proof, read `pentagon-rotation-formula-proof/README.md`.
4. Stop until a concrete question requires a narrower file.

## Do Not Open By Default

These files are useful, but they usually cost more context than they save:

1. generated JSONL, PNG, and HTML artifacts;
2. `pentagon-rotation-formula-proof/executable_proof.sage.py`;
3. `formal/pentagon-rotation-capacity.tex`;
4. broad sweep data files in `rotated-regular-products/`.

Open them only when a README or thesis companion points to a specific detail.

## Who Says What

| File or folder | Role | Current value | Maintenance risk |
| --- | --- | --- | --- |
| `thesis/09-rotated-regular-polygons.tex` | Active thesis section | Contains the current theorem/proof draft for the pentagon formula and selected empirical figures | Needs final Jörn/Kai mathematical and presentation review |
| `thesis/rotated-regular-polygons-content.md` | Thesis writing companion | Best current human/agent guide to theorem, proof route, figures, and wording risks | Not source truth; delete or shrink after prose stabilizes |
| `pentagon-rotation-formula-proof/executable_proof.sage.py` | Exact proof source | Source truth for the open half-domain executable certificate | If edited, rerun the full proof and refresh stdout |
| `pentagon-rotation-formula-proof/executable_proof.full.stdout.txt` | Full proof run output | Source truth for exact run output, status counts, and runtime | Do not hand-edit |
| `pentagon-rotation-formula-proof/README.md` | Proof packet runbook | Best entry point for proof reproduction | Keep short and routing-focused |
| `pentagon-rotation-empirics/` | Sampled pentagon artifacts | Figures, sampled sweep, and orbit viewer for exposition | Not proof input; avoid overclaiming |
| `rotated-regular-products/` | Broad regular-pair sweeps | Context for tested regular polygon products | Empirical only; not a classification theorem |
| `src/` | Shared Rust helpers | Product cache, capacity wrapper, volume helper, package paths | Ordinary code source; keep comments near code |
| `formal/lagrangian-product-rotation-symmetry.tex` | Formal symmetry source | Current rotation/reflection and factor-swap lemmas | Developer-facing proof text, not thesis prose |
| `formal/combinatorial-boundary-regularity.tex` | Formal continuity source | Current preferred endpoint route via EHZ Hausdorff continuity | Broader than this slice |
| `formal/pentagon-rotation-capacity.tex` | Old formal proof draft | Useful for notation and active-branch derivation | Stale body text includes old paths and deleted `cas_witnesses.py` references; no longer input by `formal/main.tex` |
| `experiments/sys-datascience/README.md` and `experiments/sys-datascience/methods/README.md` | Search/data-science context | Explain regular products as structured contrast in the hostile-search story | Do not use it as a proof source for the formula |

## Current Proof Status

The exact executable proof is complete for the open half-domain

```text
0 < theta < pi/10.
```

The endpoint and mirror steps are mathematical writeup steps:

1. **Endpoints:** use EHZ Hausdorff continuity and constant volume.
2. **Mirror:** use the equal odd-pentagon factor-swap symmetry.

The full proof run is recorded in

```text
pentagon-rotation-formula-proof/executable_proof.full.stdout.txt
```

Use that stdout file for exact status counts and runtime.

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
2. `capacity.rs`: explicit billiard-capacity wrapper.
3. `volume.rs`: exact-incidence volume converted to `f64`.
4. `paths.rs`: package-relative output paths.

## Thesis Boundaries

Use this split while writing:

1. `thesis/09-rotated-regular-polygons.tex` owns the regular-product side result:
   formula, proof architecture, selected empirical figures, and
   endpoint/symmetry close.
2. `thesis/08-black-box-datascience.tex` may mention product samples and broad
   regular-product sweeps as hostile-search context. It should not own the
   pentagon formula theorem.
3. `thesis/12-published-code-data.tex` should point to durable reproduction
   artifacts such as the proof script and stdout.
4. No standalone SageMath appendix is active. Verifier explanations stay with
   the theorem sections, and Section 12 owns the compact reproduction pointers;
   reopen an appendix only for a concrete reader need not met there.

## Knowledge-Base Notes

1. **Best current entry point:** this README for inventory, then
   `thesis/rotated-regular-polygons-content.md` for writing.
2. **Most stale current file:** `formal/pentagon-rotation-capacity.tex`.
   Its header marks it stale, but the body still contains historical labels,
   old `experiments/sys-landscape/...` paths, and deleted
   `cas_witnesses.py` references. It is retained only because the thesis
   companion still points to specific calculations in it.
3. **Avoid hidden source truth:** if a claim is about code behavior, check the
   producer script or exact proof script. If a claim is about final thesis
   wording, check `thesis/09-rotated-regular-polygons.tex`; use
   `thesis/rotated-regular-polygons-content.md` for drafting guidance.
4. **Generated artifacts:** do not patch-edit JSONL, HTML, or PNG outputs.
   Regenerate them with the commands below when source behavior changes.

## Commands

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

The owner README records the bounded spike and the explicit-input command for
the retained legacy figures.

Pentagon orbit viewer:

```bash
uv run --script experiments/regular-products/pentagon-rotation-empirics/build_interactive_orbit_viewer.py
```

Pentagon exact proof prefix:

```bash
sage -python experiments/regular-products/pentagon-rotation-formula-proof/executable_proof.sage.py --limit 50
```

Pentagon exact full proof:

```bash
sage -python experiments/regular-products/pentagon-rotation-formula-proof/executable_proof.sage.py --progress-every 500
```

## Layout Policy

Keep each packet flat while it stays readable. Add subfolders only when tooling
creates a real bundle, for example a separate frontend package, a separate Rust
crate, or a large artifact family.
