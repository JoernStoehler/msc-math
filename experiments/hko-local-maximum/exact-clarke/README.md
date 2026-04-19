# Exact Clarke Checker

Exact first-order proof tooling for the HKO2024 `M_10` local-maximality route.

## Status

Packet 1 completed:

- exact HKO geometry and the explicit symmetry tangent basis are implemented in
  `check.sage`;
- the local devcontainer now exposes a working `sage` command via the baked
  Miniforge / conda-forge install in `.devcontainer/Dockerfile`;
- the same Packet 1 artifacts remain runnable via `check_packet1.py` as a
  pure-Python fallback and for smaller scaffolding packets;
- current numerical exact-minimum surfaces can be summarized with
  `summarize_numerical_minima.py`;
- Packet 2 numerical reconciliation now has a durable bookkeeping surface via
  `classify_numerical_minima.py`;
- the theorem-facing orbit catalog is not closed yet and remains the main
  blocker between the current exact tooling and the final proof surface.
- the exact HKO dual-coordinate volume row is now derived in closed form and
  emitted as a dedicated Packet 3 support artifact.
- the reduced endpoint/midpoint prototype `sys` rows are now emitted exactly in
  the Packet 3 `R^40` coordinate order.
- the current widened Packet 3 representative-row surface is now bundled into the
  backend-neutral witness artifact `widened-seed-witness.json` via
  `build_widened_seed_witness.py`.
- `verify_widened_seed_witness.sage` is now the first concrete Sage verifier
  for Packet 3: it reconstructs the quartic field from that witness and
  replays exact geometry, symmetry-rank, closure, normalization, and representative-row
  rank checks.
- the current widened-seed witness verification now runs successfully in the
  local devcontainer and emits `widened-seed-witness-verification.json`.
- the next blocker is no longer volume-row or row-assembly arithmetic; it is
  permutation-level prototype multiplicity. One endpoint representative plus one midpoint
  representative expand to only `20` symmetry images, so that reduced surface cannot
  reach the target active-matrix rank `25`.
- five numerical six-facet permutation-orbit representatives are now exactified
  into exact endpoint-family `sys` rows, each with exact closure and normalization
  checks.
- six midpoint-style numerical seven-facet permutation-orbit representatives are
  now exactified into exact midpoint-family `sys` rows, each with exact closure and normalization
  checks; only the two asymmetric `lambda ≈ 0.129573855671` seven-facet representative
  classes remain unresolved on the current numerical planning surface.

Terminology note:

- “representative” means one chosen numerical representative of a permutation
  orbit class modulo the currently quotiented symmetries and cyclic relabeling;
- some filenames still contain `seed` for continuity with earlier commits, but
  theorem-facing prose in this note should read those as “representative”.

## Files

| File | Role |
|---|---|
| `check.sage` | SageMath entrypoint for exact HKO geometry and symmetry-tangent certificates |
| `check_packet1.py` | Pure-Python sympy fallback for the Packet 1 exact geometry / symmetry certificates |
| `summarize_numerical_minima.py` | Cross-check summary of the current numerical exact-minimum surface |
| `classify_numerical_minima.py` | Packet 2 bookkeeping classifier for six-facet endpoint classes and seven-facet equality-case classes |
| `count_billiard_sigma_surface.py` | Packet 2 count ladder for raw billiard words, directed-feasible sigma words, valid KKT orbits, and exact minima |
| `classify_billiard_sigma_orbits.py` | Packet 2 symmetry quotient of the directed-feasible sigma surface by cyclic relabeling and the order-10 HKO symplectic symmetry group |
| `probe_exact_billiard_sigma.py` | Packet 2 feasibility probe for exact quartic KKT solves on sampled directed-feasible billiard sigma words |
| `probe_sage_sigma_orbits.sage` | Packet 2 Sage feasibility probe for exact KKT solves on sampled symmetry-quotiented directed-feasible sigma representatives |
| `derive_endpoint_prototype.py` | Exact Packet 2 certificate for one endpoint prototype, one midpoint prototype, and the full equality-case beta segment between neighboring endpoints |
| `derive_segment_gradient_reduction.py` | Exact Packet 2 certificate that the seven-facet KKT segment gives an affine height-gradient family |
| `derive_segment_a_gradient_reduction.py` | Exact Packet 2 certificate that the seven-facet KKT segment gives a degree-2 dual-vertex row family spanned by three prototype rows |
| `derive_hko_volume_derivative.py` | Exact Packet 3 support script deriving the HKO dual-coordinate volume row in facet-major `R^40` order |
| `derive_reduced_sys_prototypes.py` | Exact Packet 3 support script combining prototype capacity rows with the HKO volume row to emit exact prototype `sys` rows |
| `classify_permutation_seed_orbits.py` | Packet 3 planning script classifying numerical representative permutations modulo HKO symmetries and cyclic relabeling |
| `derive_endpoint_seed_rows.py` | Packet 3 support script exactifying the current numerical six-facet representative permutations into exact endpoint-family `sys` rows |
| `derive_midpoint_seed_rows.py` | Packet 3 support script exactifying the midpoint-style numerical seven-facet representative permutations into exact midpoint-family `sys` rows |
| `build_widened_seed_witness.py` | Packet 3 witness assembler that freezes the current widened exact representative-row surface into one backend-neutral JSON artifact |
| `verify_widened_seed_witness.sage` | SageMath verifier for the widened representative-row witness; writes a machine-readable verification summary |
| `billiard-sigma-counts.json` | Generated Packet 2 count ladder for the HKO billiard combinatorics surface |
| `billiard-sigma-orbits.json` | Generated symmetry-quotiented count surface for directed-feasible HKO sigma words |
| `billiard-exact-probe.json` | Generated timing probe for exact quartic KKT solves on sampled directed-feasible sigma words |
| `billiard-sage-probe.json` | Generated Sage timing probe for sampled symmetry-quotiented sigma representatives |
| `hko-geometry.json` | Generated exact geometry record |
| `hko-volume-derivative.json` | Generated exact HKO volume-row certificate |
| `reduced-sys-prototypes.json` | Generated exact reduced prototype `sys` rows and their interpolation/coincidence checks |
| `numerical-permutation-orbits.json` | Generated numerical symmetry-quotiented permutation-seed count surface for Packet 3 planning |
| `endpoint-seed-rows.json` | Generated exact six-facet representative rows chosen from the current numerical permutation-orbit planning surface |
| `midpoint-seed-rows.json` | Generated exact midpoint-style seven-facet representative rows chosen from the current numerical permutation-orbit planning surface |
| `widened-seed-witness.json` | Generated backend-neutral Packet 3 witness bundle for geometry, symmetry, and the current widened exact representative rows |
| `widened-seed-witness-verification.json` | Generated Sage verification summary for the current widened representative-row witness |
| `hko-symmetry-tangent.json` | Generated exact symmetry tangent-space certificate |
| `numerical-minima-summary.json` | Generated current numerical minima summary |
| `numerical-family-reconciliation.json` | Generated Packet 2 bookkeeping summary of endpoint/equality-case classes |
| `endpoint-prototype-certificate.json` | Generated exact endpoint/midpoint prototype beta/action certificate |
| `segment-gradient-reduction.json` | Generated exact gradient-reduction certificate for the equality-case KKT segment |
| `segment-a-gradient-reduction.json` | Generated exact dual-vertex row-reduction certificate for the equality-case KKT segment |

## How To Run

```bash
cd experiments/hko-local-maximum/exact-clarke
sage check.sage
python3 check_packet1.py
python3 summarize_numerical_minima.py
python3 classify_numerical_minima.py
python3 count_billiard_sigma_surface.py
python3 classify_billiard_sigma_orbits.py
python3 probe_exact_billiard_sigma.py --limit 200
sage probe_sage_sigma_orbits.sage --limit 100
python3 derive_endpoint_prototype.py
python3 derive_segment_gradient_reduction.py
python3 derive_segment_a_gradient_reduction.py
python3 derive_hko_volume_derivative.py
python3 derive_reduced_sys_prototypes.py
python3 classify_permutation_seed_orbits.py
python3 derive_endpoint_seed_rows.py
python3 derive_midpoint_seed_rows.py
python3 build_widened_seed_witness.py
sage verify_widened_seed_witness.sage
```

## Scope Boundary

`check.sage` and `check_packet1.py` currently close Packet 1 only:

- exact dual-vertex coordinates;
- actual coefficient field;
- explicit `R^40` symmetry tangent basis and exact rank.

It does **not** yet close the full theorem route, because the final
paper-derived orbit catalog and exact active-gradient matrix are still pending.

`widened-seed-witness.json` and `verify_widened_seed_witness.sage` now give a
concrete Sage-facing Packet 3 surface, but they still do **not** close the
theorem route. They currently verify only:

- the quartic field and dual-geometry bundle already frozen by Packet 1;
- exact symmetry-basis rank;
- exact closure / normalization / common-scalar checks on the current widened
  exact representative rows;
- exact row ranks for the current endpoint and midpoint representative families.

They do **not** yet verify:

- the two unresolved asymmetric seven-facet representative families;
- the final active-gradient matrix `G`;
- the final cone certificate, nor any justified reduction of that theorem to a
  kernel-equals-symmetry summary.

## Sage Note

The local devcontainer now exposes a runnable `sage` binary via the baked
Miniforge / conda-forge install in `.devcontainer/Dockerfile`.

Current verified command:

```bash
cd experiments/hko-local-maximum/exact-clarke
python3 build_widened_seed_witness.py
sage verify_widened_seed_witness.sage
```

That verifier currently passes and emits:

- `widened-seed-witness-verification.json`, with `passed = true`,
- exact symmetry rank `15`,
- endpoint family rank `5`,
- midpoint family rank `6`,
- widened representative-row union rank `11`,
- widened-seed-plus-symmetry rank `26`.

Current exact consequence from that same verification:

- the current widened representative rows annihilate all `15` committed
  symmetry generators exactly, including the `4` translations, the scaling
  direction, and the `10` linear symplectic generators;
- the earlier affine/scaling mismatch was in the representative-row formula,
  not in the symmetry tangent encoding;
- the widened representative-row matrix still has right-kernel dimension `29`,
  so the remaining Packet 3 issue is now active-row multiplicity rather than
  affine-symmetry compatibility.

So the next Packet 3 issue is no longer "fix translation/scaling conventions".
It is "widen the exact active representative surface until the final cone /
matrix certificate becomes visible".

The same witness shape is still intended to stay backend-neutral, so a future
Rust producer should be able to emit it unchanged while Sage remains the
independent verifier.

## Field Note

Packet 1 confirms that the exact dual geometry lives in the quartic field
`Q(tan(pi/5))`, where `tan(pi/5)` satisfies `t^4 - 10 t^2 + 5 = 0`.

This quartic field is not an avoidable normalization artifact:

- the common support height is quadratic;
- the standard-coordinate facet normals already contain `sin(pi/5)` terms
  outside `Q(sqrt(5))`;
- dividing by the support height and globally rescaling the primal polytope do
  not reduce the dual-vertex coordinates to `Q(sqrt(5))`.

## Packet 2 Note

The current numerical exact-minimum artifact splits into two visible surfaces:

- `44` six-facet exact minima, corresponding to the older endpoint-style
  numerical story;
- `106` seven-facet exact minima, consistent with equality-case trajectories
  produced by the paper's three-bounce minimizing-family remark.

The numerical reconciliation script records the current evidence that the
seven-facet gradient classes lie on segments between neighboring six-facet
endpoint classes. This is bookkeeping support for Packet 2, not yet the final
paper-derived theorem-facing catalog.

The symmetry-quotiented billiard combinatorics surface is now also frozen:

- `6,240` directed-feasible sigma words collapse to `628` cyclic
  representatives modulo the order-10 HKO symplectic symmetry group;
- broken down by block count, `k = 2` gives `1600 -> 160` and `k = 3` gives
  `4640 -> 468`.
- orbit sizes are almost all generic: `620` sigma orbits have size `10`, and
  only `8` have stabilizer and size `5`.

So a Sage-first representative-based route would not start from all `6240`
directed-feasible words independently; its symmetry-quotiented front-end
surface is `628`.

The first Sage front-end timing probe is also now committed:

- on a `100`-representative sample from that `628`-orbit surface, the exact
  KKT linear solve stage runs in about `0.0015s` to `0.0046s` per
  representative, with median about `0.0028s`;
- the measured front-end solve cost projects to about `1.84s` for all `628`
  representatives;
- in that sample, `99/100` systems were consistent, with `68` unique solutions
  and `31` positive-dimensional solution spaces.

So the old SymPy-based `~14h` objection does not carry over to the current
Sage front-end solve stage on symmetry-quotiented representatives.

At the current numerical level, the beta-pattern surface compresses much
further than the raw gradient-class counts:

- the six-facet exact minima use one beta multiset up to floating jitter;
- the seven-facet exact minima appear numerically as two beta multisets, but
  the exact prototype certificate now shows that these are representatives on
  one constant-action segment between neighboring endpoint beta profiles.
- for each seven-facet class, the current reconciliation artifact also records
  a facetwise beta-profile convex-combination witness against two neighboring
  six-facet endpoint classes.

The current symmetry-reduced prototype hypothesis is summarized in:

- `experiments/hko-local-maximum/DECISIONS.md`
