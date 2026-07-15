# Fixed-normal boundary fractions and productness hierarchy

This target-free exact smoke separates movement inside one fixed-normal
realization chamber from its first locally predicted incidence boundary. It
also records coordinate, Lagrangian, affine, and combinatorial productness as
separate properties instead of treating coordinate recognition as an intrinsic
classifier.

The boundary component uses exact rational `3x3` and `4x6` products. For every
declared simple vertex `I` and nonincident facet `j`, it evaluates

```text
ell_(I,j)(h) = h_j - n_j^T N_I^{-1} h_I.
```

A seeded rational support direction determines the first positive zero among
the slacks with negative directional derivative. Paired rows at fractions
`0.1`, `0.5`, and `0.9` verify strict inactive slacks, an unchanged
facet-labeled vertex-incidence multiset by full exact vertex enumeration, fixed
normals, translation invariance, two positive-scale laws, and the predicted
zero-slack boundary witness. The timed inequalities are only the **intended-simple-vertex
inactive-slack local sufficient system**. The packet does not claim that this
is a complete global characterization of a type-cone chamber.

The preservation matrix checks an exact coordinate product, a non-coordinate
`U(2)` image, a dense exact non-symplectic `SO(4)` image, an `SL(4)` image, and a
same-incidence fixed-normal perturbation. Each row records the explicit
invertible affine construction, complementary factor planes, their exact
Kähler residuals, and the full exact facet-labeled incidence multiset.
Combinatorial evidence is used only after explicit simplicity checks; no graph
Cartesian-factorization theorem is invoked. A failed ambient coordinate-product check therefore does
not mean loss of affine or combinatorial productness. The `SO(4)` control is a
fixed product of rational Givens rotations, not a Haar draw or a generator law.

## Reproduce

From a tracked-clean committed checkout:

```bash
CARGO_TARGET_DIR=/workspaces/msc-math/target cargo run -p exp-sys-landscape \
  --bin sys-datascience-generator-type-cone-productness -- \
  --out-dir experiments/sys-datascience/methods/generator-type-cone-productness/artifacts \
  --seed 20260715
```

For byte-identical retained evidence, check out the `source_revision` recorded
in `artifacts/report.json` before running that command. The artifact commit is a
later wrapper commit, so regenerating from the wrapper itself deliberately
records a different repository revision/tree.

The producer fails closed if any row fails or tracked files are dirty. Outputs:

- `artifacts/boundary-fractions.jsonl`: six deterministic paired fraction rows;
- `artifacts/preservation-matrix.json`: five machine-readable productness rows;
- `artifacts/report.json`: source closure, counts, interpretation limits, and
  explicit deferrals.

The full repository revision/tree and a repo-wide tracked-clean predicate bind
tracked transitive inputs. The smoke contains constructed controls, not a
population sample. It supports implementation and invariance checks for these
fixtures only. It supports no `sys`, capacity, population, law-ranking,
mechanism, target-transfer, or intrinsic-dimension conclusion.

A projective/Hilbert slack-ratio distance is deliberately deferred: this packet
does not need it, so it does not introduce an unreviewed domain or finite-value
contract. Unknown-realization affine recovery and graph Cartesian factorization
are also deferred to a separate classifier/recovery packet if later needed.
