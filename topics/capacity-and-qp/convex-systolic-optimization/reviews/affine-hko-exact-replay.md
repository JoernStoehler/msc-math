# Independent exact replay of affine HKO packets

28 October 2026 UTC. Auditor-owned scratch: `/tmp/msc-math/restart/affine-hko-replay/`. Source packets: `../geometry-frontiers/affine-hko-certificate/` (full) and `../geometry-frontiers/affine-hko-compressed/` (compressed). No source packet was edited. SHA-256 snapshots in `frozen-before.sha256` and `compressed/frozen-before.sha256` both passed `sha256sum -c` after testing.

## Replay and finite checks

- Ran each `verify.py` with `/workspaces/msc-math/.local-environments/miniforge/envs/sage/bin/python` on an isolated copy. Full packet: 268 exactly feasible, reconstructed HK action rows; 62 exact convex dual identities; 50 constructed outer vertices, 10 unit-sphere contacts. Compressed packet: 18 exactly feasible, reconstructed rows; 4 exact convex dual identities; 62 facets covered by symmetry; the same outer-vertex counts. Both exited successfully in under one second of verifier time.
- All 18 serialized compressed rows occur among the full packet's 268 rows. All full-packet rows are used by at least one full-packet dual identity. The full packet has 268 unique row vectors and 262 unique word index sequences: six word sequences have different exact weights. This does not affect the stated distinct-facet condition within each word.
- `compressed/orbit_check.py` checked 20 distinct exact group matrices. The 62 normalized facet normals are unique, and representatives 0, 6, 22, 23 have disjoint orbits of sizes 2, 20, 20, 20, exactly covering them. All 20 six-dimensional/three-dimensional group matrices satisfy `L G = g L`.
- `compressed/action_symmetry_check.py` directly permuted normals and reversed words when the sign is negative. All 54 generator/word transformed actions matched the six-dimensional matrix transformation.
- `compressed/derived_62_check.py` extended this to all 360 word/group combinations: every transformed word remains exactly feasible and reconstructs the transformed bivector. It checked 80 transported convex dual identities, covering all 62 distinct facet normals. The compressed 18-row group orbit contains 224 unique action rows, of which 132 occur in the original full packet. Equality of those row sets is unnecessary: all newly derived rows are feasible by exact reconstruction.
- The certificate JSON contains strings and integers, no decoded floating-point numbers. The verifier uses exact `QQ`/`NumberField` arithmetic for predicates. `RIF(0,1)` only selects the exact algebraic embedding; `time.monotonic()` only reports elapsed time. The bound `r=143/200` is exact. A separate Sage query confirmed `s²=5` and the exact ordered-field sign comparisons.

## Negative controls

All changes below were made to copies only. `negative_controls.py` for the full packet rejected perturbed weight, row coefficient, dual coefficient, missing dual, and wrong `q0` at their expected checks. `compressed/negative_controls.py` additionally rejected a wrong representative identity and a duplicate representative/dual pair at the symmetry-coverage check. This last case leaves four valid dual identities but removes one orbit.

## Limits and division of proof work

This audit establishes the verifier predicates and the compressed symmetry transport, not the analytic HK capacity formula, the exact hull description from the 50 vertices/62 planes, or the affine equality classification. A separate direct Sage `AA` polyhedron H/V conversion was stopped after more than two minutes because it was slow; it returned no counterexample or confirmation. The independent mathematical auditor checked the slice/normal-fan hull argument. No discovery producer was rerun, and neither packet claims completeness of HK words.
