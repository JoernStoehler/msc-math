# Archaeology: agent rules

This directory contains files recovered from `msc-viterbo`, an abandoned predecessor repo. Everything here is **untrusted**.

## Do not

- **Do not trust** any claim, value, formula, proof, test assertion, or status label in these files. This includes capacity values, algorithm correctness claims, "verified" or "tested" labels, and mathematical derivations. Treat every statement as unverified, regardless of how confident the text sounds.
- **Do not adopt** naming conventions, coordinate ordering, normalization choices, or type designs from these files. Current conventions are in `crates/CLAUDE.md` and override anything here.
- **Do not edit** files in `raw/`. They are primary sources preserved verbatim.
- **Do not use as a starting point** to copy-paste or modify. Write fresh code and proofs instead.
- **Do not load into context** unless you have a specific reason (e.g., directed to by Jörn or an issue, or looking for a known pitfall). These files are large and will waste context window on unverified content.

## Do

- **Read for ideas**: algorithm approaches that were tried, data structures that were considered, test cases that were proposed.
- **Read for warnings**: what went wrong, which approaches failed, which formulas were buggy. Bug reports and dead ends (`findings-*.md`, `ARCHAEOLOGY.md` "Known bugs" section) are the highest-value files here.
- **Independently verify** anything you take from here. If a file says "tesseract capacity = 4.0 (HK2017 Example 4.6)", verify against the actual paper, not this file.

## Context

- Files were written by AI agents with varying levels of Jörn's review. Some had significant discussion behind them; others are pure unreviewed agent output. There is no way to distinguish which is which.
- "Status" labels inside files (e.g., "implemented and tested", "verified", "proven correct") are old agent self-descriptions, not verified ground truth.
- The old codebase had known bugs that persisted undetected through agent-written tests: the HK2019 QP solver silently returned wrong values, the trivialization formula was wrong, orbit validation missed segments. These bugs looked correct on a skim.

## Known-broken items

For reference, these specific items are known to be wrong in the old repo:

1. **HK2019 QP solver** — misses optima on 2D+ faces of the feasible set, returns plausible but wrong values
2. **Trivialization formula** — `tau_n(V) = (<V,Jn>, <V,Kn>)` is not a bijection on 2-face tangent spaces; was later fixed
3. **Billiard orbit validation** — only checked even-indexed segments, missed bounce transitions; pentagon returned 2.127 instead of 3.441
4. **Triangle x triangle discrepancy** — billiard returns 3.0, HK2017 returns 1.5; unresolved at time of archival
5. **Normalization convention mismatch** — some files use `sys = c^2/(2*vol)`, others use `sys = c^2/(4*vol)`

## Structure

- `raw/docs/` — 51 recovered documentation files (specs, thesis drafts, proofs, bug reports, literature summaries)
- `raw/code/` — 12 recovered Rust source files (algorithm implementations, flattened from three subdirectories)
- `raw/tests/` — 23 recovered Rust test files
- `raw/ARCHAEOLOGY.md` — index from the source branch with tables, provenance info, and known bugs
- `INDEX.md` — per-file metadata: type tag, origin, one-line description
