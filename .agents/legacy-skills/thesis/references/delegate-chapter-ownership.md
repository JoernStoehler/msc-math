# Delegate Chapter Ownership

Use this reference when deciding whether a fresh autonomous session should own
a whole chapter or thesis slice rather than receive a bounded writing subtask.
Use `coordinate-writing-agents.md` for bounded or parallel preparation and
review work.

Whole-slice ownership is plausible when the repository contains discoverable
domain sources, the chapter's thesis role and downstream interface can be
stated, and one session can iterate across mathematics, prose, assets, builds,
and review. Missing session history is then often cheaper than transferring a
large accumulated context. Do not use a chapter owner merely to avoid settling
an unresolved thesis-wide scope or claim-strength decision that would change
the assignment itself.

The integration owner retains cross-chapter dependency order, accepted project
scope, and the decision to merge. Transfer the chapter-local work needed to
reach that gate: source and claim audit, choice among plausible structures,
proof simplification, drafting or replacement, justified figures or tables,
companion maintenance, builds, rendered inspection, and revision after review.
Do not constrain the recipient to a writer role when the desired outcome
requires those activities.

Use `$harness-engineering` for the cold-start prompt. State context before task
details, including:

- what thesis and reader decision the chapter serves;
- what earlier text supplies and what later text must be able to rely on;
- required scope, accepted decisions, and prohibited stronger conclusions;
- primary entry points and the status hierarchy among sources, notes, old
  prose, and developing evidence;
- the returned worktree's downstream use and the review gate it must plausibly
  pass.

A direct long-lived session is not thereby a bounded-subagent assignment. Use
`$subagent-prompting` only for bounded child assignments whose outcome and
ownership are already fixed.

Keep provisional diagnoses and suggested strategies distinguishable from
constraints. Give the owner room to discover a better theorem boundary,
chapter structure, proof route, or presentation medium from the sources.
Define failure at the outcome level: a content inventory, first-pass prose, or
an unreviewed build is not a finished chapter when the assignment is supposed
to return a merge-ready candidate.

When adjacent research is still moving, separate the stable chapter core from
the possible extension. The owner may inspect developing work and preserve an
insertion point or reopen condition without either blocking stable writing or
turning immature observations into thesis claims. Recheck the moving owner
before final review if its state could materially change the chapter.

Treat the first chapter-owner run as a writing-workflow trial. Evaluate the
returned reading surface, mathematical and source calibration, review cost,
and usefulness as a new starting point. Update durable delegation guidance or
model-routing priors only from the observed result, not from the session's
self-description.
