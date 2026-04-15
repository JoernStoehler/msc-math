# AGENTS.md

## Project Goal

Master thesis by Jörn Stöhler, University of Augsburg.
Advisor: Kai Cieliebak. Second advisor: Elizabeth Gaar.
Deadline: End of April 2026.
Topic: Probing Viterbo's Conjecture.

Planned deliverables:
1. A printed-quality LaTeX thesis: `thesis/build/main.pdf`
2. A high-performance Rust library for symplectic geometry on polytopes: `library/`
3. A reproducible experiment pipeline: `experiments/`

## Current Layout

- `library/`: Rust library crate `symplectic`, with code in `library/src/`, tests in `library/tests/`, and benches in `library/benches/`.
- `formal/`: Developer-facing mathematical sources for the library and experiments.
  - `formal/library/*.tex`: library module mathematics.
  - `formal/<topic>/*.tex`: experiment and research mathematics by topic.
  - `formal/main.tex`: full formal build.
- `experiments/`: Rust/Python experiment packages grouped by research topic.
  - `experiments/<topic>/Cargo.toml`: package manifest and binary registrations.
  - `experiments/<topic>/<experiment>/main.rs`: experiment binary entrypoint.
  - `experiments/<topic>/<experiment>/analyze.py`: analysis and figure generation when present.
  - Data and figures live next to the experiment that produced them.
- `research/`: Design notes, method selection, and experiment plans.
- `thesis/`: Publishable thesis sources. The thesis is self-contained and does not `\input` files from `formal/`, `experiments/`, or `library/`.
- `papers/<abbreviationYear>/`: Downloaded arXiv paper sources.
- `RESULTS.md`: Thesis content plan and project findings.
- `TASKS.md`: Unified project tracker. Run `bash scripts/tasks-toc.sh` for section line ranges.
- `scratch/`: Undocumented scratch notes, migration notes, and temporary working material. Do not treat it as current convention text.
- `scripts/`: Repo helper scripts that are not tied to one runtime environment.
- `.devcontainer/`: Local devcontainer and Codex web environment documentation.
- `.agents/skills/`: Codex skills. Detailed conventions and workflows live here.
- `.codex/agents/`: Codex subagent definitions.
- `.codex/worktrees/`: Repo-local worktrees for isolated Codex sessions.

## Current Instruction Sources

Required project instructions live in this root map or in discoverable skills. Do not add nested `AGENTS.md` files as required instruction maps; root-launched Codex sessions will not reliably load them.

## General Conventions

- **File headers:** Every source file starts with a comment block stating purpose and context. Module-level files also state the module architecture.
- **Self-contained thesis:** Thesis sources copy or own their publication assets. Experiment code must not make thesis correctness depend on links into `experiments/`, `formal/`, or `library/`.
- **Feature lifecycle:** New code starts in the relevant `experiments/` subtree when it is still exploratory. Stable, approved algorithms migrate into `library/`. Validation experiments either become library tests or remain in `experiments/`.
- **Math-code correspondence:** Non-trivial Rust algorithms must cross-reference formal mathematics with labels such as `[lem:label]`, `[thm:label]`, or `[def:label]`. The matching `\label{...}` lives in `formal/library/*.tex` or the relevant `formal/<topic>/*.tex` file.
- **Jörn's time:** Spend agent time on exploration, verification, and local review before asking Jörn. Ask Jörn only for mathematical judgment, thesis-scope decisions, advisor-facing framing, taste, or external-world actions.
- **Define the check first:** Before acting, decide what result would prove the task is done. Tool success is not task success.
- **No status-only handoff:** Before replying, do the next useful step, ask one Jörn-only question, or report a concrete blocker.

## Git Conventions

- Use local `main` as the base, not `origin/main`.
- Before merging to `main`, run the `pre-merge` skill and get explicit approval from Jörn.
- Agents may commit without asking. Ask about merge approval, not commit permission.
- Destructive operations such as force-push, branch deletion on `main`, `git reset --hard`, and checkout-based reverts require explicit approval.
- Git LFS tracks `.jsonl` files through `.gitattributes`. `git add`, `commit`, and `push` work normally. A pre-commit hook blocks non-LFS files larger than 10 MB.

## Worktrees

- Default: stay in the current checkout.
- Create a worktree when the task asks for isolated edits or when parallel sessions will edit overlapping tracked files.
- Use local `main` unless Jörn names a different base:
  `git worktree add -b <branch> .codex/worktrees/<branch> main`
- Subagents stay in their existing checkout unless their prompt names a worktree path and branch.
- After merge, remove the worktree with `git worktree remove .codex/worktrees/<branch>` and delete the branch with `git branch -d <branch>`.

## Planning and Verification

- For tasks with more than one concrete change or one verification step, keep a plan with objective, dependency, owner, and verification command or review check.
- Include a quality gate in the plan. Use subagent review when Jörn asks for delegation or the active session instructions allow it; otherwise run a local review against the same checklist.
- Update the plan after meaningful results. Do not leave stale statuses.
- Before asking Jörn to review a draft, proof sketch, experiment write-up, or conclusion, first run the checks that agents can run: buildability, internal consistency, source attribution, figure/text alignment, claim/data alignment, label/cross-reference resolution, missing tests, and scope drift.

## JSONL / LFS Safety

- `.jsonl` files are generated artifacts and are LFS-tracked. Do not edit `.jsonl` with patch-style line edits.
- For smoke or warmup runs, write temporary datasets under an untracked temp directory and delete them after the run.
- If a script touches tracked outputs only for compatibility, restore those paths before finishing.
- If a tracked `.jsonl` changes unexpectedly, stop and report the exact file and command.

## Environment

Supported environments:
- Local devcontainer at `/workspaces/msc-math`: full baseline environment with Rust, Python, TeX Live, and `gh`. See `.devcontainer/README.md`.
- Codex web environment: lower-complexity environment for web sessions. See `.devcontainer/codex-cloud.md`; TeX is intentionally out of scope there.

## Quick Commands

```bash
# Rust library
cargo test -p symplectic --release --lib
cargo clippy -p symplectic --lib -- -D warnings
cargo test -p symplectic --release -- --ignored

# Rust workspace and experiments
cargo build --workspace --release
cargo build -p exp-<topic> --release

# Thesis
cd thesis/ && latexmk && ./check-build.sh

# Formal math
cd formal/ && latexmk
cd formal/library/ && latexmk
```

## TASKS.md

- `##` sections group by theme.
- `###` items are individual work units.
- Every `##` and `###` header has a status tag: `[done]`, `[active]`, `[blocked]`, `[open]`, `[Jörn]`, or `[future]`.
- `[done]` items include a date: `### [done] [2026-04-15] Item title`.
- `[active]` means exactly one session owns the whole `###` task: the header and its intent, not a literal sub-list of body bullets.
- Headers carry the key information. Bodies use bullets for decisions, reasons, blockers, or links.
- Link to logbooks, formal files, or result docs instead of duplicating findings.

## Text For Agents

Optimize files, comments, and prompts that agents read for these properties, in order:

1. **Correct, corrigible:** Verify claims against code or data. Cite sources or commands when a future agent needs to check the claim.
2. **Observable, measurable:** State checks the reader can run.
3. **Unambiguous:** Each sentence should have one reading.
4. **Complete:** Include assumptions, preconditions, and the reason behind non-obvious decisions.
5. **Actionable:** The reader should know what to do next.
6. **Simple and concrete:** Prefer familiar patterns, examples, and literal terms.

Vague-word ban: do not use "appropriate", "properly", "ensure", "good", "consider", "reasonable", "necessary", "efficient", or "robust" without saying what observable condition the word means.
