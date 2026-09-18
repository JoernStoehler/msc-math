# Human prose review exchange

Any agent working directly with Jörn can use the [human-prose-review skill](../../../.agents/skills/human-prose-review/SKILL.md): provide an annotation link, then fetch the feedback when he says done. A dedicated desk is not required. The inbox below supports coordinated batches when several producers are involved; an arrival does not justify interrupting him. No response means pending, not acceptance.

## Inbox contract

Shared inbox (write to this absolute location, even from another worktree):
`/workspaces/msc-math/.worktrees/prose-review-desk/experiments/writing-quality/human-review/inbox/`.

Each producer creates one uniquely named UTF-8 JSON request per review, `<date>-<producer>-<slug>.json`. Use the desk's existing request as an example. Required fields:

- `id`, matching the filename without `.json`.
- `producer`: name and durable thread identity; live pane if available.
- `artifact`: exact `path` and `sha256` of the text to review. Paths are relative to this worktree, or absolute for external artifacts. Retain immutable bytes; do not point only to a moving branch. Include a commit when useful.
- `context`: list of supporting artifacts with paths and hashes; explain necessary reading context in the packet itself. Empty is allowed when the packet is self-contained.
- `question`, `consequence`: the judgment sought and what it will change.
- `provenance`: source/version, writing operation, selection rationale and known feedback exposure. Keep methods and AI verdicts outside the reader packet unless needed for the task.
- `estimated_effort`: honest reader time and expected response effort.
- `status`: initially `queued`; desk owns subsequent updates (`offered`, `answered`, `deferred`, `withdrawn`).
- `response_path`: `experiments/writing-quality/human-review/responses/<id>.md`.

Keep requests small and artifacts available to the desk. Producers own their request until offered; after that, submit revisions under a new ID rather than silently replacing reviewed bytes. Producers may send batched wakeups through Herdr; Jörn need not relay messages. The desk verifies artifact hashes before presentation. The inbox is a queue, not a commitment that every item will be reviewed.

## Returning judgments

The desk creates the response file only after actual feedback. It records the request ID, packet hash, exactly what was shown (including chat framing and any extra context), date and available message identity, Jörn's exact words, and their scope. Record feedback exposure as independent, assisted, or uncertain, with specifics; a follow-up to previously discussed objections is not a fresh blind test. Separate any desk interpretation and resulting research decision under their own headings. Preserve later corrections without overwriting the original judgment.

Brief verdicts, quoted trouble spots, broader comments, and requests for context all work. Explanations are optional unless a concrete decision requires one. Chapter PASS can coexist with local objections; unmarked text remains unjudged. A local verdict never becomes chapter acceptance. Do not coerce qualified responses into binary labels.

After recording feedback, mark the request `answered`, commit coherent desk-owned changes, and send the producer a meaningful decision summary plus response path through provenance-preserving Herdr messaging. Do not fabricate response files or labels before Jörn answers.
