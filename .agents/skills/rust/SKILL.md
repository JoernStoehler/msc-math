---
name: rust
description: Use when Codex writes, edits, reviews, or delegates Rust work in this repo, especially crates, Rust experiments, API targets, tests, benches, numerical code, or Rust comments that claim mathematical meaning.
---

# Rust Conventions

This skill owns Rust code, API, testing, review, and handoff conventions.

## Instrumental Objectives
- Rust should help the thesis succeed: mathematically faithful, contract-explicit, verifiable, and maintainable by GPT-5.5.
- GPT-5.5 is the operational reader and writer. Do not write `.rs` files to teach Kai, Python programmers, Rust beginners, pre-GPT-5.5 agents, or unusually expert Rust-core developers.
- Code is read more often than written, coding is cheap, and performance matters only in profiled hotspots. Prefer local clarity and concrete variants over byte-saving, speculation, or premature optimization.

## Conventions
- Prefer simple functional Rust: simple control flow, simple data types, simple signatures, and standard crate patterns such as `Vector4<Scalar>`, `&[Vector4<Scalar>]`, and `Vec<Vec<usize>>`.
- Do not use wrappers, aliases, traits, or smart constructors that worsen readability. Add them only when they remove real complexity or enforce a context-free invariant.
- Shape APIs around mathematical operations and experiment workflows. Keep exact, f64, experiment, and helper surfaces separate when contracts differ; duplicate specialized flows when clearer than a shared abstraction.
- Put context-dependent propositions on producer/consumer function contracts, not on data containers pretending to prove them.
- Public math/numerics APIs state input/output contracts. Classify important conditions as validated here, assumed after a named validation boundary, valid mathematical non-success, or theorem-backed output guarantee.
- Use explicit result/outcome enums when callers must distinguish mathematical outcomes; avoid ambiguous `Option`.
- f64 APIs state approximation, error-bound, indeterminate-result, and heuristic-guess semantics explicitly.
- Cite formal labels, proof notes, or API targets where math/code correspondence is not obvious. Include reasoning traces and invariants when they improve verification.
- Put unresolved API decisions in the relevant API target or task file.

## Suggestions
- Default to specific orchestrator functions. Use strategy/configuration enums when callers, experiments, provenance, serialization, or reviewability need a named strategy value.
- For reusable Rust crates and substantial crate subtrees, prefer a consumer `README.md` and maintainer `DEVELOPMENT.md` once API or architecture decisions are nontrivial.
- `README.md` should let a caller perform ordinary use without opening `src/`.
- `DEVELOPMENT.md` should help a maintainer change internals without reconstructing current scope, API rationale, edit map, rejected/deferred approaches, and verification witnesses from code/tests/history.

## Suggested Workflows
- Spike subagents/worktrees: for uncertain Rust/API shapes, try small concrete variants before theorizing. Prefer parallel subagent or worktree spikes when alternatives are cheap and independent. Give subagents a bounded objective, named files or ownership, and the expected output. Record the objective and comparison before promoting the result.
- API proposals: keep unapproved proposals as reviewable diffs or `/tmp` reports. In isolated branches, commits are useful history, not API approval.
- Tests and refactors: add fast crate tests, `clippy`, and smoke checks for public behavior, numerical helpers, regressions, and router/classifier decisions. Keep slow reusable-crate correctness tests in crate tests; use experiments for slow experiment-workflow suites. During risky refactors, keep old/new behavior comparable until tests and review justify removing the old path. Mark intentionally red or ignored tests with the reason and removal condition.
- Public API review: when a Rust change adds or changes exported crate API, public rustdoc/README API docs, or checked examples used as API guidance, review the changed public surface for KISS/YAGNI before reporting success. Ground the review in the diff, current call sites/tests, and any API target or task file naming the intended caller. Prefer removing wrappers, aliases, recoverable error variants for programmer bugs, one-sided conveniences, avoidable example clones/unwraps, and abstractions without a current caller.
- Crate docs review: for Rust crate `README.md` or `DEVELOPMENT.md` review, use `references/review-crate-readme.md` and `references/review-crate-development.md` as reusable critic prompts; pass the crate path and checked surfaces.
- Review and handoff: name exact review surfaces, required cwd, package/crate, files changed, source-of-truth files or labels, contracts touched, verification run, unchecked risks, and decisions reserved for Jörn.
