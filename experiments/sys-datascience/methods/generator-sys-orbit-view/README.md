# Generic sys-orbit section view

## Decision and scope

This target-free packet makes the frozen generic coordinate section available
as one pair/distribution view of four-dimensional facet-normal rows. It applies
the copy-local
`volume_one_omega_labeled_symplectic_frame` construction to the retained
generator-orientation panel and keeps the raw coordinate comparison beside the
section comparison. It does not call `sys`, a capacity backend, or a target.

The intended downstream use is method selection: a later distribution packet
can include this partial section view when its arbitrary-but-canonical frame is
useful, while retaining direct invariant or optimized quotient comparisons
whose geometric meaning may be stronger. This packet does not make the section
the default or privileged representation.

## Mathematical and numerical contract

For normalized inequalities with dual rows in `(q1,q2,p1,p2)` order, the local
adapter follows the frozen candidate in this order:

1. reconstruct volume and scale the body to volume one;
2. translate to the analytic center;
3. form `Omega[i,j] = a_i^T J a_j`;
4. sort each omega row in a fixed descending order, quantize as in the frozen
   candidate, and lexicographically sort facets by those signatures;
5. reject any signature tie as `nonunique_omega_signature`;
6. scan ordered facet quadruples and apply symplectic Gram--Schmidt;
7. express the canonically ordered rows in the first successful symplectic
   frame.

The exact generic-domain statement is
`formal/generic-coordinate-canonization.tex`, especially
`def:generic-coordinate-section` and
`prop:generic-coordinate-canonization`. That note is agent-written and not
Jörn-reviewed. It proves the generic `Sp(4)` and facet-permutation section after
scale fixing and centering. The Rust code is an f64 prototype of the frozen
candidate, not an exact theorem implementation.

The section is partial and generic. It is discontinuous near omega-signature
ties and chooses an arbitrary-but-canonical facet-derived symplectic frame.
Every row has an explicit section status; the section distance is absent unless
both inputs have status `ok`. The symmetric dual cube control deliberately has
tied signatures and must return `nonunique_omega_signature` without emitting
coordinate rows.

The two retained pair views are:

- `raw_coordinate_unordered_row_assignment_rms`: exact minimum-cost row
  assignment followed by Euclidean RMS in the input coordinates. It quotients
  only facet-row permutation.
- `generic_sys_section_unordered_row_assignment_rms`: the same assignment RMS
  after two successful section evaluations. It is a representation distance
  on this generic f64 section, not a theorem that an optimized quotient metric
  was computed.

The report keeps section success counts and numerical residuals—analytic-center
gradient/decrement, frame symplectic defect, and frame-solve residual—separate
from both geometric distances.

## Controls and interpretation

The retained orientation panel contributes eight independent base bodies and
five paired variants per base. The adapter predeclares:

- identity self-distance is numerical zero;
- deterministic and Haar `U(2)` are inside the sys symmetry group, so their
  section distances are at most `1e-5` when both section evaluations succeed;
- Haar `SO(4)` is not expected to vanish. A nonzero value is only finite-panel
  section-view evidence, not an optimized quotient metric theorem or a target
  result.

A second control layer applies seeded positive scale, interior translation,
facet permutation, and sampled full-dimensional local `Sp(4)` transforms to
all eight identity bases. Seed and attempt are retained in every row. The
command fails after writing artifacts if an identity or generic invariance
control is above tolerance, no generic success exists for a required family,
the Haar `SO(4)` panel has no observed nonzero section value, or the tied cube
emits a false representative.

These finite controls establish implementation behavior on this panel. They do
not establish population support, intrinsic dimension, law ranking, mechanism,
or target transfer. Euclidean measurements after the section mean “Euclidean
in this facet-derived canonical frame,” not intrinsic Euclidean geometry of the
body.

Representatives for `O(4)` and `GL(4)` are explicitly deferred. Direct
invariants or optimized comparisons may be preferable for those actions.

## Reproduce

Hydrate the retained input, then run:

```bash
git lfs checkout experiments/sys-datascience/methods/generator-orientation-smoke/artifacts/panel-2-per-bucket/rows.jsonl
cargo test -p exp-generator-sys-orbit-view
cargo run -p exp-generator-sys-orbit-view --release -- \
  --input experiments/sys-datascience/methods/generator-orientation-smoke/artifacts/panel-2-per-bucket/rows.jsonl \
  --out-dir experiments/sys-datascience/methods/generator-sys-orbit-view/artifacts/panel-2-per-bucket
```

`rows.jsonl` and `report.json` are deterministic and bind the exact input,
formal and frozen-candidate sources, local producer sources, repository
revision/tree, and tracked-clean execution predicate. `cost-observation.json`
is separately labeled nondeterministic: it compares the input producer's
retained exact-reconstruction timings with one local release adapter pass, so it
is a cost-scale observation rather than a controlled benchmark.

The retained report is the detailed result source. In this finite panel all
required controls pass, every orientation row is in the section success domain,
the `U(2)` controls collapse numerically, and the `SO(4)` comparisons remain
nonzero. Those statements must retain the scope and caveats above.
