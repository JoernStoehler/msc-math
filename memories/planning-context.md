# Planning context: authoring, review and deferred summaries

Source: Jörn's live migration discussion, 2026-09-05–07. User statements below
are paraphrases except the quoted phrase. Check the dated work map separately
for current activity.

## User decisions and distinctions

### Writing and review

Jörn identified discovering a writing workflow that produces a good thesis as
instrumental to passing his full-PDF review, ideally once. He has no trusted
automated final gate; his current gate is “jörn reads the whole damn thing”.
That review is costly and ideally happens on a PDF that passes and is submitted.

Reviews during authoring serve a different purpose: feedback about what is
wrong or could be better, not certification that writing is finished. No fixed
set of those reviews is known. Agents can adapt them without asking Jörn to
approve every workflow change; a hypothetical review architecture is premature.

One concrete quality issue Jörn can spot well is a proof jump a human reader
cannot fill. An agent can investigate whether the intended bridge is false or
whether rewriting/splitting it makes the reasoning clear. Proving the endpoint
by an unrelated route does not vindicate presenting the original implication
as an obvious step.

### Why summary design is deferred

Authors need reusable understanding rather than repeated codebase crawls;
reviewers additionally need to trace thesis claims backwards to support. Jörn
expects an author to adapt summaries through both writing and the feedback it
uses. Detailed summary-layer design was therefore deferred until those needs
are better understood—not because reusable understanding is unimportant or
authoring-workflow discovery is complete. This expectation has not been tested
as a full authoring method.

Jörn raised folder layout as an unexamined part of readiness for workflow
design. That motivates assessment, not a conclusion that moving files helps.
He also rejected nested linking as the assumed retrieval method: delegated
full-text search and retrieval can be useful without a prescribed link crawl.

### Planning and preservation

Jörn accepted a non-authoritative task graph alongside memories of mixed
freshness and evidential strength. The graph maps work; it need not contain all
the hard-to-recover observations, ideas and reasoning supporting that work.

For evidence recovery, absence from HEAD can be mere overhead when an old
commit, inputs and commands are recoverable. Unknown reproduction is a
support gap. Calling an unsupported report historical does not make it evidence.

Jörn allows small reversible edits under agreed criteria. The costly mistakes
are wrong approaches, expensive searches and broad changes made before discussing
their basis—not a cheap file edit that can be rolled back.

## Observed planning failure

After summary design was deferred, the main agent narrowed the session to
navigation housekeeping and migration closure. Jörn corrected the omitted
authoring-workflow branch. `memories/todos.md` at `03f74728` likewise framed the
remaining scope as navigation. The omission occurred; the conversation did not
establish a general model-level cause or validate a preventive intervention.

The reasoning worth retaining is that useful summary design is expected to
emerge through writing and feedback; postponing the design leaves that learning
open. Completing tool repairs does not establish that the authoring path is
sufficient. This is the agent's synthesis of the correction.

## Bounded checks

- The [HKO pilot](hko-author-context.md) and
  [optimizer pilot](optimizer-review-route.md) connect named author/reviewer
  questions to source passages and support limits. An independent design review
  reported no material issue. A fresh reader answered those questions using
  the index, two entries and two active thesis passages, preserving the feasible
  upper-function distinction and summary-check versus reproduction boundary.
  This was one retrieval trial, not a writing-quality or mathematical audit.
- A separate search-based navigation check found the supported scalar capacity
  API, historical optimizer evaluators and exact flow-graph route without a
  concrete obstruction or conflicting current-status claim. It used the crate
  README/API source, optimizer runner/comparison READMEs and flow-graph
  README/exact-search source. No repair was indicated for that case; it did not
  establish repository-wide layout suitability.

Current agent hypothesis: layout assessment and authoring investigation can
proceed in parallel and inform each other. Concrete retrieval or ownership
friction could justify layout changes; neither investigation needs a complete
summary layer first.
