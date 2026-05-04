<!--
Purpose: Rust-facing tech-debt cleanup roadmap for final thesis closeout.
Context: this bundle routes cleanup work that affects agent velocity,
experiment safety, validation trust, or durable crate maintainability.
-->

# Rust Tech Debt Roadmap

## Status

- State: active.
- Last updated: 2026-05-04.
- Source surfaces: `crates/`, `experiments/`, `crates/MAP.md`,
  `experiments/MAP.md`, `research/verification.md`, relevant topic bundles,
  and `/tmp/rust-tech-debt-map.md` as an untracked exploration input.
- Refresh when: a Rust cleanup packet changes API support levels, experiment
  command safety, generated-output ownership, validation commands, or the
  interpretation of a retained thesis claim.

## Steering Cache

- [agent synthesis 2026-05-04] The strongest current pattern is unclear
  operating contracts, not one chosen architecture. The repeated agent-cost
  questions are: which command is safe, which output is canonical, which API is
  supported, which duplicate owns current data, which blocked code can be
  ignored, and which validation command protects a refactor.
  Source: `/tmp/rust-tech-debt-map.md`, spot-checked against `crates/MAP.md`,
  `experiments/MAP.md`, `research/verification.md`, and `tasks/*.md`.
  Why it matters: cleanup should proceed by independent packets unless a packet
  proves that an architecture decision is now worth Jörn's time.
- [accepted 2026-05-04] Consult Jörn for high-risk architecture/API/data-shape
  decisions. Do not consult him for low-risk, easily reversible mechanical
  cleanup where more evidence is unlikely to change the choice.
  Source: Jörn chat instruction.
  Why it matters: keeps scarce decision time for choices that are expensive to
  unwind.

## Work Map

| item | state | value class | owner/gate | next action | source |
| --- | --- | --- | --- | --- | --- |
| Broad Rust lint gate | `[done]` | map input | agents | Keep `cargo clippy --workspace --all-targets -- -D warnings` green; use it as a cheap first-pass regression gate for future cleanup packets. | current branch, Clippy output |
| Safe experiment command contracts | `[active]` | mainline thesis | agents, Jörn only for retained-output policy | Continue package-by-package classification after HKO and verification/algorithm-comparison. Next likely target: combinatorial-cells producers. | `/tmp/rust-tech-debt-map.md`, `experiments/MAP.md`, `tasks/reproducibility.md` |
| Verification trust chain | `[active]` | mainline thesis | retained claims | Decide which verification commands are cheap enough to require before broad Rust cleanup; keep path/row diagnostics in verification plumbing. | `research/verification.md`, `experiments/verification/` |
| `symplectic` API support levels | `[map-input]` | contingent during writing | Jörn for public API/architecture choices | Audit only the paths needed by retained thesis experiments before hiding, promoting, or redesigning public modules. | `crates/MAP.md`, `crates/symplectic/src/lib.rs` |
| Capacity result semantics | `[map-input]` | contingent during writing | retained claims, Jörn for thesis-facing contract | Decide whether root `ehz_capacity*` wrappers need stronger names/docs/results only after thesis usage is known. | `tasks/numerics.md`, `crates/symplectic/src/algorithms/orbit_search.rs` |
| Unsupported projected backend | `[map-input]` | contingent during writing | Jörn if the projected route is retained | Choose hide, complete, or explicitly-document-unsupported only if normal callers still see it during retained work. | `tasks/numerics.md`, `crates/symplectic/src/algorithms/orbit_search.rs` |
| Hidden hard failures in fallible APIs | `[active]` | map input | agents | Build minimal reproducers before changing behavior; prioritize panics/nontermination on public `Result` surfaces. | `/tmp/rust-tech-debt-map.md`, `crates/symplectic/src/lib.rs`, `crates/symplectic/src/geom/polytope.rs`, `crates/symplectic/src/random.rs` |
| `algebraic-numbers` proof/API map | `[active]` | mainline thesis if exact validation is cited | agents, Jörn for math/proof acceptance | Review public API, serialization contract, and formal-citation TODOs; route proof gaps to formal/research surfaces instead of hiding them in code TODOs. | `crates/algebraic-numbers/`, `crates/MAP.md` |
| Duplicate producer ownership | `[map-input]` | map input | agents, Jörn only for deleting provenance | Label current, historical, frozen-baseline, exploratory, or delete only after checking research/task truth for that package. | `experiments/sys-landscape/`, `experiments/numerics/`, `experiments/verification/algorithm-comparison/` |
| Blocked/stale/provenance code that looks live | `[active]` | map input | agents | Fix sampled stale headers and add grep-able status markers where source truth is already clear; avoid broad deletion without provenance review. | `/tmp/rust-tech-debt-map.md`, topic research notes |
| Local diagnostic text | `[active]` | map input | agents | Improve path/row/error context opportunistically while touching nearby experiment or verification code. | `/tmp/rust-tech-debt-map.md` |
| Large mixed-purpose files | `[future]` | future/follow-up by default | architecture decision if reopened | Split only when a concrete retained task is blocked by the mixed purpose. | `/tmp/rust-tech-debt-map.md` |

## Agent Cache

- [fresh 2026-05-04] `/tmp/rust-tech-debt-map.md` is an exploration report, not
  tracked truth and not a cleanup plan. Recheck cited files before editing.
  It is useful for cluster names, evidence anchors, and binary inventory.
- [fresh 2026-05-04] `cargo clippy --workspace --all-targets -- -D warnings`
  passed on branch `rust-tech-debt-cleanup` after mechanical lint fixes in
  Rust tests, benches, and one numerics-gradient doc comment. Refresh by
  rerunning the command before stacking larger cleanup.
- [fresh 2026-05-04] Verification target-pool shared-cache paths are optional
  candidate inputs: missing paths load as empty through `database::load`.
  Conflicts and parse errors still fail loudly. `target_pool.rs` and `io.rs`
  now include path/row context for the local failure points touched in this
  branch.
- [fresh 2026-05-04] `hko-facet-splitting` now has `--help` and `--smoke`.
  Full mode still writes `facet-splitting/hko-neighborhood-splitting.jsonl`;
  smoke mode writes the separate
  `facet-splitting/hko-neighborhood-splitting-smoke.jsonl`.
- [fresh 2026-05-04] `experiments/hko-local-maximum/README.md` now records the
  HKO Rust command contract in one place. It distinguishes smoke/default/full
  and canonical output modes for all eight HKO binaries.
- [fresh 2026-05-04] `hko-lagrangian-probe` now rejects unknown arguments and
  supports `--help`; its `--smoke` mode still writes
  `lagrangian-boundary/lagrangian-probe-smoke.jsonl`.
- [fresh 2026-05-04]
  `experiments/verification/algorithm-comparison/README.md` records command
  safety for `cmp-ablation`, `cmp-benchmark`, and `cmp-benchmark-profile`.
  `cmp-ablation --smoke` and `cmp-benchmark --smoke` write separate smoke
  JSONL files; full mode keeps the tracked evidence paths.
- [fresh 2026-05-04] `experiments/crosspolytope/main/main.rs` no longer says
  it fills a placeholder capacity. The current source truth is
  `research/crosspolytope.md`: capacity `4.0` is recorded, with explicit
  caveat that search is complete only through `m = 13`.

## Pruned / Stale

- None yet. Add entries here when a tempting cleanup route is rejected after
  source-grounded review, so future agents do not rediscover it.
