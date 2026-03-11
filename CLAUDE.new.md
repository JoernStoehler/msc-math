# CLAUDE.md

Master Thesis: Probing Viterbo's Conjecture
Author: Jörn Stöhler, University of Augsburg
Advisor: Kai Cieliebak
Second advisor: Elizabeth Gaar
Timeline: Oct 2025 – March 2026

## [Aspirational] End state

This repo is making progress towards a completed master thesis with:
- A printed-quality LaTeX document `thesis/build/main.pdf`
- A high-performance stable Rust library for symplectic geometry on polytopes in `crates/`
- A reproducible experiment pipeline in `experiments/`

## Mathematical Context

The thesis is motivated by a paper from Haim-Kislev and Ostrover 2024, which disproved Viterbo's conjecture in dimension 4 via an explicit counterexample polytope. The conjecture was until then a famous open problem in symplectic geometry.

Viterbo's Conjecture (2000): For any convex body K in R^2n, including any polytope K in R^4, the systolic ratio `sys(K) = c_EHZ(K)^2 / (2 vol(K))` is at most 1, where `c_EHZ(K)` is the Ekeland-Hofer-Zehnder capacity of K.
Haim-Kislev and Ostrover (2024, Annals): Defines a 10-facet counterexample with `sys > 1`.

We follow Haim-Kislev 2017, Chaidez-Hutchings 2021 in extending the usual smooth symplectic geometry setting to polytopes in R^4. We extend the published algorithms for computing c_EHZ(K) by implementing them in Rust, adding optimizations that exploit known facts about the symplectic geometry of polytopes, and we verify the correctness of our code with excessive paranoia to avoid any errors even on large, or adversarially chosen, polytopes.

We then probe the conjecture by computing `sys` across large polytope datasets and looking for patterns.

## Multi-Language Codebase

Branches often touch multiple languages simultaneously:
- **Rust** (crates/, experiments/): most code that requires performance, or is correctness-critical, or just interacts with other rust code.
- **Python** (experiments/): for plotting, data processing, orchestration, and data science experiments where python is the more common and less cumbersome choice.
- **LaTeX** (thesis/, experiments/): for the thesis pdf, facing the real readers (Jörn, Kai, Elizabeth) and the imagined readers (a motivated MSc student with a background in symplectic geometry and optimization theory).
- **Markdown** (various): agent-facing writeups, including conventions, rules, workflows, documentation, takeaways, experiment ideas, data interpretation, reports and learnings, and much more.
- **Json/Jsonl/Csv** (experiments/): for datasets that are consumed by and produced by experiments. It's just a more convenient data format than binary formats, e.g. easier git diffs.

Each topic section below mentions its relevant review subagent(s) for focused checklists.

## Knowledge Placement

**When you produce new knowledge** (findings, conventions, docs, comments):
- Tied to a specific file or function? → code comment, doc comment, or file header. This is the natural location agents look at when working with that code.
- Applies to most agents? → CLAUDE.md.
- Applies to a minority of agents? → `.claude/skills/*/SKILL.md` (progressive disclosure: name + description always loaded, body on demand).
- Project management (tasks, ideas, deferred work, constraints)? → `TASKS.md` (root). Grows stale; that's fine.
- Session learning or cross-session state? → `MEMORY.md`. Migrate stable entries to CLAUDE.md or standard locations.
- Don't dump unrelated knowledge into README.md files. Each README covers its own directory's purpose.

**When you need knowledge you don't have:**
- Check code comments, file headers, and README.md in the relevant directory first.
- Check CLAUDE.md (you already have it in context — search for keywords).
- Check skill names and descriptions — load the skill if it matches your need.
- Check `TASKS.md` for project-level context (what's planned, what's deferred, why).
- Check `papers/` for referenced paper sources when verifying math or citations.
- Check `.devcontainer/` for environment details (what's installed, how sessions run).

**When editing CLAUDE.md, SKILL.md, or agent prompt files:**
- Load the `writing-conventions` skill first. It contains the rationale, style rules, and cross-reference tag system.
- Editing CLAUDE.md or agent prompts without loading the skill risks breaking conventions that are expensive to detect later.

**Agent prompt architecture:** Subagent definitions in `.claude/agents/*.md` 1:1 copy relevant CLAUDE.md sections into their prompt body. This duplication is intentional — agents reliably follow inline instructions but unreliably follow "go read file X." Agent prompts use `<copied-from>` tags to mark the source section. Details in the `writing-conventions` skill.

## Communication with Jörn

**Before requesting Jörn's attention:** Investigate first. Autonomous investigative work is basically costless. An investigation is worth doing if it either resolves the problem without Jörn, or speeds up Jörn's investigation via a report with preliminary findings.

**When requesting Jörn's attention:**
- Describe the narrowly scoped cognitive task Jörn should do
- Say why Jörn should do it instead of you
- Provide the context it exists within — Jörn usually drops in without working memory of your session
- After pauses in discussion, re-provide session context. Jörn switches between multiple agent sessions and does not monitor what agents do.

**Formatting for efficient exchange:**
- Number items so Jörn can respond "3 yes, 5 no" instead of quoting paragraphs
- Omit filler phrases — aim for efficient information exchange, not politeness
- When presenting decisions with tradeoffs: use tables, quantify costs/benefits, state recommendation upfront
- When you make repo changes Jörn should know about, mention and explain them — Jörn reviews diffs in VS Code but may not check them unprompted

**Interaction dynamics:**
- Push back on contradictions, gaps, unclear statements, and oversights. Jörn is not infallible — he sometimes makes ambiguous typos or has brainfarts — and he welcomes pushback.
- Never take silence as confirmation. Especially during fast-paced back-and-forth where Jörn may respond to only parts of messages, or respond with delay.
- **Word-choice sensitivity:** Jörn communicates distinctions via subtle word choices that agents tend to gloss over. When Jörn says "not quite" and corrects a nuance, the specific words he chose carry meaning. Don't paraphrase corrections back into your original framing — adopt his exact phrasing and check whether you lost a distinction.

## Staying Focused Across Long Sessions

**Plan file as persistent memory:** Update the plan file as you work — it survives context compaction, your working memory does not.
- After completing an item: mark it done, note any surprises or context future items need.
- Before starting a new item: record what you're about to do and why.
- When discovering context relevant to upcoming items: write it into the plan now, not "later."
- When you need something to survive a session boundary or compaction: put it in the plan file.

**What gets lost at compaction** (danger ranking, most to least dangerous):
1. **Scheduled items you haven't started** — you forget they exist and they never get done
2. **Context and considerations for upcoming items** — you redo them from scratch or miss nuances
3. **Completed items** — low cost, already done, only needed for final reporting

**Session recovery after compaction or handoff:**
- If you suspect you lost context: check the plan file first, then MEMORY.md.
- If you need details from the pre-compaction conversation: delegate JSONL transcript reading to a subagent. Never read the transcript yourself — it's too large and wastes your context window.
- Never guess about what happened pre-compaction — verify or say "I don't know."

## Session Workflow



Every agent session owns a git worktree. Subagents and teams work in the same worktree.

**Time economics:** Jörn's time is scarce; agent time is practically free ($0/h). Plans minimize Jörn's workload, even at vastly higher total agent work. We parallelize agents via multiple sessions, agent teams, and subagents.

### Session pattern: scope → plan → implement → review → merge

**Scope phase** (Jörn + agent together):
- Agree on a single chunk of work for this session.
- Jörn scopes the task within his long-term project vision. Agents cannot reliably do this — they lack deep models of how tasks affect downstream work or later sessions.
- Agents provide preliminary investigation findings to help Jörn scope faster.
- Handoff to plan phase happens explicitly.

**Plan → implement → review** (agent autonomous):
- These three phases are carried out autonomously, usually with no involvement from Jörn.
- Jörn is messaged in chat only when his attention is specifically requested.
- Jörn does not monitor agent actions or intermediate status. End-of-turn messages must recap context so Jörn can jump back in without reading the full history.
- Agents decide autonomously when to transition between phases and MAY return to earlier phases (e.g. replanning after a dead end).
- Focus on one phase at a time to avoid splitting attention.

**Merge phase** (Jörn + agent together):
- When the agent is satisfied with its deliverable OR wants to give up, it messages Jörn.
- Include: what happened this session, what unknown unknowns were discovered, how known unknowns were resolved, and a checklist of the final review.
- Only Jörn merges to `main`. Only Jörn creates PRs.

### What needs discussion vs. what doesn't

The deciding factors are rollback cost and verification cost:

**Act freely** — cheap to verify, easy to roll back:
- Writing and editing code (git handles rollback; tests verify)
- Investigation, research, trying things out and throwing them away
- Committing and pushing to the working branch

**Act, then Jörn verifies** — cheap to verify, moderate risk:
- Attempts where agent self-verification is reliable and Jörn's check is fast
- Drafts that are faster to correct than to discuss upfront

**Discuss with Jörn first** — expensive to verify or hard to roll back:
- Scope changes — agents don't reliably notice when they've drifted or when a scope change has bad downstream consequences

**Never without explicit instruction:**
- Destructive operations with no rollback
- Creating PRs or merging to `main`

**When in doubt**, default to discuss-first. Jörn can always override with "just do it."

### Autonomous difficult tasks

Agent time is cheap. Use it aggressively:
- Spawn multiple agents for the same task (or variations) and pick the best deliverable.
- Redo a deliverable based on learnings from a first attempt.
- Run throwaway exploratory tasks whose sole purpose is to learn unknowns.
- **Revert plan required:** For all these patterns, there must be a plan ahead-of-time for how to revert an agent's work. This is why we use git worktrees and why only Jörn merges to `main`.

## Subagents & Review



Spawn a subagent when a subtask can run in parallel, needs isolated context, or benefits from focused work (e.g., literature extraction, code review, exploratory investigation).

- Use Sonnet for read-heavy extraction tasks (literature, code review). Reserve Opus for tasks requiring deep reasoning (mathematical reasoning, code writing).
- Keep subagent tasks focused and small. Agents may stall on tasks requiring 1000+ lines across multiple files.
- **For long-running agents (>10min expected)**: Use `run_in_background=True` so Jörn's messages can reach you during execution.

### The core rule

Never write a factual claim without verifying it against evidence in the same session. "The code cross-checks X" requires reading the code and confirming the cross-check exists. "The data shows Y" requires reading the data. When verification is impossible, mark with `% [TODO: JÖRN -` or `% [GAP -`. Violating this rule is the single most damaging failure mode — it spreads across the whole thesis when others rely on a false claim, and then wastes a lot of Jörn's time to identify downstream issues and redo work.

**Citation verification (core rule instance):** Never produce author names, paper titles, or literature attributions from memory. Always verify against `thesis/bibliography.bib` (for cited works) or the paper files in `papers/` (for author names and content). Agents confidently produce plausible-sounding but wrong author names from training data — e.g., "Cieliebak-Hutchings" instead of the correct "Chaidez-Hutchings" (CH2021). The authoritative sources are:
- `thesis/bibliography.bib` — all cited works with correct author fields
- `papers/<key>/` — local copies of referenced papers

### Review workflow

Reviews use a 3-phase pipeline. Fix each phase's findings before proceeding to the next.

**Phase 0 — Module sanity** (`review-modules`): Folder conventions, builds pass, tests pass, pipeline consistency, data freshness.

**Phase 1 — Syntax/style** (language-based, parallelizable across files):
- `review-tex-style` — LaTeX format, environments, labels, headers, citation format
- `review-rust-style` — code conventions, module structure, cross-ref format, magic number docs
- `review-python-style` — script conventions, paths, headers, figure sizing, visual quality
- `review-notes-style` — README structure, assumptions documented

**Phase 2 — Semantics/content** (concern-based, on clean files):
- `review-tex-math-correctness` — proofs: gaps, unclear steps, mistakes, definition mismatches
- `review-tex-educational` — audience fit, forward refs, pedagogical quality
- `review-tex-facts` — claims vs evidence (JSONL, fixtures, bib data, code refs)
- `review-rust-tests` — test philosophy, coverage, input diversity, property verification
- `review-rust-math-correctness` — doc comment formulas match code, invariant enforcement
- `review-experiment-observations` — reported facts vs JSONL/output data
- `review-experiment-interpretation` — reasoning quality, overreach, editorializing

### How to run reviews (main agent does this directly)

1. `git diff main...HEAD --name-only` → pick relevant subagents from the phases above
2. Run phase 0 (`review-modules`) first if builds/tests might be broken
3. Run phase 1 agents in parallel, fix findings
4. Run phase 2 agents in parallel on the cleaned files, fix findings
5. Present merged report to Jörn

This is mandatory before presenting `.tex` deliverables to Jörn and recommended for all deliverables. Do not delegate this orchestration to a subagent — subagents cannot spawn subagents.

## Git

**Always use local `main`, never `origin/main`.**

Jörn merges locally and pushes later, so `origin/main` is frequently stale. Comparing against `origin/main` inflates diffs with already-merged commits.

**For code reviews:** Use three-dot diff (`git diff main...HEAD`) to show only what the branch changed. Two-dot diff (`main..HEAD`) includes divergence and creates false alarms.

**State the base explicitly:** "Compared against local `main` at `abc1234`."

If unexpected files appear in diff, investigate — likely means branch needs rebasing.

This section is copied to `.claude/agents/{review-modules.md}`.

## Thesis Writing

This section is copied to `.claude/agents/{review-tex-style.md, review-tex-math-correctness.md, review-tex-educational.md, review-tex-facts.md}`.

### Build

```bash
cd thesis/ && latexmk && ./check-build.sh
```

`check-build.sh` parses the build log for overfull hboxes (> 1pt) and undefined references. It exits non-zero if any are found. **Agents must run this after every compilation** and fix any new warnings they introduced.

Available: TeX Live 2023, pdflatex, xelatex, lualatex, latexmk, biber, chktex.

### Jörn Reviews PDF, Not .tex

Jörn reads the compiled PDF. He does not read `.tex` source files for review.

**When presenting content for Jörn's review:**
1. Compile the thesis (`cd thesis/ && latexmk`)
2. Look up the rendered number from `thesis/build/main.aux`
3. Tell Jörn: "Lemma 3.43 on page 25" — not "see rank-deficiency-dismissal.tex"

**When reporting edits:**
- Describe by rendered location: "the proof conclusion of Theorem 5.1"
- Not by source location: "line 418 of simple-minimizer-proof.tex"

**When referring to theorems/sections/equations in chat:**
- Use rendered numbers: "Theorem 5.3", "Section 2.1", "equation (3.7)"
- Not label names: `thm:simple-minimizer`, `sec:algorithm`
- How to get rendered numbers:
  ```bash
  grep 'label-name' thesis/build/main.aux
  ```
  Extract the number from `\newlabel{label-name}{{number}{page}...}`.

Note: In `.tex` source, always use `\ref{label}` — never hardcode numbers. This rule is about **chat messages to Jörn**, not about LaTeX source.

### Theorem/Section Numbers

Never guess — read from `thesis/build/main.aux` after building:
```bash
grep -E 'newlabel\{(sec:|thm:|lem:|def:|rem:|cor:)' thesis/build/main.aux
```

### Rust Cross-References

Rust `///` doc comments reference thesis proofs using `[lem:label]`, `[thm:label]`, `[def:label]`, `[alg:label]` format — matching the LaTeX `\label{}` name exactly. When editing a theorem or lemma in the thesis, grep `crates/src/` for the label to find affected Rust comments:
```bash
grep -r '\[lem:label\]' crates/src/
```
The `\label{}` name is the stable identifier. Rendered numbers (e.g., "Lemma 3.2") appear only in the PDF and must never appear in Rust source.

### Four Audiences

Every line of LaTeX must work for all four audiences simultaneously:

1. **Human readers** (Jörn, Kai, Elizabeth)
   - Want the main result upfront
   - Will skim definitions, revisit if confused
   - All proofs are skippable
   - Value: algorithm, proof ideas, geometric intuition

2. **Imaginary master student** (nominal target)
   - Typical math master background: linear algebra, analysis, basic topology, intro symplectic geometry, intro optimization
   - Every definition stated in full, not deferred to literature
   - Must follow the chapter linearly without external references

3. **QC agents** (verification)
   - Verify one chunk at a time, trusting previously verified chunks
   - For every proof step: must immediately confirm "yes, that follows directly"
   - Words must have clear, specific meanings
   - Never state anything incorrect, even if non-fatal

4. **Downstream agents** (Rust implementers, test writers)
   - Need full detail in all definitions, lemmas, proofs
   - Need ALL properties listed (including unused ones) for generating tests
   - Need concrete values and example calculations

### Correctness

We write mathematics, code, and documentation in a clear, detailed, explicit, structured, verifiable way:
- "clear" = easy to understand, not vague or ambiguous
- "explicit" = relevant implications already spelled out, not left for the reader to derive
- "detailed" = all steps included for verification; only omit steps that are both irrelevant for most readers and straightforward to fill in
- "structured" = organized into modular chunks; readers can keep details for relevant chunks and high-level takeaways for others
- "verifiable" = the reader can check correctness by doing the local validity check for every step and every cross-chunk reference

We refactor, simplify, and improve until verification becomes straightforward and doable for readers. Without straightforward verification, we risk hidden gaps or mistakes.

### Default Status

All content is **agent-written and unreviewed** unless explicitly marked otherwise. When a `.tex` file has no review markers, assume nothing has been verified by Jörn.

### Comment Conventions

Use prefixed comments to separate meta information by audience:

#### Jörn's review status (`% Jörn:`)

Three levels, strictly ordered: **text > math > structure**. Only record the highest approved level — higher implies all lower levels.

1. **Structure**: proof approach/strategy is correct, section organization is right
2. **Math**: mathematical content is correct (but writing may need polish)
3. **Text**: the written prose is correct (final quality)

```latex
% Jörn: structure approved (abc1234) — from \subsection{Sampling procedure} to \end{proof}
% Jörn: text approved (abc1234) — from \subsection{Sampling procedure} to "Acceptance rate sweep"
```

The commit hash is from `git rev-parse HEAD` after committing the approved version.

Only one marker per scope. When a higher level is approved, replace the lower marker. Scope must be explicit (section names or line ranges). Content outside any `% Jörn:` marker is unreviewed.

**Staleness rule**: When an agent edits content within a `% Jörn:` marker's scope, the agent **MUST** delete the marker. The edited content reverts to default status (agent-written, unreviewed). The commit hash serves as a backup for detecting staleness via diff.

Jörn reviews the **PDF-visible text** (rendered output), not the `%` comments.

#### QC agent findings (`% QC:`)
```latex
% QC: polytope per Definition~\ref{def:polytope}, facet data per Definition~\ref{def:facets}
```
→ Instructions for QC agent on what to verify, or resolved QC findings

#### Developer agents (`% Downstream:`)
```latex
% Downstream: R_i = (2.0 / h_i) * J_0 * n_i
% Downstream: Test: |R_i| = 2/h_i for all i
```
→ How to implement in Rust, what tests to write

#### Writing agents (`% [TODO: JÖRN -`)
```latex
% [TODO: JÖRN - verify this E-L derivation. Agent wrote this by expanding the original
%  sketch, but agent-written proofs are unreliable. Check for errors in the calculation.]
```
→ Marks content needing Jörn's attention

#### Gap tracking (`% [GAP -`)
```latex
% [GAP - AGENT CONFIDENCE 70%: The derivation above shows X, but the equation below
%  claims Y. Agent verified lines A-B are correct, but cannot connect them to lines C-D.
%  JÖRN: verify if gap is real, fix if so, or explain the connection if agent missed it.]
```
→ Known mathematical gaps with epistemic confidence

#### Human readers (plain `%`)
```latex
% Use J_0^2 = -I here
```
→ Regular LaTeX comments for humans reading the source

### File Headers

Every `.tex` file starts with a `%` header block containing:

1. **Identity**: `% filename.tex — \input'd from parent.tex`
2. **Sources**: where the content comes from (Jörn's dictation, literature, agent-written, etc.)
3. **Structure**: outline of sections/subsections

Do NOT put review status in the header. Review status lives inline via `% Jörn:` markers.

### Content Rules

1. **Self-contained**: No definition or theorem statement may be deferred to the literature. Every definition is stated in full. Every theorem is stated in full.

2. **Deferred proofs**: A proof MAY be deferred to the literature ONLY IF:
   - The theorem exceeds thesis scope due to complexity, AND
   - The proof is not relevant to the thesis—only the theorem is.

3. **Notation consistency**: Notation and definitions must match `correspondence.tex` exactly. If correspondence.tex uses symbol X, this file uses symbol X. No synonyms, no alternative forms, no "equivalent" restatements unless a lemma proving equivalence is included.

4. **Writing rule**: Proofs cannot cite external sources mid-proof. External results must be proven inline or stated as Claims within the proof. The thesis must be self-contained and verifiable by reading this document alone.

5. **Citation verification**: Author names and paper attributions must be verified against `thesis/bibliography.bib` or `papers/`. Never produce author names from memory. See Subagents & Review § The core rule for details.

### Proof Writing

**Structure**: Assumptions → Claim → Overview → Steps → Conclusion

**Level of detail**:
- Detailed enough for Jörn to verify by skimming
- Annotate non-obvious steps: cite the specific theorem/lemma used, state why hypotheses are satisfied
- Never gloss over gaps or handwave — if a step is non-trivial, say so explicitly

**Agent limitations — what agents CAN do:**
- Turn natural language descriptions into proofs
- Improve proof writing, fix errors, detect suspicious steps
- Report unclear or suspicious proof steps

**What agents CANNOT do:**
- Provide final high-reliability verification — that must come from Jörn
- Agent skill at spotting errors is specifically "only okay" — not bad, not good
- Agents can spot errors, but only in proofs written in a clear, detailed, explicit, structured way. In less perfect writing, errors and gaps can be overlooked.

**Every proof must pass Jörn's verification after every edit.** We must be able to trust and build upon verified proofs. Never claim Jörn "approved" content unless he explicitly verified the math.

### Emphasis and Structure

- **Emphasis proportional to importance**: Don't dedicate a full section to a trivial identity
- **Geometric definitions first, formulas derived**: E.g., action = integral of Liouville form, not the coordinate expression
- **Standard definitions**: Use them exactly as in the literature. If you use a different form, state a lemma proving equivalence

### Format Rules

- Use `\definition`, `\lemma`, `\theorem`, `\proposition`, `\proof` environments
- Use `\remark` and `\example` for context/intuition/illustrations
- No prose paragraphs outside environments, except minimal connective text between environments
- Calculations displayed as formulas, not described in English prose

## Experiment Writing

This section is copied to `.claude/agents/{review-tex-style.md, review-experiment-observations.md, review-experiment-interpretation.md}`.

Builds upon **Thesis Writing** — all Thesis Writing conventions apply to experiment `.tex` files too, except those specific to mathematical proofs (Proof Writing, Four Audiences' "imaginary master student" criterion). This section adds experiment-specific conventions.

- **Write up what's there — nothing more, nothing less.** Report what the data shows. No invented interpretations, no omitted patterns, no editorializing. Facts are facts, correlations are correlations, unknowns are unknowns. Speculation must be explicitly labeled as interpretation.
- Experiment writeups live in `experiments/<name>/<name>.tex`, wired into the thesis via `\input`
- Every factual claim must be verified against the actual data (JSONL) in the same session
- When verification is impossible, mark with `% [TODO: JÖRN -` or `% [GAP -`
- Agent-generated figures and writeups are drafts until Jörn reviews
- Results checked by Jörn before inclusion in thesis
- Statistical claims require reproducible computation
- Plots visually inspected for sanity

## Rust Library

This section is copied to `.claude/agents/{review-rust-style.md, review-rust-tests.md, review-rust-math-correctness.md}`.

**Invariant:** `cargo test` passes from `crates/` with zero failures.

### Module structure

Single crate `symplectic` with modules:
- `geom::*` — polytope types, geometry primitives
- `algorithms::hk2017` — general capacity (exponential)
- `algorithms::billiard` — Lagrangian product capacity (fast)
- `algorithms::tube` — tube algorithm (placeholder)
- `kkt` — shared KKT solver (used by hk2017 and billiard)
- `constants` — shared tolerance constants
- `random` — random polytope generation
- `dataset` — dataset serialization

**When modifying shared modules** (kkt, constants): Check all callers. Use `cargo test --lib` to verify.

### Three capacity algorithms

| Module            | Applies to                        | Cost                    |
|-------------------|-----------------------------------|-------------------------|
| algorithms::hk2017| All polytopes                     | Exponential in #facets  |
| algorithms::billiard| Lagrangian products only         | Fast                    |
| algorithms::tube  | No Lagrangian 2-faces             | Polynomial–exponential  |

Where domains overlap, algorithms must agree on the computed capacity.

### Coding conventions

- Colocated tests: `foo.rs` has `foo_test.rs` in the same directory. Submodule tests use `#[path = "foo_test.rs"]`.
- A source file may have multiple test files (e.g. `foo_math_test.rs`, `foo_test.rs`)
- Prefer iterator chains over `for` loops. Minimize mutable state. Use `map`, `filter`, `flat_map` for transformations.
- Types encode mathematical invariants, validated at construction
- nalgebra for linear algebra, proptest for property-based testing
- **Coordinate convention**: (q₁, q₂, p₁, p₂) — components [0,1] = q-space (Lagrangian), [2,3] = p-space (Lagrangian), [0,2] = (q₁, p₁) symplectic plane, [1,3] = (q₂, p₂) symplectic plane. Defined in `geom/symplectic.rs`. Common mistake: assuming (q₁, p₁, q₂, p₂) ordering.
- **No rayon inside algorithms**: Parallelism is at the dataset level (multiple polytopes in parallel), not inside capacity algorithms like HK2017.

### Mathematical documentation

- Definitions, lemmas, and proofs live as doc comments on the corresponding types/functions
- Long proofs are outsourced to colocated `*_proof.md` files
- The Rust crates are self-contained mathematically — no dependency on thesis/. The thesis is downstream
- Quality bar: specific, correct, detailed, clearly written enough that (1) Jörn can verify with low effort and (2) agents can rely on them when implementing

**Math-code correspondence:** Rust types, function signatures, and function bodies 1:1 correspond to mathematical definitions. "1:1" means literal structural correspondence, not just "inspired by."

**Verification criteria for mathematical doc comments:**
- Doc comment formulas must match code's actual computation (not aspirational, not approximate)
- Invariants stated in doc comments must be enforced by types/constructors/assert!/debug_assert!
- Properties stated in doc comments must have corresponding tests

### Cross-references to thesis

When a Rust function implements something proved in the thesis, reference the proof by its LaTeX `\label{}` name. Rules:

1. **Format**: `[lem:label]`, `[thm:label]`, `[def:label]`, `[alg:label]` — matching the LaTeX `\label{}` name exactly.
2. **Always include** a one-line English description of what the referenced result says. Example:
   ```rust
   /// Maximises Q(β) subject to the KKT constraints; see `[lem:kkt]` (thesis):
   /// the unique maximum exists and equals 1/(2·action(orbit)).
   ```
3. **Never duplicate proofs** inline. The comment says *what* the code computes and *which lemma* justifies it. The thesis says *why*.
4. **Never use rendered numbers** like "Lemma 3.2" — these change when sections renumber. Use the label.
5. **Verification**: grep `crates/src/` for `[lem:...]`, `[thm:...]`, `[def:...]` occurrences, find the `.tex` `\label{...}`, and check the lemma statement matches what the comment claims.

### Testing philosophy

Two classes of tests, both applied excessively:

1. **Math proposition tests** (due diligence falsification): proptest generators approximate mathematical quantifiers ("∀ polytopes K", "∀ A ∈ Sp(4)", etc.). Properties under test are mathematical propositions (e.g. J^2 = -I, symplectomorphisms preserve capacity).
2. **Standard correctness tests**: Rust best practices for correctness-critical code — edge cases, invariant checking, regression tests.

**Test exhaustiveness is Jörn's domain.** Beyond conventional software tests, we add unusual test suites that check the correspondence of our code with our mathematical definitions and proofs. Jörn must design which mathematical propositions the test suites need to cover, because the difference between high-confidence and moderate-confidence correctness signals requires complex domain models of the whole proof that agents do not have. Agents CAN brainstorm, implement, and debug mathematical proposition tests. Agents CANNOT provide the exhaustiveness signal.

### Testing expensive functions

For expensive functions (e.g., `ehz_capacity()` with exponential cost), split tests into two categories:

#### Category A: Input-Output Behavior
**What it tests:** Does `f(input)` return the correct output value? Mathematical properties (conformality, monotonicity, etc.).

**Test strategy:**
- **Preferred:** Use fixtures (pre-computed in release mode), run tests in debug suite (fast, <1s)
- **Alternative:** Mark `#[ignore]`, run in release mode (slow but thorough)

#### Category B: Internal Behavior
**What it tests:** Does the code execute safely without crashes, bounds errors, overflow, or assertion failures?

**Test strategy:**
- Run in debug mode (enables debug_assert!, overflow checks, bounds checks)
- Use small inputs (F ≤ 6 for capacity) to stay fast (<5s per test)

### Test organization

| Pattern | Suite | Speed | Use for | Example |
|---------|-------|-------|---------|---------|
| **Fixture-based property** | Default (debug) | <1s | Math properties vs pre-computed fixture | `capacity_properties_test.rs` |
| **Internal behavior smoke** | Default (debug) | <5s | Small inputs (F ≤ 6) with debug checks | `lib_test.rs` |
| **Expensive input-output** | `#[ignore]`, release | ~1s release | Complex cases (F > 8), fixture unsuitable | `pentagon_capacity()` |
| **Fixture generator** | `#[ignore]`, release | minutes | Regenerate fixture after code changes | `test_dataset.rs` |
| **Staleness detector** | Default (debug) | <1s | Warn if fixture out of sync | `fixture_staleness_check()` |

Every test MUST have at least a doc comment stating the mathematical property it asserts. Tests for expensive or complex functions should additionally explain why they use their execution mode (debug/release/fixture), why they use their specific input, and relationship to other tests (if any).

### Test suites

| Suite | Command | When to run | Time |
|-------|---------|-------------|------|
| **Default** | `cargo test --lib` | Every iteration | ~145s wall |
| Regenerate capacity fixture | `cargo test --release regenerate_test_dataset -- --ignored` | After changes to `ehz_capacity()` | ~20s |
| Expensive capacity tests | `cargo test --release -- --ignored` | After capacity algorithm changes | ~2s |
| All ignored tests | `cargo test -- --ignored` | Full validation | ~5 min |

**Fixture location:** `tests/fixtures/capacity_dataset.json` (committed, 27 polytopes with precomputed capacities, scaled variants for conformality tests).

### Magic numbers

Empirically chosen constants (tolerances, thresholds, cutoffs) must have their rationale documented in code comments on the constant definition itself. Include: why that value, what data point motivated it, known limitations, and what must be re-validated if changed.

### Performance claims require measurement

Never state performance without benchmark. "~1ms" is a claim. "Benchmark shows 1.5-2.0ms for 5-16 facets" is measured. Add benchmark if claim exists without measurement.

### Thesis constraints

Polytope4D: 5-16 facets typical. Research code, March 2026 deadline. Correctness > performance.

Don't suggest: Theoretical numerical analysis, O(n²) documentation when n ≤ 16, production features unlikely to matter.

Do suggest: Critical path tests, benchmarks for claims, robustness fixes (timeouts, limits).

### Commit checklist

Before final report:
- [ ] All tests pass (`cargo test`)
- [ ] Zero clippy warnings (`cargo clippy`)
- [ ] Critical paths have tests
- [ ] Performance claims have benchmarks
- [ ] Working tree clean (no uncommitted changes)

## Experiments

This section is copied to `.claude/agents/{review-modules.md, review-python-style.md, review-notes-style.md}`.

Per-experiment folders under `experiments/`, each containing: Rust binary (.rs), Python script (.py), LaTeX writeup (.tex), data (.jsonl), figures (.png), and README (.md).

Pipeline: Rust binary → .jsonl data → Python script → .png figures → .tex writeup → thesis

**`experiments/reproduce.sh`** documents the full pipeline from zero data to compiled thesis. It is the single source of truth for reproduction. When adding, removing, or changing an experiment, update `reproduce.sh` to match.

**Library stability boundary:** Only stable, proven code goes into `crates/` library. New algorithm variants are self-contained in experiment binaries (e.g. `ablation.rs`). Copy library internals into the binary where needed. If a variant is later promoted to production, it enters the library then.

### Philosophy

Experiments are always investigative — even mature ones with thesis-ready writeups remain open to revisiting, expansion, and updating (e.g. when assumptions break or new ideas emerge).

Progression is fluid, with no clear cutoff points:
- From [nothing] → [idea] → [plan with preliminary findings] → [active hypotheses with mysteries] → [findings from non-runnable code] → [failed attempt summary] → [cleanup commit in git log]
- From [nothing] → [full bundle: scripts + thesis section + datasets + extra rust code]

Agents constantly comment on, iterate, clean, refactor, and narrow experiments — tweaking parameters, trying variations, exploring edge cases, simplifying code, focusing scope, removing dead ends.

When cleaning up code that's no longer useful:
- If learnings worth preserving: create `experiments/<topic>.md` with git ref to last commit
- Otherwise: just delete (it's in git history)

### Directory structure

```
experiments/
  Cargo.toml             Builds all experiment Rust binaries (depends on symplectic crate)
  reproduce.sh           Source of truth for full pipeline (zero data → thesis PDF)
  IDEAS.md               Ongoing thoughts, ideas, edge cases, preliminary findings
  <name>/                Per-experiment folder
    <name>.rs            Rust binary source
    <name>.py            Python analysis script
    <name>.tex           Thesis writeup section
    <name>.jsonl         Dataset (generated by Rust binary)
    <name>.png           Figures (generated by Python script)
    README.md            Findings, methodology, key results
  requirements.txt       Python dependencies
```

### Script conventions

**File naming:**
- Each experiment lives in `experiments/<name>/`
- Rust binary: `<name>.rs`, Python script: `<name>.py`, Writeup: `README.md`, Thesis section: `<name>.tex`, Data: `<name>.jsonl`, Figures: `<name>.png`

**Independent scripts, not a package:**
- No `__init__.py`, no shared imports between scripts
- Each script is self-contained: reads data, performs analysis, writes output
- If two scripts share logic, copy-paste until it stabilizes

**Script headers:**
Every script must document in the docstring:
- **Goal**: What question does this answer?
- **Input**: What data does it read?
- **Output**: What files does it write?

Example:
```python
#!/usr/bin/env python3
"""
Analyze systolic ratios across polytope dataset.

Goal: Identify distribution of sys values, locate counterexamples
Input: experiments/<name>/data.jsonl
Output: experiments/<name>/histogram.png
"""
```

**Path conventions:**
```python
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parent.parent.parent
EXPERIMENT_DIR = Path(__file__).resolve().parent  # data/figures are colocated
```

No hardcoded paths outside `REPO_ROOT`.

**Error messages:** Make them actionable. Bad: "File not found". Good: "File not found: data.jsonl. Run Rust binary first."

**Dependencies:** Listed in `experiments/requirements.txt`; install with `pip install -r experiments/requirements.txt`. Use plain Python with standard data science libraries (numpy, pandas, matplotlib, scipy). No custom framework.

### Figure sizing

All figure formatting is handled in Python. LaTeX is a 1:1 pass-through (`\includegraphics{file.png}`, no `width=`/`scale=`).

- `figsize` = the physical size in the printed PDF. `\textwidth` ≈ 5.4" (A4, 12pt article, default margins).
- `bbox_inches='tight'` expands the output beyond `figsize` to fit labels. Verify the output PNG width fits.
- Multi-panel figures at 5.4" are often too cramped. Prefer separate figures over wider canvases.

### Pipeline direction

Rust binary → .jsonl → Python script → figures/tables → thesis

**No circular dependencies:**
- Python never calls Rust directly
- Rust binaries are built from `experiments/Cargo.toml` (`cd experiments/ && cargo build --release`)
- To add a new experiment: create `<name>/` folder, add `[[bin]]` entry to `Cargo.toml`, update `reproduce.sh`

### Data and figures in git

**Tracked (committed):**
- `experiments/<name>/*.jsonl` — datasets generated by Rust binaries
- `experiments/<name>/*.png` — figures generated by Python scripts
- `experiments/<name>/<name>_output.txt` — stdout from binaries whose deliverable is stdout

**Why:** Worktrees inherit data immediately, changes are visible in diffs.

**Regeneration convention:**
- **Regenerate on the branch that changes the code.** Data should match the code that produced it.
- **Separate commits**: Code changes committed separately from data regeneration

**Merge conflicts (data/figures):** Resolve by regenerating on the merged result.

### Quality standards

**Rerunnable from zero:**
- Starting from empty experiment directories, running all scripts should reproduce all outputs
- No manual steps, no "run this once, then comment it out"

**Document assumptions:**
- If script assumes file exists, document it in header and error message
- Example: "Assumes benchmark.jsonl exists. Run: cd experiments/ && cargo run --bin benchmark --release"

**Not production code:**
- No exhaustive testing required (not like Rust crates)
- But must be reproducible
- Focus on clarity and correctness over performance

## Environment

- Sessions run in a devcontainer with the repo at `/workspaces/msc-math`.
  - Worktrees: use `--worktree` flag or `EnterWorktree` tool. Hooks in `.claude/hooks/` override defaults to branch from local `main`. Worktrees land at `.claude/worktrees/<name>/`.
- Pre-installed: Rust 1.93 (cargo, clippy), Python 3.11 (pytest, ruff, mypy, black), gh CLI (via post-create hook)
- LaTeX: TeX Live 2023 (pdflatex, xelatex, lualatex), latexmk, biber, chktex

**Runtime limits:**
- Repeated standard commands (tests, builds, lints) **must complete in ≤10 minutes**
- This prevents triggering the CPU monitor, which kills sessions after 20min of sustained high CPU
- Exceptions: one-off tasks like finished experiments, final dataset generation, or thesis compilation
- For tests: tune proptest parameters, mark slow tests with `#[ignore]`, or split into fast/slow suites
- If a command needs >10min repeatedly, it's a signal to optimize or redesign

## Quick Commands

```bash
# Rust
cd crates/ && cargo build
cd crates/ && cargo test --lib
cd crates/ && cargo clippy --lib -- -D warnings

# Long-running commands: always wrap with timeout to prevent zombie processes
timeout 5m cargo test              # routine tests
timeout 30m cargo test -- --ignored  # slow property/monitoring tests

# Python
ruff check experiments/
pytest experiments/

# LaTeX
cd thesis/ && latexmk
```

## Archaeology

The `archaeology/` directory contains files recovered from `msc-viterbo`, an abandoned predecessor repo. **Everything here is untrusted.** Do not trust, adopt, edit, copy from, or load into context without specific reason. Read for ideas and warnings only.

## Working notes (redistribute later)

**Rollbacks are cheap.** Git handles rollback; agent time (1h = $0) is practically free. Commit your work regularly so rollbacks are possible and so context survives compaction. When you defer a question to keep working, write it down (plan file or TODO comment) so it doesn't get dropped. Deferred ≠ dropped. The worst case of deferring is wasted agent time — which is acceptable unless Jörn is actively waiting on you or important session context will be lost.

**Fix obvious bugs you find, even if another agent wrote the code.** Don't ignore problems just because they weren't your fault. Report what you found and fix it — or if the fix is risky/large, report it and explain why you didn't fix it.
