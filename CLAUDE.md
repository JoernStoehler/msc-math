# CLAUDE.md

Master Thesis: Probing Viterbo's Conjecture
Author: Jörn Stöhler, University of Augsburg
Advisor: Kai Cieliebak
Second advisor: Elizabeth Gaar
Timeline: Oct 2025 – mid-April 2026

## End state

A printed-quality LaTeX thesis (`thesis/build/main.pdf`), a high-performance Rust library for symplectic geometry on polytopes (`crates/`), and a reproducible experiment pipeline (`experiments/`).

## Mathematical Context

Viterbo's Conjecture (2000): For any convex body K in R^2n, the systolic ratio `sys(K) = c_EHZ(K)^2 / (2 vol(K))` is at most 1. Haim-Kislev and Ostrover (2024, Annals) disproved it in dimension 4 with an explicit 10-facet counterexample.

We follow HK2017, CH2021 to compute c_EHZ for polytopes in R^4, implement the algorithms in Rust with correctness verification, and probe the conjecture by computing sys across large polytope datasets.

## The Core Rule

Never write a factual claim without verifying it against evidence in the same session. "The code cross-checks X" requires reading the code. "The data shows Y" requires reading the data. When verification is impossible, mark with `% [TODO: JÖRN -` or `% [GAP -`.

**Citation verification:** Never produce author names or paper titles from memory. Verify against `thesis/bibliography.bib` or `papers/`. Agents confidently produce wrong names (e.g. "Cieliebak-Hutchings" instead of the correct "Chaidez-Hutchings").

**External systems (core rule instance):** When documenting external systems (LICCA cluster, university services, third-party tools), **link to the official documentation — do not paraphrase it.** Agent-written paraphrases of official docs are unverifiable, go stale silently, and future agents trust them over the real source. Reference files should contain only:
- Links to official documentation
- Facts personally verified in the current session (with date)
- Clearly marked TODOs for anything not yet verified

## Procedural layer is Jörn-gated

Do not create, modify, or delete skills, agents, hooks, or CLAUDE.md without Jörn's explicit approval. Propose changes in conversation; Jörn implements them. This is analogous to the math verification rule — agents can't reliably produce or quality-check procedural knowledge.

## Communication with Jörn

**Before each message, ask: does Jörn need to read this?** If no, don't send it. If yes, make it as short as possible.

| Situation | BAD | GOOD |
|-----------|-----|------|
| Task done | Wall of text summary | "Done. 12 files changed, tests pass." |
| Obvious subtask | "Should I also update X?" | Just do it. |
| Agent reports back | Dumping raw subagent output | "Review clean. 3 style fixes applied." |
| Need a decision | "What do you think?" | "X needs Y because Z. Doing it unless you object." |
| Own mistake | Self-flagellation | Fix it silently. |
| Jörn calls out mistake | Explaining why | "My mistake. Fixing now." |
| Told to STOP | Apologetic summary | (silence) |
| Status update | Session logistics | Research substance first |

**Interaction dynamics:**
- Read and respond to Jörn's messages BEFORE making tool calls
- Push back on contradictions and oversights — Jörn welcomes it
- Never take silence as confirmation
- Adopt Jörn's exact phrasing when he corrects nuance
- Questions must be self-contained — Jörn switches sessions and doesn't have TASKS.md memorized
- Number items so Jörn can respond "3 yes, 5 no"

**When receiving feedback:** Fix the instance, abstract the error class, scan for all instances, record durably in the relevant skill or CLAUDE.md.

## Session Workflow

Sessions work in git worktrees. Use full worktree paths in subagent prompts.

**Time economics:** Jörn's time is scarce; agent time is free. Parallelize via subagents and teams.

### scope → plan → implement → review → merge

**Scope** (Jörn + agent): Jörn scopes. Agents provide investigation findings.

**Plan → implement → review** (agent autonomous): No Jörn involvement unless specifically requested. End-of-turn messages recap context. Agents may return to earlier phases.

**Merge** (Jörn + agent): Agent reports what changed, what's verified, what needs Jörn. Only Jörn merges to `main`.

### Decision authority

| | Cheap to verify | Expensive to verify |
|---|---|---|
| **Easy rollback** | Act freely | Act, then Jörn verifies |
| **Hard rollback** | Discuss first | Discuss first |

Never without instruction: destructive operations, PRs, merging to `main`.

### Long sessions and compaction

Update the plan file as you work — it survives compaction, working memory does not. Write design decisions and their WHY into the plan (not just progress markers). After compaction, read the plan file to recover context. Never guess about pre-compaction events.

## Multi-Language Codebase

- **Rust** (crates/, experiments/): performance-critical and correctness-critical code
- **Python** (experiments/): plotting, data processing, orchestration
- **LaTeX** (thesis/, experiments/): thesis and math.tex files
- **Markdown**: agent-facing writeups, conventions, documentation
- **Json/Jsonl/Csv** (experiments/): datasets

## Skill Reference

Load skills on demand. Skills also serve as review specifications.

- `git-conventions` — local `main` (not `origin/main`), three-dot diffs, commit checklist
- `math-tex` — lemma statements and proofs colocated with code
- `tex-build` — build commands, PDF review workflow
- `tex-format` — .tex file structure, environments, figures
- `tex-content` — correctness, proofs, citations
- `rust-conventions` — coding style, math-code correspondence
- `rust-tests` — testing philosophy, fixtures, organization
- `experiment-conventions` — directory structure, pipeline
- `python-conventions` — script headers, figure sizing, visual quality
- `review` — how to run review subagents (mandatory before presenting to Jörn)
- `collaboration` — multi-agent coordination
- `session-handoff` — end-of-session persistence
- `data-pipeline` — expensive test data, LICCA cluster

## Environment

- Docker devcontainer at `/workspaces/msc-math`. OS-level isolation.
- Worktrees: `--worktree` flag or `EnterWorktree`. Branch from local `main`. Land at `.claude/worktrees/<name>/`.
- Pre-installed: Rust 1.93, Python 3.11 (pytest, ruff, mypy, black), gh CLI, TeX Live 2023
- `rm` is aliased to `trash-put`; use `/bin/rm` for real deletes
- **Runtime limit:** repeated commands must complete in ≤10 minutes (CPU monitor kills at 20min sustained)

## Quick Commands

```bash
# Rust
cd crates/ && cargo build
cd crates/ && cargo test --release --lib
cd crates/ && cargo clippy --lib -- -D warnings
timeout 5m cargo test --release
timeout 30m cargo test --release -- --ignored

# Python
ruff check experiments/
pytest experiments/

# LaTeX
cd thesis/ && latexmk
```

## Archaeology

`archaeology/` contains untrusted files from an abandoned predecessor repo. Don't use without specific reason.
