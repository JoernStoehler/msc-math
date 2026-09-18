# Second thesis sprint — coordinator starting packet

Status: starting packet prepared on September 16; not an instruction to launch a new thread or start its clock. Source recovery, both replacement drafts and task-graph routing are complete.

## Open these artifacts first

- `thesis/candidate/main.tex` is the recovered frozen candidate; build from repo root with `sh thesis/candidate/build.sh`.
- `docs/source-recovery/README.md` and `verification.json` record the isolated rebuild: all 55 inputs matched recorded checksums; all 86 pages match frozen text and rendered pixels at 96 dpi. PDF bytes differ; content does not. Recovery is not complete experimental reproducibility.
- `docs/review-evidence/README.md` routes the ordinary-path Pro package, self-reviews and human feedback. `docs/source-recovery/evidence/frozen.pdf` is the original frozen thesis.
- `docs/pentagon-chapter-v2/README.md` describes the completed four-page rewrite and integration obligations. It does not contain the DS ridge relation. Jörn accepted the preceding version's mathematical argument, not the new prose.
- `docs/literature-rewrite/evidence-map.md`, `replacement-context.tex`, `references.bib`, and `standalone.pdf` are the fresh Sol synthesis with Luna source checking. PDF is four pages, not human-approved; integrate or edit for the introduction's appropriate length rather than inserting it blindly.
- `docs/sprint-two/integration.md` maps both replacements into the recovered paths, including overview, bibliography and obsolete-appendix changes.
- `docs/task-graph/` is the current rendered task map. The old graph files are preserved as history, not active running-state evidence.

Preparation checkpoint: `3b180fb2` (source/evidence recovery, completed drafts, graph routing and handoff). Preserve unrelated dirty work. No push or reset is part of preparation.

## Outcome and user budget

The objective is Jörn's explicit whole-thesis PASS judgment, for a mathematically mature master's reader with a smooth symplectic background. A build, automated review or chapter-level approval does not establish this outcome. Preserve the research and broadly the thesis structure; large reorganizations and new experiments need justification against that objective, not automatic adoption from a review.

Jörn offers about 120 minutes total attention for sprint two. Async replies usually take 10 seconds–10 minutes, occasionally an hour; a sustained reading block can be about 30 minutes. Use his async inbox for consequential questions, work on independent tasks while replies are pending, and avoid serial sentence-repair requests. Jörn explicitly rejected being asked to guess when the work will finish. The new coordinator owns a brief scope-based delivery estimate, communicates uncertainty and updates it from progress; do not infer a deadline from the old September 14 deadline or the preparation budget. Bring Jörn consequential tradeoffs, not routine scheduling or Git housekeeping decisions.

The outgoing coordinator has a separate 15-minute preparation budget ending approximately September 16 09:23 UTC / 11:23 Berlin. Extensions require a concrete bounty proposal: deliverable, benefit, cost and observable completion. This is not the new sprint's duration or Jörn's attention budget.

## Starting evidence and approval boundaries

- Original September 14 18:00 Berlin approval target was missed; Jörn later judged the frozen 86-page thesis FAIL. Its PDF SHA-256 is `d7dae9a78dffc87fe89f3005bed9b6b51d28728fa90e1a5c811bc09b236e1695`.
- Jörn judged the original HKO chapter PASS and Chapter 4 appropriate/PASS with borderline writing. Do not spend his time rechecking unchanged accepted material without a reason.
- On September 16 he accepted the new analytic rotated-pentagon argument, but rejected its exposition. `docs/pentagon-chapter/user-reading-feedback.md` records the exact boundary. The latest four-page rewrite in `docs/pentagon-chapter-v2/` has agent reviews, not human prose approval.
- A frozen ten-minute self-review preceded the external Pro review. Pro contributed the analytic pentagon proof, a restricted-family ridge–capacity identity, important literature corrections and independent HKO computational corroboration. Preserve that provenance; do not retroactively attribute the discoveries to the original DS work.
- Two process audits are complete. `docs/process-audit/independent/reconciliation.md` substantially confirms the first with modest refinements. No third general audit is needed to begin thesis repairs.

## Work selection

1. Start from the recovered frozen candidate, not the legacy thesis entry or an arbitrary later fragment. Confirm the source-recovery handoff and its scientific-evidence limits. Do not recreate sources under `.git/`.
2. Integrate the completed analytic pentagon replacement and fresh literature draft after checking the actual artifacts. Their integration notes identify old overview/appendix claims that must change too. Keep recovery verification separate from these substantive edits.
3. Give the DS chapter a fresh purpose/evidence triage rather than polishing the existing experiment inventory by default. Its objective includes finding patterns, reconstructing geometric meaning, and developing conjectures/proofs; high-ratio search is a separate possible contribution. Existing experiments are not invalidated merely because the first narrative was poor. Decide which primary claims have reader-visible evidence and recoverable producers/inputs, and which should be narrowed, demoted or replaced. The known CH2021 construction/reference obligation needs closing in the assembled chapter.
4. Put the exact ridge–capacity relation in the DS discussion if it earns its place there; Jörn explicitly rejected placing it in the rotated-pentagon chapter. Keep its restricted-family scope clear.
5. Rebuild abstract/introduction/conclusion around the actual results and align the remaining exposition, particularly preliminaries. Choose replacement rather than local patching when the old draft is a poor scaffold.

At an expensive research decision, consider whether a different route or a connection between existing results can answer the question more cheaply. This is not a demand for novel proofs before using already-known repairs. Distinguish a scientifically negative result from an execution-incomplete attempt: recovering CEM with matched controls was useful, not wasted work.

## Production and review responsibilities

**Observed resource constraint:** this session exposes four internal agent slots including the root. A Sol spawn was blocked while root, Sol, recovery and an interrupted worker occupied the tree. The reason/configurability of that limit has not been established; its presence must not be silently treated as an optimal resource allocation. Check actual capacity when planning parallel work, surface a binding limit and its consequences, and investigate a justified configuration change if needed. No measured attribution of the first sprint's failure to this limit exists.
**Documented control, not yet tested here:** official Codex documentation on September 16 names `agents.max_concurrent_threads_per_session` (excluding the primary thread), with `agents.max_threads` as a legacy alias: https://learn.chatgpt.com/docs/agent-configuration/subagents . Neither spelling was found in the inspected user/repository config files. The documented count semantics differ from the four-slot description exposed in this session. Effective runtime source, overrides, and a fresh-session test remain unresolved; do not claim a config edit has increased capacity without observing it.
**Resource decisions:** Jörn explicitly objects to silently abandoning a valuable approach because the current environment lacks resources. For a binding RAM/compute/concurrency constraint, estimate the demand, alternatives, cost and expected benefit, and bring him a concrete resource/bounty decision before discarding the approach on that basis. His example of a proof needing 32 GB/cloud RAM was hypothetical, not an established project incident. This does not require approval for every ordinary bounded computation or authorize provisioning paid resources without consent.
**User boundary:** Jörn objected to opening a Herdr pane merely to bypass that internal limit. Background-agent requests do not authorize changing his pane layout. The unrequested Luna pane was closed after its completed files were preserved. Do not repeat that workaround without his permission.

The root coordinates scope, ownership, integration and Jörn's attention; substantial drafting and review should live in bounded worker contexts. Authors own draft→review→revision→rendering, with clear write boundaries and completed artifacts returned to root. Give them the task-specific user feedback and positive examples, not only generic audience instructions. Delegate useful independent work without invented approval gates; escalate genuine scope changes or external authority requirements.

The recent failure was not lack of a review label: reviewers accepted recoverable mathematics while missing unsuitable prose. Use the actual passing HKO text and rejected pentagon passages as diagnostic evidence. `docs/pentagon-chapter-v2/review.md` records that one diagnostic still missed the implementation-led opening before its criterion was corrected; this is not a validated automated substitute for Jörn.

Judge what the reader needs, in the order needed. Examples already rejected include introducing an unexplained objective, gratuitous shorthand, unnecessary terminology such as “first harmonic”, implementation-first framing (“Floating-point evaluations”), vague transition claims, premature caveats and redundant previews. These are examples of a broader judgment, not a banned-word checklist. Preserve mathematical meaning when revising prose.

Use diverse short review samples when whole-document risks are unknown, then direct Jörn's sustained reading toward remaining consequential uncertainty. First-objection feedback is useful but should not consume the whole window repeating known local defects. Keep the PDF fixed during reading. The current review-files helper invalidates old links when invoked and serves symlinks, so immutable artifact filenames and coordinated delivery matter; do not overwrite a served file.

## Literature and provenance

The old claim that EHZ/cylindrical equality remains open for convex bodies in R4 is wrong. The fresh literature lane verifies the primary sources and rewrites the context from the mathematical question. An old source saying “open” does not establish current status; seek subsequent evidence and retain uncertainty if unresolved. Agreement between reviewers using the same old source is not independent corroboration. Scope/date notes belong with the evidence, not as an audit essay in thesis prose.

Source leads already checked in prior work include Abbondandolo–Edtmair–Kang arXiv:2412.01777 Corollary 1; Balitskiy–Mitrofanov–Polyanskii arXiv:2603.12495 for factor-family restrictions; Haim-Kislev arXiv:2511.16644 for relevant cuts/Zoll context. Use the new source map for exact versions, scope and relevance rather than copying this routing paragraph into the thesis.

## Start and completion

Read the recovered candidate README, current literature handoff and pentagon-v2 integration note. Identify the highest remaining thesis risk and assign bounded work while integration proceeds; do not spend the opening hour recrawling process logs. Establish and communicate a scope-based delivery estimate rather than asking Jörn to supply it. Use honest checkpoints and raise deadline risk early.

Completion requires a fixed assembled PDF and Jörn's explicit whole-thesis judgment. Report residual limitations and exact artifact identity; do not retroactively convert narrow approval or mathematical proof acceptance into chapter/writeup PASS. No new sprint has been started by this preparation packet.

Preparation environment change: the graph worker installed Graphviz and graph-easy to validate the migrated viewer. Original hidden files were preserved; graph source/consumer paths changed together. Current skill validation and rendering passed, but this does not establish that the broader workflow will produce human-approved prose.
