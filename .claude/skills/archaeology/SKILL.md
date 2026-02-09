---
name: archaeology
description: Use when reading or working with files in archaeology/. Contains trust rules, known-broken items, and what to read vs. avoid.
---

# Archaeology directory rules

The `archaeology/` directory contains files recovered from `msc-viterbo`, an abandoned predecessor repo. **Everything here is untrusted.**

- **Do not trust** any claim, value, formula, proof, test assertion, or status label. Treat every statement as unverified.
- **Do not adopt** naming conventions, coordinate ordering, or type designs. Current conventions override.
- **Do not edit** files in `raw/`. They are primary sources preserved verbatim.
- **Do not use as a starting point** to copy-paste or modify. Write fresh code and proofs.
- **Do not load into context** without a specific reason. These files are large and will waste context on unverified content.
- **Read for ideas**: approaches tried, data structures considered, test cases proposed.
- **Read for warnings**: what went wrong, which formulas were buggy. Bug reports (`findings-*.md`, `ARCHAEOLOGY.md`) are the highest-value files.
- **Independently verify** anything you take from here against the actual papers.

The old codebase had known bugs that persisted undetected through agent-written tests: the QP solver silently returned wrong values, the trivialization formula was wrong, orbit validation missed segments. These bugs looked correct on a skim. This is why Jörn's domain input on what to test matters.

## Known-broken items

1. **HK2019 QP solver** — misses optima on 2D+ faces, returns plausible but wrong values
2. **Trivialization formula** — `tau_n(V) = (<V,Jn>, <V,Kn>)` not a bijection on 2-face tangent spaces
3. **Billiard orbit validation** — only checked even-indexed segments; pentagon returned 2.127 instead of 3.441
4. **Triangle × triangle discrepancy** — billiard returns 3.0, HK2017 returns 1.5; unresolved
5. **Normalization convention mismatch** — some files use `sys = c^2/(2*vol)`, others `sys = c^2/(4*vol)`
