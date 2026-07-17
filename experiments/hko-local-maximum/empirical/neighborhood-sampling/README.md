# HKO Neighborhood Sampling

This experiment groups nearby-polytope random samplers under one Rust binary.
Each sampler writes artifacts into its own subfolder.

## Samplers

| Sampler | Command | Artifact folder | Meaning |
| --- | --- | --- | --- |
| `m10` | `hko-neighborhood-sampling m10` | `m10/` | General fixed-`F=10` dual-vertex perturbations near HKO. |
| `m11` | `hko-neighborhood-sampling m11` | `m11/` | Add one cutting facet to move from `F=10` to `F=11`. |
| `m10-lagrangian-product` | `hko-neighborhood-sampling m10-lagrangian-product` | `m10-lagrangian-product/` | Fixed-`F=10` perturbations that preserve the 5 `q` + 5 `p` Lagrangian-product structure. |
| `m10-lagrangian-product-probe` | `hko-neighborhood-sampling m10-lagrangian-product-probe` | `m10-lagrangian-product/` | Radial boundary probe in the same fixed-`F=10` Lagrangian-product family. |
| `m10-quotient-ray` | `hko-neighborhood-sampling m10-quotient-ray` | Explicit `--out-dir` | Event-labelled finite shell screen in the fixed 25-dimensional Euclidean HKO local slice. |

There is no current pure `m11-lagrangian-product` sampler. A true such sampler
would add a product-preserving facet, giving `6 q + 5 p` or `5 q + 6 p`.

## Files

- `main.rs`: dispatches to one sampler.
- `samplers/`: sampler implementations.
- `m10/`, `m11/`, `m10-lagrangian-product/`: tracked data, figures, analysis
  scripts, and job scripts owned by each sampler.

## Frozen HKO quotient-ray screen

The canonical retained invocation is:

```bash
cargo run -p exp-hko-local-maximum --release --bin hko-neighborhood-sampling -- \
  m10-quotient-ray --frozen-panel \
  --seed 44 --directions 32 --r-max 0.5 --bisect-tol 1e-4 \
  --out-dir /path/to/new-empty-output-directory
```

`--frozen-panel` rejects noncanonical CLI settings, debug builds, a dirty Git
worktree, or a mismatch between the sampler source compiled into the binary and
the checked-out sampler source. The manifest retains the exact process
invocation, clean commit/tree, toolchain and profile, executable hash, compiled
sampler-source hash, relevant local source-tree hashes, all numerical settings,
and claim boundaries. After all JSON/JSONL outputs are flushed, the producer
writes `artifact-bundle.json`, which hashes every other emitted artifact.

This is a nominal mechanism/readiness screen over 32 seeded directions, not a
capacity-certified positivity boundary. Radii are ambient 40-dimensional
Euclidean displacements in one fixed labelled-coordinate gauge. The measure is
gauge/metric dependent, and the output establishes no global quotient,
positivity inradius, monotonicity, trapping, star-shapedness, stable radius
distribution, population probability, or exclusion of rare thin tubes. The
binary/source binding is auditable but not a cryptographic Rust build
attestation for every linked path-dependency object; that residual is recorded
in the manifest.
