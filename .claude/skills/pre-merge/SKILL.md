---
name: pre-merge
description: Checklist before presenting work for merge to main. Load when finishing a task and preparing to report to Jörn.
---

# Pre-Merge Checklist

Run through before telling Jörn work is ready.

## Build and test

```bash
cd crates/ && cargo test --release --lib
cd crates/ && cargo clippy --lib -- -D warnings
cargo build -p exp-<group> --release   # or: cargo build --workspace --release
cd thesis/ && latexmk && ./check-build.sh
```

All must pass. If thesis doesn't compile, fix before presenting — Jörn reviews the PDF.

## Data freshness

For experiments with committed data, check whether code changed more recently than data:

```bash
git log -1 --format='%H %ci' -- crates/exp-<group>/<subdir>/run.rs
git log -1 --format='%H %ci' -- crates/exp-<group>/<subdir>/*.jsonl
```

If code is newer, regenerate data on this branch.

## Content checks

- [ ] All new factual claims verified against evidence (core rule)
- [ ] New math.tex content has proofs (no statement-only stubs)
- [ ] New thesis math wrapped in `\begin{unverified}...\end{unverified}`
- [ ] Cross-references resolve (`thesis/build/main.aux`)
- [ ] Logbook entries cite sources inline for all numbers

## Update TASKS.md

- Mark completed tasks as done (move to Completed section with date and one-line summary)
- Update status/next-steps for tasks affected by this work
- Add newly discovered tasks if any

## What to report to Jörn

- What changed (files, scope)
- What's verified vs what needs Jörn's review
- Any unresolved TODOs or GAPs introduced
- If work is incomplete: write a handoff file to `handoffs/<name>.md` with context, scope, key files, prior findings, and success criteria