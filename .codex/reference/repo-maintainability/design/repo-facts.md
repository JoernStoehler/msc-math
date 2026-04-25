<!--
Purpose: current-state fact base for the repo maintainability / architecture
program.
Context: this file is the first-pass source for later `ARCHITECTURE.md` and
data-flow docs. It records observed repo facts without proposing target-state
policy unless the current repo already states that policy explicitly.
-->

# Repo Facts

## Status

- Phase: current-state fact collection.
- Date: 2026-04-16.
- Role: source note for later architecture/doc synthesis.
- Non-goal: do not turn open architecture choices into fake settled facts.

## Sources

- [AGENTS.md](/workspaces/msc-math/AGENTS.md:1)
- [TASKS.md](/workspaces/msc-math/TASKS.md:445)
- [crates/symplectic/src/lib.rs](/workspaces/msc-math/crates/symplectic/src/lib.rs:1)
- [crates/symplectic/src/database.rs](/workspaces/msc-math/crates/symplectic/src/database.rs:1)
- [experiments/combinatorial-cells/src/lib.rs](/workspaces/msc-math/experiments/combinatorial-cells/src/lib.rs:1)
- [experiments/hko-local-maximum/src/lib.rs](/workspaces/msc-math/experiments/hko-local-maximum/src/lib.rs:1)
- [experiments/numerics/gradient/src/lib.rs](/workspaces/msc-math/experiments/numerics/gradient/src/lib.rs:1)
- [experiments/sys-landscape/src/lib.rs](/workspaces/msc-math/experiments/sys-landscape/src/lib.rs:1)
- [import-surface-inventory.md](/workspaces/msc-math/.codex/reference/repo-maintainability/design/import-surface-inventory.md:1)
- [shared-helper-inventory.md](/workspaces/msc-math/.codex/reference/repo-maintainability/design/shared-helper-inventory.md:1)
- [data-flow-inventory.md](/workspaces/msc-math/.codex/reference/repo-maintainability/design/data-flow-inventory.md:1)
- [docs-navigation-inventory.md](/workspaces/msc-math/.codex/reference/repo-maintainability/design/docs-navigation-inventory.md:1)
- [execution-constraints-inventory.md](/workspaces/msc-math/.codex/reference/repo-maintainability/design/execution-constraints-inventory.md:1)

## Repo-Wide Structure

- The planned deliverables are:
  - printed thesis in `thesis/build/main.pdf`
  - Rust library in `crates/symplectic/`
  - reproducible experiment pipeline in `experiments/`
- The top-level repo areas and their stated roles are:
  - `crates/symplectic/`: Rust crate `symplectic`
  - `formal/`: developer-facing mathematical sources
  - `experiments/`: Rust/Python experiment packages by topic
  - `.codex/reference/repo-maintainability/`: durable repo-maintainability reference notes
  - `thesis/`: self-contained publication sources
  - `papers/`: downloaded paper sources
  - `TASKS.md`: project tracker
  - `research/README.md`, `research/*.md`, and `tasks/*.md`: thesis story
    interpretation and related work obligations
- `AGENTS.md` is already the root orientation map and always-loaded instruction
  surface.

## Current Documentation Surfaces

- There was no committed top-level `ARCHITECTURE.md` before this session.
- Repo orientation is currently split across:
  - `AGENTS.md`
  - `TASKS.md`
  - `crates/symplectic/src/lib.rs`
  - `crates/symplectic/src/database.rs`
  - per-topic `experiments/<topic>/src/lib.rs` headers
- `crates/symplectic/src/lib.rs` already explains library-internal submodule boundaries
  and dependency direction.
- `crates/symplectic/src/database.rs` already explains that the storage layer does not
  choose a canonical mutable shared cache path.
- Topic `src/lib.rs` files already exist as package-local helper/doc surfaces,
  even when some are still thin.
- The maintainability program state already lives in
  [main.md](/workspaces/msc-math/.codex/reference/repo-maintainability/design/main.md:1)
  and [TASKS.md](/workspaces/msc-math/TASKS.md:445).

## Library Surface Facts

- The simple root reexport surface in `crates/symplectic/src/lib.rs` is small. It
  includes:
  - `ehz_capacity`, `ehz_capacity_pruned`, `ehz_capacity_unpruned`, `ehz_capacity_billiard`, `OrbitSearchResult`
  - `volume`, `omega0`, `lagrangian_product`
  - polygon helpers
  - known-polytopes helpers
  - test utils
  - `Polytope4D`, `ConstructionError`, `Skeleton`, `QhullError`
- Experiments already use deeper public paths beyond those root reexports.
- The current import-surface inventory classifies experiment-facing paths as:
  - simple public
  - expert public
  - accidental internal
  - unclear
- Current boundary-sensitive deep paths include:
  - `symplectic::algorithms::hk2017::orbit_recovery`
  - `symplectic::algorithms::hk2017::permutations`
  - `symplectic::algorithms::billiard::facet_classification`
  - `symplectic::kkt::qp_assembly::build_augmented_system`
- Experiments currently rely on deep paths such as:
  - `symplectic::algorithms::facet_adjacency`
  - `symplectic::kkt::saddle_point_solver`
  - `symplectic::database`
  - `symplectic::random`
  - `symplectic::derivatives`

## Experiment Helper Facts

- Topic packages already have helper-crate entry points at:
  - `experiments/combinatorial-cells/src/lib.rs`
  - `experiments/hko-local-maximum/src/lib.rs`
  - `experiments/numerics/gradient/src/lib.rs`
  - `experiments/sys-landscape/src/lib.rs`
- Some shared logic is already in topic helper crates, but repeated helper
  logic still exists across binaries.
- Repeated helper families currently observed:
  - step-bound event logic
  - sys quotient / ascent scaffold
  - orbit-enumeration wrappers
  - solver instrumentation helpers
- The repeated-helper inventory currently records these candidate homes:
  - step-bound event logic: topic-local helper crate
  - sys quotient and step wrapper: mostly topic-local helper crate, with
    backend policy remaining per-binary local
  - orbit-enumeration wrappers: topic-local helper crate
  - solver instrumentation helpers: unresolved because the output contract is
    not yet stable

## Data And Dataset Facts

- `crates/symplectic/src/database.rs` provides JSONL loading/saving/merge machinery but
  does not define a canonical shared cache path.
- `PolytopeRecord` uses `dual_vertices_rational` and `vertices_rational` as
  defining data, with optional later-filled fields such as `source`, `volume`,
  `capacity`, `sigma_gap_cutoff`, and `sigmas`.
- The following three committed files were byte-identical on 2026-04-16:
  - `experiments/sys-landscape/cache.jsonl`
  - `experiments/combinatorial-cells/polytopes.jsonl`
  - `experiments/verification/orbit-recovery/polytopes.jsonl`
- Their shared SHA-256 hash was
  `8679b89763a10bf1380410f288845f03bcdc8e365035aa31235ff00c9cc07363`.
- `experiments/sys-landscape/variable-f-ascent/cache.jsonl` is intentionally
  local and stores intermediate search states.
- Current data-shape classes observed in experiments:
  - shared polytope catalog rows with dual vertices / vertices / source /
    volume / capacity / best sigma
  - topic-local transient caches
  - analysis-output JSONL files consumed by `analyze.py`
  - run artifacts that also serve as resume inputs
- Some experiment code trusts cached `capacity` and `sigmas.first().perm` as a
  fast path.

## Current Dependency / Ownership Facts

- The thesis is self-contained and must not depend on runtime links into
  `experiments/`, `formal/`, or `crates/symplectic/`.
- `formal/` is developer-facing math, not thesis input.
- Stable code is intended to migrate from `experiments/` into `crates/symplectic/`, but
  exploratory code starts in `experiments/`.
- Library tests are for fast regressions; slow validation and broad sweeps
  belong in `experiments/`.
- Data and figures are colocated with the experiment that produced them.
- `AGENTS.md` says new nested `AGENTS.md` files should not be relied on for
  required instructions.

## Maintainability Tensions Observed

- The current practical experiment-facing library surface is larger than the
  simple root reexport surface.
- Topic helper crates exist, but some repeated logic still remains copied in
  binaries.
- Shared dataset mirrors exist, but canonical-path policy is not yet explicit.
- Repo-level navigation exists in pieces, but not yet in one descriptive
  architecture/data-flow doc pair.
- The discovery notes distinguish between doc-gap rows and architecture-decision
  rows; not every missing explanation is only a documentation problem.

## Fact-Level Open Questions

- Which current deep paths are intended expert-facing dependencies versus
  accidental internals?
- Which repeated helpers should stay per-binary, move to topic helper crates,
  or eventually move into `crates/symplectic/`?
- Which of the byte-identical cache paths should be treated as canonical, if
  any?
- Which current-state facts belong in `AGENTS.md`, `ARCHITECTURE.md`, and a
  separate data-flow doc?
- Which small inconsistencies should be fixed in code or filenames before the
  architecture docs are written around them?

## Next Safe Resume Point

- Use this file as the first-pass fact source for:
  - narrowing `ARCHITECTURE.md` to component/code architecture
  - drafting a separate data-flow doc
  - identifying any small cleanup edits that simplify the architecture story
    without forcing a broad refactor
