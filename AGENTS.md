# AGENTS.md

## Project Goal

Master thesis by Jörn Stöhler, University of Augsburg.
Advisor: Kai Cieliebak. Second advisor: Elizabeth Gaar.
Deadline: End of April 2026.
Topic: Probing Viterbo's Conjecture.

Planned deliverables:
1. A printed-quality LaTeX thesis: `thesis/build/main.pdf`
2. Durable Rust crates for symplectic geometry and exact arithmetic: `crates/`
3. A reproducible experiment pipeline: `experiments/`

## Current Layout

- `crates/`: Durable Rust crates.
  - `crates/symplectic/`: symplectic geometry crate, with code in `crates/symplectic/src/` and benches in `crates/symplectic/benches/`.
  - `crates/algebraic-numbers/`: exact ordered algebraic scalar crate, with code in `crates/algebraic-numbers/src/`, benches in `crates/algebraic-numbers/benches/`, and smoke/property tests in `crates/algebraic-numbers/tests/`.
- `formal/`: Developer-facing mathematical sources for the crates and experiments.
  - `formal/library/*.tex`: reusable crate mathematics.
  - `formal/<topic>/*.tex`: experiment and topic mathematics.
  - `formal/main.tex`: full formal build.
- `experiments/`: Rust/Python experiment packages grouped by research topic.
  - `experiments/<topic>/Cargo.toml`: package manifest and binary registrations.
  - `experiments/<topic>/<experiment>/main.rs`: experiment binary entrypoint.
  - `experiments/<topic>/<experiment>/analyze.py`: analysis and figure generation when present.
  - Data and figures live next to the experiment that produced them.
  - Durable Sage validation lives under `experiments/verification/sage/` when it stops being topic-local.
- `research/`: Research-facing notes, interpreted analysis, decision history, and topic summaries.
- `contracts/`: Canonical algorithm correspondence and verification contracts.
- `thesis/`: Publishable thesis sources. The thesis is self-contained and does not `\input` files from `formal/`, `experiments/`, or `crates/`.
- `papers/<abbreviationYear>/`: Downloaded arXiv paper sources.
- `ROADMAP.md`: Agent-facing closeout overview and routing map for
  `tasks/*.md`.
- `tasks/`: Topic mini-roadmaps, cached task knowledge, and the compact
  once-run final thesis-done gate in `tasks/verify-thesis-done.md`.
- `crates/MAP.md`: Durable-crate navigation map: crate roles, subsystem
  boundaries, core entities, API tiers, and representation boundaries.
- `experiments/MAP.md`: Experiment navigation map: topic packages, helper
  crates, artifact patterns, and provenance search.
- `scratch/`: Undocumented scratch notes, migration notes, and temporary working material. Do not treat it as current convention text.
- `scripts/`: Repo helper scripts that are not tied to one runtime environment.
- `.devcontainer/`: Local devcontainer and Codex web environment documentation.
- `.agents/skills/`: Codex skills. Detailed conventions and workflows live here.
- `.codex/agents/`: Codex subagent definitions.
- `.codex/reference/`: Durable agent-facing reference notes, prompt packets, and repo-maintainability design material.
- `.codex/worktrees/`: Repo-local worktrees for isolated Codex sessions.

## Current Instruction Sources

Required project instructions live in this root map or in discoverable skills.
Subtree `MAP.md` files are descriptive navigation caches, not always-loaded
instruction surfaces. Do not add nested `AGENTS.md` files as required
instruction maps; root-launched Codex sessions will not reliably load them.

## Next Map Layer

Read these files when their purpose matches the task. They are the intended
one-hop maps after this always-loaded file.

| Surface | Read when |
| --- | --- |
| `ROADMAP.md` | orienting on thesis closeout streams, current phase, or where a task belongs |
| `tasks/README.md` | editing `tasks/*.md` or interpreting task-bundle status/cache conventions |
| `research/INDEX.md` | looking for interpretation notes, proof-route state, or research-result caches |
| `crates/MAP.md` | navigating durable crate boundaries, API tiers, and core entities |
| `experiments/MAP.md` | navigating experiment topic packages, helper crates, data patterns, and provenance |
| `tasks/verify-thesis-done.md` | checking the once-run final thesis-done gate |
| `.agents/skills/verification/` | running repeatable quality, claim-support, repo-promise, code, data, or figure passes |
| `thesis/submission/README.md` | checking university forms, submission mechanics, or preservation actions |

## General Conventions

- **File headers:** Module-level source files start with a short purpose/context comment block. Small leaf files may rely on module docs and clear names. Detailed language-specific header rules live in the relevant convention skills.
- **Self-contained thesis:** Thesis sources copy or own their publication assets. Experiment code must not make thesis correctness depend on links into `experiments/`, `formal/`, or `crates/`.
- **Feature lifecycle:** New code starts in the relevant `experiments/` subtree when it is still exploratory. Stable, approved algorithms migrate into `crates/`. Validation experiments either become crate tests or remain in `experiments/`.
- **Test/validation boundary:** Crate tests are fast live checks for developer feedback and ordinary regressions. Slow mathematical validation, edge-case searches, broad random sweeps, and generated evidence datasets live in `experiments/`.
- **Math-code correspondence:** Rust code cross-references formal mathematics when correctness depends on a formal result. Use labels such as `[lem:label]`, `[thm:label]`, or `[def:label]`; pure orchestration does not need a label. The matching `\label{...}` lives in `formal/*.tex`.
- **Experiment paths:** Use semantic experiment paths. Do not force balanced subtrees when the semantics are asymmetric.
- **Research notes:** Put research-state notes, interpreted analysis, decision history, and next-step planning in `research/`. Keep only execution-facing packet docs under `experiments/`.
- **Data ownership:** Keep generated data with the producer that writes it. Avoid multiple binaries writing to the same tracked output.
- **Cross-file references:** Comments and notes should reference neighboring surfaces explicitly, e.g. `<file>.tex:\ref{label}`, `<file>.rs:symbol`, or `<file>.sage:symbol`.
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

- Work only in the assigned cwd. Treat the tool default cwd as untrusted until it matches the assigned cwd.
- Use `/workspaces/msc-math` on `main` only when the task deliberately targets the root checkout or Jörn explicitly grants main-checkout work.
- Create a worktree when the task asks for isolated edits or when parallel sessions will edit overlapping tracked files.
- Use local `main` unless Jörn names a different base:
  `git worktree add -b <branch> .codex/worktrees/<branch> main`
- Every subagent prompt names the required cwd. `spawn_agent` cannot set cwd; subagents must anchor commands and edits from their own tools.
- After merge, remove the worktree with `git worktree remove .codex/worktrees/<branch>` and delete the branch with `git branch -d <branch>`.

## Planning and Verification

- For tasks with more than one concrete change or one verification step, keep a plan with objective, dependency, owner, and verification command or review check.
- Include a quality gate in the plan. Use subagent review when Jörn asks for delegation or the active session instructions allow it; otherwise run a local review against the same checklist.
- Route planning surfaces explicitly:
  `research/INDEX.md` and `research/*.md` = thesis story interpretation,
  proof-route state, and research caches,
  `tasks/verify-thesis-done.md` = once-run final thesis-done gate,
  `ROADMAP.md` = overview and routing surface,
  `tasks/*.md` = topic mini-roadmaps and cached task knowledge.
- Do not put repeated quality workflows, intermediate milestones, or
  `writer-ready` / `submission-ready` / `freeze-ready` acceptance detail into
  `tasks/verify-thesis-done.md`. Put reusable checks in the `verification`
  skill and topic-specific obligations in `tasks/*.md`.
- If an intermediate milestone needs durable multi-session acceptance criteria
  but is still not part of thesis-done, create a separate planning or milestone
  file instead of extending `tasks/verify-thesis-done.md` by default.
- Update the plan after meaningful results. Do not leave stale statuses.
- Before asking Jörn to review a draft, proof sketch, experiment write-up, or conclusion, first run the checks that agents can run: buildability, internal consistency, source attribution, figure/text alignment, claim/data alignment, label/cross-reference resolution, missing tests, and scope drift.

## JSONL / LFS Safety

- `.jsonl` files are generated artifacts tracked by Git LFS.
- Trace figure, table, dataset, and experiment-artifact provenance with
  targeted `rg` and local source inspection. There is no repo-wide generated
  dataflow map; rebuild one only if repeated provenance work proves it is worth
  maintaining.

## Environment

Supported environments:
- Local devcontainer at `/workspaces/msc-math`: full baseline environment with Rust, Python, TeX Live, and `gh`. See `.devcontainer/README.md`.
- Codex web environment: lower-complexity environment for web sessions. See `.devcontainer/codex-cloud.md`; TeX is intentionally out of scope there.

## Quick Commands

```bash
# Rust crates
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

## Task Roadmaps

- Start from `ROADMAP.md`, then open the relevant `tasks/*.md` bundle.
- Follow `tasks/README.md` when editing roadmap or task-bundle files.

## Text For Agents

Optimize files, comments, and prompts that agents read for these properties, in order:

1. **Correct, corrigible:** Verify claims against code or data. Cite sources or commands when a future agent needs to check the claim.
2. **Observable, measurable:** State checks the reader can run.
3. **Unambiguous:** Each sentence should have one reading.
4. **Complete:** Include assumptions, preconditions, and the reason behind non-obvious decisions.
5. **Actionable:** The reader should know what to do next.
6. **Simple and concrete:** Prefer familiar patterns, examples, and literal terms.

Vague-word ban: do not use "appropriate", "properly", "ensure", "good", "consider", "reasonable", "necessary", "efficient", or "robust" without saying what observable condition the word means.
