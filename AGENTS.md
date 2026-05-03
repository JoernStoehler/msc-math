# AGENTS.md

This file is the always-loaded repo map. It should help agents find the right
surface quickly and carry only context that is broadly useful across tasks.
Detailed workflows and conventions live in skills, topic maps, and task files.

## Project Goal

Master thesis by Jörn Stöhler, University of Augsburg.
Advisor: Kai Cieliebak. Second advisor: Elizabeth Gaar.
Deadline: End of April 2026.
Topic: Probing Viterbo's Conjecture.

Planned deliverables:
1. A printed-quality LaTeX thesis: `thesis/build/main.pdf`
2. Durable Rust crates for symplectic geometry and exact arithmetic: `crates/`
3. A reproducible experiment pipeline: `experiments/`

## Domain Map

- `thesis/`: Publishable, self-contained thesis sources. The thesis owns or
  copies its publication assets and does not `\input` files from `formal/`,
  `experiments/`, or `crates/`.
- `crates/`: Durable Rust crates.
  - `crates/symplectic/`: Symplectic geometry crate, with source in
    `crates/symplectic/src/` and benches in `crates/symplectic/benches/`.
  - `crates/algebraic-numbers/`: Exact ordered algebraic scalar crate, with
    source in `crates/algebraic-numbers/src/`, benches in
    `crates/algebraic-numbers/benches/`, and smoke/property tests in
    `crates/algebraic-numbers/tests/`.
- `formal/`: Developer-facing mathematical sources for crates and experiments.
  - `formal/library/*.tex`: Reusable crate mathematics.
  - `formal/<topic>/*.tex`: Experiment and topic mathematics.
  - `formal/main.tex`: Full formal build.
- `experiments/`: Rust/Python experiment packages grouped by research topic.
  - `experiments/<topic>/Cargo.toml`: Package manifest and binary registrations.
  - `experiments/<topic>/<experiment>/main.rs`: Experiment binary entrypoint.
  - `experiments/<topic>/<experiment>/analyze.py`: Analysis and figure
    generation when present.
  - Data and figures live next to the experiment that produced them.
  - Durable Sage validation lives under `experiments/verification/sage/` when it
    stops being topic-local.
- `research/`: Research-facing notes, interpreted analysis, decision history,
  proof-route state, and topic summaries.
- `contracts/`: Canonical algorithm correspondence and verification contracts.
- `papers/<abbreviationYear>/`: Downloaded arXiv paper sources.

## Domain Navigation

Read these maps when their surface matches the task:

| Surface | Read when |
| --- | --- |
| `ROADMAP.md` | orienting on thesis closeout streams, current phase, or where a task belongs |
| `tasks/README.md` | editing `tasks/*.md` or interpreting task-bundle status/cache conventions |
| `research/INDEX.md` | looking for interpretation notes, proof-route state, or research-result caches |
| `crates/MAP.md` | navigating durable crate boundaries, API tiers, and core entities |
| `experiments/MAP.md` | navigating experiment topic packages, helper crates, data patterns, and provenance |
| `tasks/verify-thesis-done.md` | checking the once-run final thesis-done gate |
| `thesis/submission/README.md` | checking university forms, submission mechanics, or preservation actions |

Subtree `MAP.md` files are descriptive navigation caches, not always-loaded
instruction surfaces. Do not add nested `AGENTS.md` files as required
instruction maps; root-launched Codex sessions will not reliably load them.

## Domain Conventions

- **File headers:** Module-level source files start with a short purpose/context
  comment block. Small leaf files may rely on module docs and clear names.
  Detailed language-specific header rules live in the relevant convention
  skills.
- **Feature lifecycle:** New exploratory code starts in the relevant
  `experiments/` subtree. Stable, approved algorithms migrate into `crates/`.
  Validation experiments either become crate tests or remain in `experiments/`.
- **Test/validation boundary:** Crate tests are fast live checks for developer
  feedback and ordinary regressions. Slow mathematical validation, edge-case
  searches, broad random sweeps, and generated evidence datasets live in
  `experiments/`.
- **Math-code correspondence:** Rust code cross-references formal mathematics
  when correctness depends on a formal result. Use labels such as `[lem:label]`,
  `[thm:label]`, or `[def:label]`; pure orchestration does not need a label. The
  matching `\label{...}` lives in `formal/*.tex`.
- **Experiment paths:** Use semantic experiment paths. Do not force balanced
  subtrees when the semantics are asymmetric.
- **Research notes:** Put research-state notes, interpreted analysis, decision
  history, and next-step planning in `research/`. Keep only execution-facing
  packet docs under `experiments/`.
- **Data ownership:** Keep generated data with the producer that writes it.
  Avoid multiple binaries writing to the same tracked output.
- **Cross-file references:** Comments and notes should reference neighboring
  surfaces explicitly, e.g. `<file>.tex:\ref{label}`, `<file>.rs:symbol`, or
  `<file>.sage:symbol`.
- **JSONL / LFS safety:** `.jsonl` files are generated artifacts tracked by Git
  LFS. Trace figure, table, dataset, and experiment-artifact provenance with
  targeted `rg` and local source inspection. There is no repo-wide generated
  dataflow map; rebuild one only if repeated provenance work proves it is worth
  maintaining.

## Harness Map

- `.agents/skills/`: Discoverable skills. Use the skill whose name and
  description match the task; detailed conventions and validation live there.
- `.agents/skills/verification/`: Repeatable quality, claim-support,
  repo-promise, code, data, and figure passes.
- `.codex/agents/`: Repo-local subagent definitions.
- `.codex/reference/`: Durable agent-facing reference notes, prompt packets, and
  repo-maintainability design material.
- `.codex/worktrees/`: Repo-local worktrees for isolated Codex sessions.
- `.devcontainer/`: Local devcontainer and Codex web environment documentation.
- `scratch/`: Undocumented scratch notes, migration notes, and temporary working
  material. Do not treat it as current convention text.
- `scripts/`: Repo helper scripts that are not tied to one runtime environment.

## Session Rules

- Work only in the assigned cwd. Treat the tool default cwd as untrusted until it
  matches the assigned cwd.
- Use `/workspaces/msc-math` on `main` only when the task deliberately targets
  the root checkout or Jörn explicitly grants main-checkout work.
- Spend agent time on exploration, verification, and local review before asking
  Jörn. Ask Jörn only for mathematical judgment, thesis-scope decisions,
  advisor-facing framing, taste, or external-world actions.
- Before acting, decide what result would prove the task is done. Tool success
  is not task success.
- Before replying, do the next useful step, ask one Jörn-only question, or
  report a concrete blocker. Do not hand off status only.
- Remove generated scratch/build artifacts that are clearly from the current
  agent's command and not intended deliverables. Do not remove files whose
  origin or purpose is ambiguous; leave unrelated untracked or dirty work alone.

## Worktrees And Git

- Use local `main` as the base, not `origin/main`.
- Create a worktree when the task asks for isolated edits or when parallel
  sessions will edit overlapping tracked files.
- Use local `main` unless Jörn names a different base:
  `git worktree add -b <branch> .codex/worktrees/<branch> main`
- Every subagent prompt names the required cwd. `spawn_agent` cannot set cwd;
  subagents must anchor commands and edits from their own tools.
- Before merging to `main`, run the `pre-merge` skill and get explicit approval
  from Jörn.
- Agents may commit without asking. Ask about merge approval, not commit
  permission.
- After merge, remove the worktree with
  `git worktree remove .codex/worktrees/<branch>` and delete the branch with
  `git branch -d <branch>`.
- Destructive operations such as force-push, branch deletion on `main`,
  `git reset --hard`, and checkout-based reverts require explicit approval.
- Git LFS tracks `.jsonl` files through `.gitattributes`. `git add`, `commit`,
  and `push` work normally. A pre-commit hook blocks non-LFS files larger than
  10 MB.

## Planning And Verification

- For tasks with more than one concrete change or one verification step, keep a
  plan with objective, dependency, owner, and verification command or review
  check.
- Include a quality gate in the plan. Use subagent review when Jörn asks for
  delegation or the active session instructions allow it; otherwise run a local
  review against the same checklist.
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
- Before asking Jörn to review a draft, proof sketch, experiment write-up, or
  conclusion, first run the checks that agents can run: buildability, internal
  consistency, source attribution, figure/text alignment, claim/data alignment,
  label/cross-reference resolution, missing tests, and scope drift.

## Environment

Supported environments:
- Local devcontainer at `/workspaces/msc-math`: full baseline environment with
  Rust, Python, TeX Live, and `gh`. See `.devcontainer/README.md`.
- Codex web environment: lower-complexity environment for web sessions. See
  `.devcontainer/codex-cloud.md`; TeX is intentionally out of scope there.

Quick commands:

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

## Text For Agents

Optimize files, comments, and prompts that agents read for these properties, in
order:

1. **Correct, corrigible:** Verify claims against code or data. Cite sources or
   commands when a future agent needs to check the claim.
2. **Observable, measurable:** State checks the reader can run.
3. **Unambiguous:** Each sentence should have one reading.
4. **Complete:** Include assumptions, preconditions, and the reason behind
   non-obvious decisions.
5. **Actionable:** The reader should know what to do next.
6. **Simple and concrete:** Prefer familiar patterns, examples, and literal
   terms.

Vague-phrase check: words such as "appropriate", "properly", "ensure", "good",
"consider", "reasonable", "necessary", "efficient", and "robust" often hide
missing criteria. Treat them as search triggers, not banned tokens. Rewrite only
when the phrase has multiple plausible readings that would change future agent
behavior. Preserve precise project terms when replacing the word would change
the meaning, and state the observable condition when the word is a task
criterion.
