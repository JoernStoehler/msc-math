# Regular Products Experiments

This package owns experiments and proof-support artifacts for Lagrangian
products of rotated regular polygons.

It is separate from `experiments/sys-landscape/` because this side result has a
different thesis role from the hostile-landscape search. The regular-product
packet studies structured polygon families; the sys-landscape package studies
global search behavior and adversarial examples.

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

Exact SageMath executable proof and proof-companion notes for the pentagon
formula. The proof does not depend on the empirical JSONL or figures.

```text
src/
```

Small Rust helpers shared by the regular-product producers:

1. `product_polytope_cache.rs`: neutral product-polytope cache construction.
2. `capacity.rs`: explicit billiard-capacity wrapper.
3. `volume.rs`: exact-incidence volume converted to `f64`.
4. `paths.rs`: package-relative output paths.

## Commands

Broad regular-product sweeps:

```bash
cargo run -p exp-regular-products --release --bin regular-rotated-products
```

Pentagon empirical minima sweep:

```bash
cargo run -p exp-regular-products --release --bin regular-pentagon-rotation-empirics -- --canonical
```

Pentagon empirical figures:

```bash
uv run --script experiments/regular-products/pentagon-rotation-empirics/analyze.py
```

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
