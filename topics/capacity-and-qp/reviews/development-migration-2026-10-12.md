# QP development package relocation review

12 October 2026. Integrating owner: `/root/migrate/qp_area`. Independent
read-only reviewer: `/root/migrate/qp_area/api_review`.

The complete 146-file development/evidence package moved from
`experiments/dev-quadratic-program/` to
`topics/capacity-and-qp/development/`. Its three Cargo package names, internal
module layout, selected-route controls, producer-specific audit/performance
packets and fixtures were retained together. Production `symplectic::kkt` and
`capacity_4d` remain conventional source modules of the reusable crate; the
[topic account](../README.md) states this explicit scientific/package boundary
and the deferred physical production-source relocation.

## Source and consumer changes

The directory move itself preserved all file bytes. Subsequent corrections:

- Root `Cargo.toml`: three workspace entries now name the topic package paths.
  The empirical owner separately updated its four entries from the current
  file after this write; its changes were preserved.
- Direct QP dependencies: `experiments/dev-gradient-ascent/Cargo.toml`, its
  `optimizer-runs/Cargo.toml`, `experiments/performance/Cargo.toml` and
  `experiments/qp-error-bounds/Cargo.toml` now reach the moved package.
- The moved main package reaches the empirical `sys-landscape` package at its
  canonical topic path, coordinated with the empirical owner. Its nested
  ablation package's internal `../..` dependency did not need changing.
- `src/artifact_cases.rs`: the manifest-relative repository ancestor depth
  changes from two to three, and its dataset directory follows the empirical
  producer move to `topics/empirical-studies/polytope-datasets`. File names,
  record parsing and selection behavior are unchanged.
- Two shell scripts and current Markdown navigation use canonical topic paths.
  `src/fallback_route/mod.rs` has only a path-comment correction.
  Historical cost-report artifact locations remain provenance, explicitly
  qualified. Retained JSON/JSONL bytes were not edited.

The [relocation receipt](development-relocation-2026-10-12.json) records the
retained data hashes and exact paths changed after the byte-preserving move.
External live consumer navigation and inventory references were handed to the
migration coordinator/shared-interface owner using the complete old-prefix to
new-prefix map. Frozen September proposal receipts remain source-specific
historical evidence, rather than instructions to apply patches to this package.

## Validation and independent verdict

The independent reviewer approved the limited code/data/path scope. It checked
all 146 baseline files remain present, retained JSON/JSONL hashes are unchanged,
Rust implementation changes are confined to root/dataset location, package
names/internal targets remain stable, the three workspace entries and named
Cargo dependencies resolve, Markdown links resolve, and updated script targets
exist. Its current-navigation corrections were applied; the stale introductory
“not moved” sentence was corrected to relocation without regeneration.

Integrator checks passed:

```sh
cargo check --all-targets \
  -p exp-dev-quadratic-program -p exp-qp-general-algorithms \
  -p exp-dev-qp-numerics-audit -p exp-dev-gradient-ascent \
  -p optimizer-runs -p exp-performance -p exp-qp-error-evidence
cargo test -p exp-dev-quadratic-program --release --test selected_route_correspondence
```

The focused compile completed successfully in 6.12 seconds. The correspondence
gate passed all four tests (47.04-second release rebuild, 1.24-second test
execution): general bounds, exact general action window versus the complete
control, retained F10 window derivatives, and exact product capacity/closure
winners. These fixed in-memory fixtures were inspected before execution; no
retained table, full producer or numerical campaign was run.

Focused rustfmt on the two changed Rust files, `bash -n` on the two changed
shell scripts, local Markdown-link checks and owned whitespace checks passed.
All 103 other Rust files retain their pre-move hashes; both retained JSON/JSONL
files match their pre-move hashes.

These checks establish source/build wiring and finite selected-route
correspondence. They do not provide new all-input mathematical or arithmetic
verification, recertify historical numerical populations, or establish complete
physical-orbit families. No package/API identity or numerical algorithm was
changed, and no new scientific result is claimed.
