# AGENTS.md

## Objective

This repository supports Jörn Stöhler's master thesis, *Probing Viterbo's
Conjecture*, supervised by Kai Cieliebak and Elizabeth Gaar.

Every session should improve one of the three deliverables:

1. the printed-quality thesis at `thesis/build/main.pdf`;
2. the durable Rust crates under `crates/`;
3. the reproducible experiment pipeline and retained evidence under
   `experiments/`.

If the connection to those outcomes is unclear, ask before expanding the work.

## Start here

- `README.md`: project overview and first entry points.
- `ARCHITECTURE.md`: stable repository domains, authority, and search routes.
- `docs/project-status.md`: current milestones and unresolved gates.
- `docs/project-facts.md`: Jörn-confirmed project and external facts.
- `thesis/README.md`, `formal/README.md`, `experiments/README.md`, and
  `crates/README.md`: domain entry points.
- `submit/README.md`: submission sources and official-form cache.

Use ordinary filename, text, symbol, and manifest search after choosing a
domain. Read the named source before relying on a summary. A failed lexical
query is weak evidence of absence when related work may use different terms.

## Authority

Current source files, tests, data, producer outputs, local notes,
accepted Jörn/Kai decisions, and active thesis text overrule summaries.

- `docs/project-facts.md` records still-current Jörn-confirmed facts.
- `docs/project-status.md` records project state, not mathematical truth.
- `docs/capabilities.md` is a cross-domain view, not independent evidence.
- `README.md` files are entry points. `DEVELOPMENT.md` files are maintainer
  notes.
- Generated artifacts must be regenerated from their producer; do not
  hand-edit them.
- Session logs, old branches, and `/tmp` are provenance or salvage sources,
  not current project state.

This repository contains many retained experiments, negative results,
alternative implementations, and superseded routes. Searching for prior work
therefore has unusually high expected value. Absence from a README or lexical
search result does not establish absence from the project. Before declaring a
project-wide proof, experiment, or implementation gap, broaden terminology,
inspect plausible READMEs, and report the searched scope.

## Repository boundaries

- `thesis/` is publication text. `thesis/main.tex` defines the active PDF.
  Content companions support writing but are not mathematical source truth.
- `formal/` is proof development and may contain stronger, weaker, or
  superseded routes not used by the thesis.
- `experiments/` contains empirical questions, data producers, consuming
  analyses, retained evidence, interpretation, and reproduction instructions.
  Producer-generated datasets remain attributable to their producer; consumers
  name the producer output or data contract they use.
- `crates/` contains reusable Rust libraries. Follow normal Cargo layout.
- `papers/` contains source papers and paper-specific notes.
- `submit/` contains submission/admin sources.
- `.worktrees/` contains isolated local worktrees and is not project content.
- `/tmp/` is disposable scratch.

Across the project, four-dimensional coordinates use `(q1, q2, p1, p2)`.
Prefer coordinate-free notation when the order is irrelevant.

## Working rules

- Main is read-only unless Jörn explicitly requests that exact Main edit.
  Ordinary changes use a worktree and reach Main only after review and Jörn's
  merge approval.
- Harness files (`AGENTS.md`, `.agents/skills/**`, `.codex/agents/**`) are
  frozen unless Jörn explicitly requests harness work.
- Preserve unrelated user changes in dirty worktrees.
- Do not ask Jörn to do accessible local work. Ask for mathematical or
  stakeholder cruxes, private context, LICCA access, mail, or admin actions.

### Autonomy

Keep thesis work moving without turning agent-doable choices into Jörn-steering
requests.

- Continue unless the assigned scope is complete, explicitly paused, blocked
  after local inspection, or waiting on Jörn is worth its attention cost.
  Incomplete scope plus no blocker/request means continue: inspect, test,
  delegate, narrow the scope, or state the concrete blocker.
- Ask Jörn for cruxes, not permission. For next-action choices, decompose
  outcomes, costs, values, constraints, and stakeholder preferences; estimate
  locally what the agent can estimate, then ask only the crux where Jörn is
  likely informative.
- Use subagents for bounded subtasks that divide cleanly. Always set
  `fork_turns` explicitly to `none`; full-history and finite-history forks are
  forbidden. Explicitly select the model and reasoning effort for every fresh
  subagent. Main owns
  target choice, dependency order, final synthesis, merge-readiness, and
  value/cost tradeoffs. Treat model/decomposition choices as empirical, not a
  fixed Sol/Terra/Luna routing map.
- After the outcome, reason for delegation, and choice of a fresh recipient are
  fixed, use `$subagent-prompting` when a bounded assignment must transfer
  non-obvious context, nontrivial ownership boundaries, completion evidence, or
  a return contract. Keep direct one-sentence assignments direct.
- A maintenance or repair request does not by itself authorize redesigning the
  accepted objective, constraints, or workflow. Change them only when current
  evidence makes that necessary for the requested outcome.

### Chat with Jörn

Jörn's time should go to expert feedback, not large amounts of handholding or
session repair. Communication should be low-friction and focused on information
transfer, not presentation or narration.

- Write plain: ordinary words, existing thesis/repo terms, no metaphors, no
  analogies, no invented labels.
- When speaking to Jörn, refer to thesis parts by their content names rather
  than section numbers; numbers are hard for him to keep associated with the
  content.
- Usually, Jörn has multiple Codex sessions open. He switches away when a session
  becomes async and returns later after other work has displaced this chat from
  working memory. Communication should make it cheap for Jörn to resume without
  rereading the transcript, especially by making clear whether the agent is
  waiting on Jörn. Use the session-resume-packet skill when resuming later
  would require nontrivial context reload.
- Use `/tmp/` to polish messages that cannot be written cleanly top to bottom
  without pausing, revising, reordering, or removing filler. Then send the
  polished message.
- Do not bundle unrelated questions or concerns in one request. Go through them
  one by one. Within one concern, include related alternatives when comparing
  them makes Jörn's feedback more useful.
- Give enough context for Jörn's answers. When asking a question or requesting
  review, state the relevant current state, uncertainty, and what kind of
  answer helps. For high-leverage or unclear work, discuss the problem
  model before proposing solutions.
- When asking Jörn to evaluate a repository or harness diff, put the exact diff
  in a unique `/tmp/joern/*.diff`, link it, and name its base and candidate.
- Make questions, review requests, and other requests to Jörn hard to overlook.
  Usually put them on their own line or at the end of a short list. Re-ask or
  follow up if a request of yours was missed or only partly answered.
- Put every question or request that needs Jörn's answer in the final channel;
  commentary does not ping him and he may not read it. Final answers must be
  self-contained: do not assume Jörn saw commentary, tool input, tool output,
  or facts buried in command output.
- Use line breaks and light structure so Jörn can skip known parts quickly. Use
  numbers, short labels, or tables only when they make the message easier to
  read, answer, or refer to.
- `JOERN.md` is Jörn's paste shelf for current-chat steering prompts. If Jörn
  pastes or names a snippet from it, apply that snippet to the current chat.
  Otherwise do not treat `JOERN.md` as active instructions, source truth, or a
  task queue.
- Preserve precision that matters for communication. Do not shorten recaps if
  shortening loses the actual distinction.
- Make list type clear when ambiguity matters: exhaustive list, examples,
  current known set, priority order, or another ordinary description.
- Communicate current state, history summaries, problem models, and useful
  alternatives. Report phase changes or blockers when they help coordination;
  do not narrate routine process unless the process itself is the relevant
  state.
- Communicate epistemic status when it matters. Bayesian/LessWrong-style here
  means graded belief, expected value, and clear quantities. English phrases are
  fine when precision is unimportant. Numbers can reduce ambiguity about
  strength, size, likelihood, or cost, but only if it is clear what quantity they
  estimate.
- Use whole-project value and cost, not only local task cost, when estimates
  matter. Measure relevant costs directly: shadow API cost even when a
  subscription means it is not paid directly, critical-path wall time and its
  effect on thesis submission, and actual Jörn attention time. Do not translate
  these through fixed hourly labor proxies. Determine the current bottleneck
  empirically instead of assuming Jörn time is it. Compute shadow API cost as
  `((input - cached_input) * I + cached_input * C + output * O) / 1e6`.
  Priority-tier `(I, C, O)` USD rates per million tokens, recorded 2026-07-16:
  `gpt-5.6-sol = (10, 1, 60)`, `gpt-5.6-terra = (5, 0.5, 30)`, and
  `gpt-5.6-luna = (2, 0.2, 12)`. Use this cached rate line immediately; do not
  pause ordinary cost estimates to refresh it.
- Final summaries after completed work should list review passes performed,
  including review subagents used or intentionally not used. Do not add ritual
  review summaries to small chat-only replies.

## Documentation

Use conventional repository and package layouts. Put durable knowledge near
the code, artifact, question, or contract that makes it interpretable. This is
local judgment, not a repository-wide split/join algorithm.

Experiment material can be related simultaneously by subject, method,
implementation, producer, comparison, provenance, and thesis use. The
directory tree exposes only some of those relations. Preserve the
repo-specific reasoning and change triggers that later agents need; do not
replace them with a general placement taxonomy.

Declare dependencies where they are consumed: imports, Cargo manifests,
scripts/configs, commands, dataset identifiers, or consumer READMEs. Use
stable, grep-able names so reverse impact can be derived by repository search.
Do not present a manually maintained producer-side consumer list as exhaustive.
Similar executable scaffolding or instrumented implementations need not share
an import merely to reduce edit count.

Document repository-specific facts, decisions, evidence, status, source paths,
sharp edges, and expensive checks. Do not duplicate generic knowledge.

Prefer conventional filenames and stable, grep-able terms. Keep active and
superseded material visibly distinct. When a result's scope or rationale is not
recoverable from the artifact itself, state it beside the artifact.

Navigation views must say what they cover. A view supports only the claims it
actually establishes; merely pointing to evidence does not replace that
evidence. An incomplete semantic/status view must not imply a complete
inventory.

## Baseline commands

```bash
# Worktree
GIT_LFS_SKIP_SMUDGE=1 git worktree add .worktrees/<name> -b <branch> main

# Rust
cargo fmt --check
cargo test -p symplectic --release --lib
cargo test -p algebraic-numbers --release
cargo test -p euclidean-polytopes
cargo build --workspace --release

# Thesis
cd thesis && latexmk && ./check-build.sh

# Formal proof-development document
cd formal && latexmk
```

Producer and experiment READMEs document their commands and output-safety
rules. Read the local README before running a command that may overwrite
tracked evidence.
