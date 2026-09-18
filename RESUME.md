# Resume the thesis here

**Use branch `thesis/resume-20260918`.** Its existing checkout is
`/workspaces/msc-math/.worktrees/candidate-assembly-integration`.
This is the single selected manuscript and consolidated evidence tree. Do not
resume from dirty `main`, an older review checkout, or another research branch.
The directory name is retained because the review server and an idle agent still
refer to it; it does not denote a second candidate.

## State and authorization

The thesis is **not accepted and has not been submitted for final grading**.
Jörn authorized consolidation after rejecting a scattered handback. The completion
attempt was stopped; this handoff is not authorization for a new autonomous run.
The previous deadline was 2026-09-18 22:00 UTC. The last reported account quota was
6% at 16:59:57 UTC; this is a dated observation, not current availability or a dollar
balance. The additional shadow-API ceiling was $3,000, not permission to spend it
without a justified plan. Earlier 2% and 5% completion forecasts were unsupported
and retracted. No reliable writing-to-PASS workflow was established.

PASS means submission-ready, with only minor issues Kai might leave for Jörn to
fix in approximately two hours. Borderline is FAIL. Final grading is one-time;
diagnostic feedback is separate. No university submission or public release is
authorized. Jörn's last reliable availability window ended at 17:00 UTC; his later
brief presence was not an extension. Use the async queue for necessary questions;
make each self-contained, identify the exact judgment/consequence and give your
expectation. Do not ask him to reconstruct facts available here. Ending a routine
status turn does not execute background work.

## Manuscript and build

Build from this checkout with `sh thesis/candidate/build.sh`.
`thesis/candidate/main.tex` selects the active sources, including intentional
`legacy/` fallback for visualization and availability. No other worktree is a
LaTeX input. The consolidation validation and frozen current draft are recorded
in `docs/resume/build-verification.json` and `docs/resume/thesis-resume.pdf`.
The clean build has 94 pages and no warnings. Run `python3 docs/resume/verify.py`
to verify retained versions, source hashes and the PDF. That PDF is a resume
artifact, **not a grading submission**.

The preceding diagnostic is `docs/whole-review/thesis-v3-writing.pdf`, SHA256
`249f597a4094add726f077b83538301ce7394cd42d62420e3970aee4fd683fda` (93 pages).
Since that freeze, commits `ac8da690`, `44372a61`, `54143c6d`, and `4b3a8af1`
clarified notation, framing, the flow genericity proof and the billiard lift.
These changes have no human acceptance. Their records are in
`docs/coordination/{repair-framing,flow-exposition-repair}.md`.

## Where to continue assessing unfinished work

- `docs/coordination/whole-exposition-assessment.md`: source-corrected assessment
  of chapters outside DS/AI. Abstract and flow repairs were subsequently made;
  remaining suggestions are not all independently verified defects. Four of five
  initially alleged proof gaps were downgraded; do not resurrect them as facts.
- `docs/whole-review/thesis-v3-writing-changes.md`: latest DS/AI changes and checks.
  `docs/ai-reflection-review/` retains exact human annotations and revised drafts.
- `experiments/writing-quality/human-review/responses/` and
  `docs/reviewer-trial/human/`: exact scoped human feedback. The earlier DS sample
  was rejected; local positive judgments do not accept the whole chapter.
  `docs/reviewer-trial/evaluation/qualification-utility.md` supersedes the overly
  negative initial assessment of the qualification flagger.
- `docs/ds-retrospective-revalidation/README.md`: 14,335 current capacity intervals
  among 14,336 original bodies, one unresolved numerical-policy case. Volume/sys
  remain numerical. `docs/ds-evidence-closure/` and the DS chapter's cited experiment
  directories retain the original analysis and later empirical results.
- `docs/empirical-viterbo-design/desk/root-review/README.md`: reviewed research
  packets, including the fixed-pentagon/symmetric-partner bound. Their presence
  does not mean manuscript integration or novelty is established. The selected
  product-position result is already in the manuscript. Zonotope bound remains
  conjectural. No further research campaign is running.
- `docs/open-thesis-literature/`, `docs/imported-theorem-contracts/`, and the named
  technical-review directories contain source checks. `docs/candidate-assembly-integration/README.md`
  is a chronological adoption ledger; its earlier counts and open items are not
  current instructions. Later entries supersede earlier ones.

## Consolidation and provenance

`docs/resume/branch-inventory.json` accounts for the changed tracked files on all
38 retained research/quarantine branches: source commit, source blob, owning
thread UUIDs from commit trailers, and the retained path of each version.
442 previously absent paths were imported. Existing active files were preserved;
68 differing historical versions were stored under `docs/resume/historical-variants/`
with `.snapshot` suffixes. This is deliberate evidence retention, not adoption
of old prose or old plans. No pending branch merge is required to read these packets.

`docs/resume/dirty-source-inventory.json` accounts for 288 dirty-file observations
from main and the interrupted review checkout. Matching bytes already in this tree
are identified; 53 differing files are retained as unselected snapshots. Main and
the original review checkout were not modified. Machine-local `.codex/config.toml`
was deliberately not imported. The current project `AGENTS.md` is copied unchanged from main so a standalone
checkout retains its guidance. Do not install other archived startup instructions.
Other worktrees are retained for existing path consumers and rollback; they are
not required to build this manuscript or read the consolidated tracked evidence.

## Sessions and non-repository material

`docs/resume/sessions.json` gives exact thread UUIDs, observed live panes and local
session-log paths. Main coordinator: `01a0b153-5091-7333-935c-f08856210097`;
replacement: `01a0b4e9-b856-7de2-b42a-39d3526743b9` (idle w26:p8).
Empirical, reviewer and human-review desk identities are also recorded. Inspect
live Herdr identity before messaging; old pane numbers are not durable identities.
Logs live in `~/.codex/sessions/`, not `/tmp`; they supplement committed records.

Large historical raw geometry is a registered external artifact, not a temporary
file: snapshot `f7bc6be841e9d30741d5bf7ec8d4f0c0c74ec22745d3325b792e64cec333ca96`
under the `polytope-datasets` artifact registration and standard host cache.
See `docs/resume/external-dependencies.md` for concrete recovery and dependency
checks. Historical reports retain original absolute command/output paths as
provenance; those are not instructions to use the old checkout. For a path beneath
an old worktree, first use the same repository-relative path here, then consult
the branch inventory for an explicitly archived variant. No handoff document or
selected PDF depends on a temporary message file or expiring review URL.
