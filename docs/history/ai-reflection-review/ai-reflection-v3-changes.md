# AI reflection v3: repairs to v1

**Comparison baseline:** v1, commit `255dc128`, the version Jörn annotated.
Version 3 restores that text and applies the ten annotations. It does not use
v2's rewritten opening or its reorganization of the argument.

**Unchanged anchors:** the entire opening paragraph; the heading and both
retained paragraphs of the HKO account; the whole-task/budget discussion after
its first paragraph; and the final three paragraphs of “Why exposition
remained difficult”. Bibliography and disclosure are unchanged.

## Annotation-to-edit map

| # | Annotation anchor in v1 | Change in v3 |
|---|---|---|
| 1 | “The desired result was clear enough…” | Removed the annotated final HKO paragraph. The two preceding paragraphs remain verbatim. |
| 2 | “A clear objective makes the inferred criterion…” | Replaced inference/coincidence with sufficient overlap between the pursued criterion and the user's needs. Also corrected the subsection heading and its two earlier references to inferring a criterion, so they do not contradict this correction. Exact new wording is below. |
| 3 | “A prompt does not update the model's parameters” | Deleted. |
| 4 | “it supplies the context in which…” | Replaced with the author's working interpretation of prompting through textual cues and deliberate reasoning. Exact new wording is below. |
| 5 | “Some of my threads extended beyond a hundred turns.” | Deleted; no new measurement or figure added. |
| 6 | “each new message could receive a plausible response…” | Deleted the sentence beginning “Habits inherited from short, fast exchanges…”. |
| 7 | “Missing context or a confused description…” | Deleted the unsupported causal sentence. The opening sentence about long conversations remains and now leads directly into the checkpoint paragraph. |
| 8 | “Scientific exposition instead needs to show…” | Deleted the proposed definition. |
| 9 | “An agent's ability to decipher a dense paragraph…” | Preserved that sentence and added the requested cross-page reference/memory example. Exact addition is below. |
| 10 | “The changing human contribution” | Removed the recap subsection and its two paragraphs. The existing opening and whole-task discussion still explain the shift away from directing individual steps. |

## New wording to assess

For annotation 2, the heading is **“A pursued criterion of success”**. The
first paragraph's last two sentences now read:

> An agent could correctly describe my goal and still pursue something else.
> I found it useful to distinguish understanding a goal from choosing the
> criterion of success that directs the work.

The training hypothesis now says **“adopt a criterion”**, replacing **“infer a
criterion”**. The paragraph containing the annotated claim begins:

> The practical requirement is sufficient overlap: satisfying the criterion
> an agent pursues must also produce a useful end result for me. The goals
> need not coincide. A clear objective makes completion evaluable: what
> result is needed, why it is needed, which constraints matter, and what would
> leave the task unfinished.

For annotation 4, the replacement is:

> I found it useful to think of prompts as both cueing learned behavior
> through their wording and supplying context for deliberate reasoning.

For annotation 9, the addition is:

> The same problem arose with forward and backward references across several
> pages. An agent with those pages in context could follow the references,
> while a human reader had to remember deferred points or interrupt the
> argument to recover an earlier definition.

These are the substantive additions; the other changes are the mapped
deletions and the consistency corrections under annotation 2. Unmarked text
is preserved to reduce new review work, not treated as previously approved.
