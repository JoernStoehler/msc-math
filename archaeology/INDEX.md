# Archaeology Index

Per-file metadata for recovered content from `msc-viterbo`. All content is untrusted — see `CLAUDE.md`.

## raw/docs/ (51 files)

| File | Type | Origin | Description |
|------|------|--------|-------------|
| algorithm-billiard-spec.md | spec | unknown | Minkowski billiard algorithm spec for Lagrangian products with LP formulation |
| algorithm-hk2019-spec.md | spec | unknown | HK2019 QP algorithm spec with permutation enumeration and complexity bounds |
| algorithm-spec.md | spec | unknown | Tube branch-and-bound algorithm spec for polytopes without Lagrangian 2-faces |
| appendix-experiments-spec.md | thesis-draft | unknown | Thesis appendix spec listing algorithms, fixtures, and benchmarking methodology |
| billiard-SPEC.md | spec | unknown | Billiard algorithm crate spec with 3-bounce LP solver design |
| billiard-TEST_SPEC.md | test-spec | unknown | Billiard test spec with 25 polygon fixtures and 625 product tests |
| billiard-correctness-proof.md | proof | unknown | Billiard correctness analysis with LP epigraph reformulation and triangle verification |
| claims-audit-notes.md | audit | unknown | Audit of uncited mathematical claims across tube, billiard, HK2019 implementations |
| complexity-audit.md | audit | unknown | Complexity audit identifying magic numbers, coupling, test gaps, dead code |
| conv-math-code-correspondence.md | convention | unknown | Conventions for math-code correspondence with tolerance philosophy and proof verification |
| conv-rust-algorithms.md | convention | unknown | Rust algorithm conventions and crate organization reference table |
| developer-spec-v2.md | spec | unknown | Consolidated master spec defining polytopes, 2-faces, Reeb dynamics, all algorithms |
| developer-spec.md | spec | unknown | Deprecated v1 spec retained for historical reference |
| ffi-contract.md | spec | unknown | PyO3 FFI contract with H-rep input validation and output payload spec |
| findings-orbit-validation.md | bug-report | b5d43c9, 05169c3 | Billiard orbit validation bug: only checked q-displacement, missed p-transitions |
| findings-trivialization-bug.md | bug-report | unknown | Trivialization bug: tau_n(V) not bijective on 2-face tangent spaces |
| geom-SPEC.md | spec | unknown | Shared geometry primitives spec: polytopes, volume, systolic ratio, 2D utilities |
| hk2017-SPEC.md | spec | unknown | HK2017 crate spec with Q-function maximization and KKT solver |
| impl-plan-hk2017.md | implementation-plan | unknown | HK2017 implementation plan with TDD methodology and tesseract verification |
| implementation-notes.md | meta | unknown | Rough session notes on closure conditions and flow map normals |
| lit-haim-kislev-2019.md | literature-summary | unknown | HK2019 simple loop theorem: facets visited at most once |
| lit-haim-kislev-ostrover-2024.md | literature-summary | unknown | HK-O 2024 counterexample: pentagon product has systolic ratio 1.047 |
| lit-rudolf-2022-billiard.md | literature-summary | unknown | Rudolf 2022: Minkowski billiard = EHZ capacity for Lagrangian products |
| literature-capacities.md | reference | unknown | Capacity values with paper citations for ball, tesseract, pentagon, simplex |
| math-introduction-draft.md | thesis-draft | unknown | Mathematical introduction draft with combinatorial Hamiltonian and Reeb dynamics |
| mathematical-claims.md | reference | unknown | Mathematical claims list with citations and verification status labels |
| open-questions.md | meta | unknown | Open questions log: all marked resolved as of 2026-01-25 |
| optim-SPEC.md | spec | unknown | Optimization library spec for QP maximization over polytopes |
| proto-math-capacity-ehz.md | thesis-draft | unknown | Early EHZ capacity definition draft |
| proto-math-constructions.md | thesis-draft | unknown | Early mathematical constructions draft |
| proto-math-polar.md | thesis-draft | unknown | Early polar body definition draft |
| proto-math-polytope.md | thesis-draft | unknown | Early polytope definition draft |
| proto-math-symplectic.md | thesis-draft | unknown | Early symplectic form definition draft |
| proto-math-volume.md | thesis-draft | unknown | Early volume computation draft |
| root-SPEC.md | spec | unknown | Root project specification with agents-first design and monorepo structure |
| spec-billiard.md | spec | unknown | Billiard developer spec with 3-bounce enumeration and differential inclusion |
| spec-hk2017.md | spec | unknown | HK2017 developer spec with Q-function formula and naive/graph enumeration |
| test-cases.md | test-spec | unknown | Test case specification |
| test-interpretation.md | test-spec | unknown | Test interpretation specification |
| test-propositions.md | test-spec | unknown | Mathematical propositions for unit testing with ground truth values |
| thesis-02-math.md | thesis-draft | unknown | Math background chapter: symplectic R^4, Reeb orbits, EHZ capacity |
| thesis-02.1-standard-counterexample.md | thesis-draft | unknown | Placeholder chapter for Viterbo counterexample (empty) |
| thesis-algo-minkowski-billiard.md | thesis-draft | unknown | Billiard algorithm chapter spec with pseudocode and implementation plan |
| thesis-algo-oriented-edge-graph.md | thesis-draft | unknown | Oriented-edge graph algorithm chapter with maps, constraints, pseudocode |
| thesis-ehz-capacity.md | thesis-draft | unknown | EHZ capacity definition with Reeb dynamics and CZ index formulas |
| thesis-simplex-conjecture.md | thesis-draft | unknown | Simplex conjecture experiment idea: sys <= 3/4 for 4-simplices |
| thesis-viterbo-counterexample.md | thesis-draft | unknown | Brief Viterbo counterexample chapter outline referencing HK-O 2024 |
| trivialization-derivation.md | proof | unknown | 2-face trivialization derivation using quaternion matrices and CH2021 |
| tube-SPEC-proofs.md | proof | unknown | Tube algorithm proofs |
| tube-SPEC.md | spec | unknown | Tube algorithm spec |
| tube-geometry-spec.md | spec | unknown | Tube geometry specification with trivialization and flow maps |

## raw/code/ (12 files)

| File | Type | Origin | Description |
|------|------|--------|-------------|
| action.rs | implementation | billiard-deleted (46095acd) | Action computation using support functions for 2-bounce and 3-bounce trajectories |
| algorithm.rs | implementation | billiard-deleted (46095acd) | Main billiard algorithm entry point enumerating 2-bounce and 3-bounce edge combinations |
| archive__tube.rs | implementation | algorithm-archive (f613c166) | Tube algorithm with flow maps, action functions, and branch-and-bound search |
| billiard.rs | implementation | algorithm-archive (f613c166) | Billiard algorithm infrastructure including Lagrangian factor extraction and polygon types |
| billiard_lp.rs | implementation | algorithm-archive (f613c166) | LP-based billiard algorithm using linear programming for edge parameter optimization |
| geom.rs | implementation | tube-reverted (2b71e367) | 2D geometry primitives including symplectic form, affine maps, and polygon intersection |
| hk2019.rs | implementation | algorithm-archive (f613c166) | HK2019 quadratic programming algorithm marked as broken with incomplete QP solver |
| polytope.rs | implementation | tube-reverted (2b71e367) | Polytope data structures with vertex enumeration, 2-face enumeration, and enrichment |
| reverted__tube.rs | implementation | tube-reverted (2b71e367) | Branch-and-bound tube algorithm using affine flow maps and priority queue |
| solve.rs | implementation | billiard-deleted (46095acd) | Constrained optimization solver validating achievable billiard trajectories |
| trivialization.rs | implementation | tube-reverted (2b71e367) | 2-face trivialization using quaternion matrices for coordinate transformation |
| types.rs | implementation | billiard-deleted (46095acd) | Core data structures including Polygon2D, LagrangianProduct, and BilliardTrajectory |

## raw/tests/ (23 files)

| File | Type | Origin | Description |
|------|------|--------|-------------|
| algorithm_agreement.rs | test-file | unknown | Tests agreement between billiard and HK2019/tube algorithms on Lagrangian products |
| algorithm_metadata.rs | test-file | unknown | Tests algorithm trait methods, metadata, and input validation |
| billiard_comprehensive_comparison.rs | test-file | unknown | Tests billiard vs HK2017 on square, rectangle, triangle polygon products |
| billiard_orbit_invariants.rs | test-file | unknown | Tests billiard orbit breakpoint positions and closure properties |
| billiard_witness.rs | test-file | unknown | Tests billiard witness orbit geometry and facet constraint verification |
| capacity_known_values.rs | test-file | unknown | Tests capacity computation against hardcoded values (tesseract, triangle, pentagon, etc.) |
| capacity_monotonicity.rs | test-file | unknown | Tests monotonicity axiom K⊆L implies c(K)≤c(L) |
| capacity_scaling_axiom.rs | test-file | unknown | Tests scaling axiom c(λK)=λ²c(K) for billiard and HK2019 |
| capacity_symplectomorphism.rs | test-file | unknown | Tests symplectomorphism invariance c(AK)=c(K) for A∈Sp(4) using HK2019 |
| fixtures.rs | test-file | unknown | Test fixture polytopes and random Lagrangian product generators |
| generators.rs | test-file | unknown | Sp(4) matrix generators and witness property checker functions |
| lagrangian_product.rs | test-file | unknown | Tests Lagrangian product detection and facet index mapping |
| mod.rs | test-file | unknown | Test suite module declarations and topic organization |
| polygon_2d.rs | test-file | unknown | Tests 2D polygon convexity and H-representation conversion |
| polytope_preprocessing.rs | test-file | unknown | Tests PolytopeData construction and Lagrangian 2-face filtering |
| tube_algorithm.rs | test-file | unknown | Tests tube algorithm on non-Lagrangian polytopes and failure modes |
| tube_failure_diagnostic.rs | test-file | unknown | Diagnostic categorizing tube and HK2017 failures on random polytopes |
| tube_fixtures.rs | test-file | unknown | Test fixture polytopes for tube algorithm (cross-polytope, tesseract variants) |
| tube_flow_map_tests.rs | test-file | unknown | Tests flow map symplecticity and determinism |
| tube_hk2017_comparison.rs | test-file | unknown | Tests tube vs HK2017 cross-validation on random 8-facet polytopes |
| tube_integration.rs | test-file | unknown | Tests tube algorithm with scaling axiom and Mahler bound |
| tube_orbit_invariants.rs | test-file | unknown | Tests tube orbit breakpoint positions and Reeb flow properties |
| tube_rotation_debug.rs | test-file | unknown | Debug tests for tube rotation computation on skewed simplex |
