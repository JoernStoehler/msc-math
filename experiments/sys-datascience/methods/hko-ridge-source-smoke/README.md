# HKO Ridge Source Smoke

Question: for a fixed HKO reference and a tiny deterministic perturbation
sample, do sys-datascience ridge-area features move in the same direction as
the copied HKO-local smoke packet suggested?

This packet exists to fix ownership/provenance, not to establish a strong
thesis result. It is sys-datascience-owned because the checked mechanism is a
method/reference feature behavior: HKO is used as a fixed reference row, and
the perturbations are only a small deterministic smoke sample.

Run from the repository root:

```bash
cargo run -p exp-sys-datascience --release \
  --bin sys-datascience-hko-ridge-source-smoke -- \
  --out-dir experiments/sys-datascience/methods/hko-ridge-source-smoke/artifacts
```

Source/provenance:

- base geometry comes from `symplectic::geom::known_polytopes::hko_pentagon`;
- perturbations are generated in `src/main.rs` with fixed `seed = 42`,
  `epsilon = 0.01`, and `perturbed_count = 8`;
- volume, capacity, and `sys` are recomputed through `exp-sys-landscape`
  computation APIs;
- ridge-area features call
  `experiments/sys-datascience/prepare/features_face_symplectic.rs` and use the
  same `/ sqrt(volume)` normalization as the prepared sys-datascience table.

No generated ridge rows from `hko-ridge-area-packet` or
`sys-ds-hko-local-ridge-smoke` are inputs. Those names are recorded only as
history for the ownership problem that motivated this source rewrite.

Interpretation boundary: this is an empirical smoke check over one fixed random
seed and eight accepted perturbations. Treat it as a provenance-clean reference
packet and regression target, not as evidence for HKO local optimality.
