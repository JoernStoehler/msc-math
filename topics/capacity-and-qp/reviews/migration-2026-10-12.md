# Capacity/QP consolidation review, 12 October 2026

Receiving/integrating owner: `/root/migrate/qp_area`. Independent read-only
reviewers: `/root/migrate/qp_area/math_review` and
`/root/migrate/qp_area/api_review`. This is a bounded agent review of repository
consolidation, not Jörn's mathematical approval or thesis PASS.

## Changed ownership and behavior

Five proof packets moved from `formal/` to `topics/capacity-and-qp/proofs/`,
with same-named LaTeX input stubs preserving the combined formal build:
`hk2017-qp-conventions.tex`, `hk2017-qp-core.tex`,
`hk2017-qp-precision.tex`, `ehz-kkt-system.tex`, and
`search-pruning-correctness.tex`. Precision now composes
`direct-kkt-certificate.tex` and `conditional-precision.tex`.

The convention and QP-core bodies were preserved byte-for-byte against the
assigned dirty-tree baseline. All ten precision proof environments were
preserved byte-for-byte. Outside those proofs, two descriptions now identify
the selected production implementation rather than a development-only route,
and the precision status remark routes the two distinct source packets.

The duplicate transition theorem in pruning was replaced by a reference to the
canonical KKT lemma, carrying both pre-existing labels. Its matching hypotheses
were checked separately. The canonical proof now handles empty blocking sets
without requiring their minimum slack, retains the nonempty facet-intersection
condition, and chooses arbitrary positive travel time when every directional
constraint derivative vanishes instead of dividing by zero. Its `unverified`
status was retained.

The pruning narrative now says refinements are at least as restrictive, since
A2 and A3 coincide on simple polytopes. The reviewer found an inherited claim
that A3 had an active Fourier–Motzkin implementation and negligible O(F^4)
precompute cost. The source now identifies A3 as a mathematical/historical
refinement and active production as A2; the unsupported runtime claim was
removed. Old example enumeration counts are explicitly retired observations,
not newly regenerated evidence.

Four durable September packets moved byte-for-byte from `docs/` to
`topics/capacity-and-qp/evidence/`, retaining their same directory names:
`qp-derivation-review`, `numerics-correspondence-check`, `qp-flow-integration`
and `imported-theorem-contracts`. The fourteen packet files retain all frozen
patches, proposals, hashes and source identities. The
[receipt](../evidence/relocation-2026-10-12.json) records their file hashes;
[the evidence index](../evidence/README.md) explains interrupted-assembly
applicability and explicit flow/product dependencies. Historical absolute paths
remain provenance. No patch was applied to the selected thesis.

Four Rust KKT files have documentation-only corrections: `mod.rs`,
`projection_solver.rs`, `rational_solver.rs`, and `saddle_point_solver.rs`.
They now distinguish numerical stationary models from exact null directions,
stationary objective from fixed-word optimum, current dual-vertex assembly from
normal/height translation, and legacy residual diagnostics from certificates.
The erroneous capacity square and backwards diagnostic comparison were fixed.
No executable Rust expression, signature, threshold, output type or policy was
changed. The development README separately identifies production certified
complete-candidate capacity, retained-f64-candidate aggregation and full exact
candidate resolution. The substantive topic account connects their mathematical
and evidence boundaries.

## Independent verdicts and scope

**Mathematical/source reviewer: approved.** It checked baseline preservation,
transition-hypothesis equivalence, positive-time rescaling, empty blocker and
zero-derivative branches, label aliases, formal input order and pruning-strength
wording. Its implementation/status findings above were repaired and re-read.
It spot-checked production certificate naming, staged bounds, gradual underflow
and pinned dependencies. It verified all fourteen relocated packet files
against both the receipt and original Git `HEAD:docs/...` bytes.

Not reviewed as a whole: machine implementation correctness of every certificate,
outer candidate coverage, historical A3 producers/counts, or the conditional
precision chain's known mathematical gaps. No theorem promotion was inferred.

**API/documentation reviewer: approved.** It inspected current production general
and product routes, raw f64 KKT kernels and exact rational witness semantics,
then checked the final topic account, four KKT documentation changes,
development README and evidence index. Its product-proof link, wider-window
curvature qualifier and explicit outward defect-bound condition corrections
were integrated. Its final comment-only review confirmed the distinctions
between numerical labels and exact eigenpair/residual/null-direction arguments.

This is documentation/API review, not independent machine-arithmetic verification,
proof-wide acceptance or all-input algorithm equivalence. No implementation
changes, producers or numerical campaigns were requested or performed.

## Checks

- The combined formal document compiled successfully with `latexmk -pdf
  -interaction=nonstopmode -halt-on-error main.tex` from `formal/`. The local
  build produced 114 pages. No undefined-reference or duplicate-label warnings
  were found; existing font-shape substitution warnings remain. Compilation
  checks source paths and syntax, not mathematical correctness.
- A baseline/current label scan found all 76 moved labels retained once, with
  no duplicates in the topic proof packet.
- The ten precision proof environments and complete convention/QP-core source
  bodies matched the assigned baseline as described above.
- Focused `rustfmt --check --edition 2021` on the four changed KKT Rust files
  passed. Executable bodies were unchanged, so new algorithm tests were not
  added or run for this documentation-only correction.
- Relocation compared every packet file hash before and after the move. The
  integration proposal's three proposed-source hashes still match its manifest;
  original interrupted prerequisites were not revalidated against a newer
  selected thesis.
- Local topic/development README links and owned diff whitespace were checked.
  Cross-owner consumers were handed to the shared-interface/inventory owners;
  assembled repository checks belong to the migration coordinator.

## Open boundaries and receiving owners

The conditional computable-beta proposal, Type A/Type B near-null LP rationale,
source-algebraic-to-binary64 distortion bounds, complete within-word/physical
minimizer families and broad certificate implementation verification remain
scientific obligations outside this consolidation. Their pre-existing gaps
were not repaired or hidden by source relocation.

The shared-interface owner received the old-to-new proof/evidence maps for
external documentation, scripts and thesis references. The inventory owner
received the same maps. Physical crate movement was deferred: public imports
and the coupled experiment/production correspondence remain stable, and no
Rust consumer or behavior change was required by this area cleanup.
