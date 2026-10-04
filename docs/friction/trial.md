# Collaboration trial evidence

Historical evidence imported on 30 September 2026 from the [Pages trial](https://chatgpt.com/space/page_9d9e95b76fbc81919da91c9c8370e5e3). Procedures and proposed next actions below describe the trial at import; they do not assign work or require Jörn to run a test now. Later trial evidence should be appended with dates and the reporting surface. Friction records and maintenance are owned by [README.md](README.md).

This trial tests whether ChatGPT Pages can coordinate Jörn, ChatGPT Chat, Work and Codex, Claude, and subagents across the msc-math project. The [project hub](https://chatgpt.com/space/page_6abcac1aeaf08191b64e957bf7258013) holds the shared work view; this Page records observed friction, workarounds and the evidence needed to decide whether to improve Pages or build a small external plugin.

**Started 30 September 2026. Verdict pending.** Setup exercised reading, source discovery, child creation and guarded editing. Guided test T01 now exercises selection-anchored comments and a fresh-session reply. Comment creation succeeded; independent-session replies, simultaneous claims and Claude access remain untested. C01 passed the fresh-session handoff recovery test; no acknowledgement session is required.

## Trial episodes

| Episode | Test | Pass evidence | Status |
| --- | --- | --- | --- |
| Setup | Build a source-backed hub and durable friction log | Correct links, saved content, visible unknowns, unrelated content preserved | Saved content verified; native visual rendering unverified |
| Join and hand off | A fresh Chat, Work or Codex session reads the hub, registers its scope and leaves a handoff | No repeated project briefing; another session finds the output and next action | Passed: C01 recovered the setup handoff, registered and checked the source without another briefing |
| Comments and authorship | Original Work session posts a selection-anchored comment; fresh session discovers and replies; Jörn responds | Thread is visible from both sessions; human and agent attribution are distinguishable; notification behavior is reported separately | T01 started: comment created and fresh-session prompt updated; external reply pending |
| Comment lifecycle | React, resolve, filter resolved threads, reopen; edit the anchored text and inspect the anchor | Replies/history survive; thread and anchor remain findable; no accidental task launch | Next after T01 reply |
| Human edits and concurrent updates | Jörn edits a small trial field while a session holds an older read; two sessions then update separate and overlapping fields | Fresh reads preserve user changes; stale writes conflict or merge intelligibly | Not tested |
| Discovery and launch | Find hub/subpage/comment; launch Page chat; inspect chosen surface, inherited guidance and any notifications | Low-effort launch and access; no reliance on assumed auto-notification | Prompt link visually confirmed in supplied screenshot; resulting surface/context untested |
| Artifacts and mathematics | Read a linked PDF and try only needed native attachments, notation or embeds | Evidence opens correctly; precise statements remain inspectable; unsupported forms are explicit | Not tested |
| History and recovery | Recover a mistaken edit, compare changes and export a handoff where available | User changes and provenance are recoverable; another session can continue from the exported state | Not tested |
| Share independent work | Two sessions report on different bounded items | Both updates survive; ownership and evidence are clear | Not tested |
| Competing claims | Two sessions attempt to take the same item | Duplicate execution is prevented or detected before work starts | Not tested; a manual ownership row provides no atomic lock |
| Claude participation | Claude reads and updates the same state through an available authorized route | Direct access or a measured, workable export and return path | Not tested |
| Reconcile a change | A source changes or a session stops without a handoff | Stale state becomes visible; source truth and ownership can be recovered | Not tested |

## Open capability questions

These are untested, not observed failures.

- Do all relevant ChatGPT surfaces expose the same Page content, tools and Agent Instructions? Selecting a Page in this session does not prove it is automatically loaded elsewhere.

- Can Claude read and write this shared state directly? No Claude route has been established.

- Do concurrent edits to separate rows of one table cause retries or lost updates? Block hashes guard against stale replacements, but do not supply a task lease.

- Can participants notice source changes and abandoned claims cheaply? No continuous monitoring or schedule is configured.

- Will mathematical notation and PDF evidence be usable enough here? The Page content catalog does not document native math extensions; keep precise mathematical statements in their authoritative sources until tested.

## Decision criteria

Decide after real join/handoff, independent-work, competing-claim and cross-provider episodes, or sooner if a required capability fails.

**Improve Pages and project practice** when links, small native blocks, explicit registration and reusable prompts make coordination cheap and reliable.

**Build an external plugin** when repeated, consequential friction requires atomic ownership, freshness tracking, provider access or structured updates that Page practice cannot supply. Implement only the demonstrated missing operations; keep Pages as a readable view if useful.

The smallest candidate design is a single-user shared store of work items, session reports, questions, decisions, evidence links and friction events. Narrow operations could be read project, claim item, report progress, release or hand off, and append friction. Claims need conflict checks and expiration; records need stable IDs, timestamps, provenance and revision history. Include a compact attention view and portable export. Multiple human accounts, sharing administration and a full document editor are outside this team's stated needs. This is a design candidate, not an implementation commitment.

## Change record

- 30 September 2026: hub and child log created and read back; Page-scoped Agent Instructions and a reusable coordination prompt saved on the hub. Ten initial friction entries recorded; the schema error was resolved by one corrected edit. Setup Work session released its scope. No thesis worker launched, external plugin built, or automation configured. Independent-session, concurrent-claim and Claude tests remain open.

- 30 Sep 2026, C01: fresh-session handoff recovery passed; [published main matched 84d5df3](https://github.com/JoernStoehler/msc-math/compare/84d5df3cf707f2df37cfe35294ecd9c47fc090f9...main) and the thesis pause remained. Task released. Friction: F03, F10, F14.

- 30 September 2026: reorganized all fourteen friction records into unresolved, no longer relevant, and resolved or mitigated sections. Added explicit outcome kinds, evidence and maintenance rules; preserved stable IDs, repeat observations and concurrent trial updates. F11 records a deliberate workflow change and F12 a disappearance with unknown cause; F13 and F14 remain open. Each item now has its own native record.

## Guided walkthrough

**Current step T01:** the original Work session created an open selection-anchored comment on the hub’s session-launch prompt, headed **Pages trial T01 — cross-session comments**. Find the thread by its heading on the hub. The updated prompt asks the fresh session to discover and reply to it.

Jörn’s only next step is to let the new session try. If it does not discover the comment, point it to the thread heading or ask it to list Page comments; record the extra intervention. After its reply, Jörn adds a short human reply or reaction in that same thread. The original Work session then reads the thread to check visibility, author/session attribution and the reply chain. Leave it open until that check.

Next test batches: comment reactions and resolve/reopen; human edits and comment-anchor survival; independent/overlapping session writes; discovery and launch; needed file/PDF/math support; history and recovery; Claude access. Test the user actions with the fewest steps; the agents handle reads, comparisons and logging. Do not claim ongoing monitoring when no turn or automation is running.

## Original session follow up

30 September 2026, original Work session: the fresh Codex Work session C01 registered and added independent source checks and friction findings. Its additions survived our first stale-table conflict after reread and retry. T01’s selection-anchored comment and 👀 reaction are saved; the original selected-text anchor remains in the comment readback after the launch prompt was edited nearby. At the last comment read, no fresh-session or human reply was present.

Additional friction: updating shared trial tables encountered another invalid-arguments rejection, followed by an edit_conflict on a guarded whole-block retry. No change was committed by those calls. The original session switched to this separate evidence section to preserve concurrent contributions and avoid repeatedly touching the same tables. If this repeats, prefer one observation or work record per block, or append-only records with a derived status view. This is observed update ergonomics, not evidence of lost data.

Reaction attribution remains a test question: the write receipt reported count 1 and reactedByMe true but an empty reactors list; it did not identify the reacting session. Comment-message attribution did identify the original agent and conversation.

Next user step: in the new session, request a reply to the hub comment headed Pages trial T01 — cross-session comments. After the agent reply, Jörn adds a short human reply or reaction to that thread. Keep it open until the original session checks the reply chain; then test resolve and reopen.

## Chosen surface split

30 September 2026: Jörn chose Pages for persistent views intended for him and tightly integrated with ChatGPT, and GitHub repository files for durable workspace content maintained collaboratively by agents. Detailed friction tracking moves to this directory; the Page becomes the concise human overview. Evaluate any additional tools against the next best working alternative and total user/agent effort, including setup and maintenance.

## Migration result

30 September 2026: the repository records were committed to main and read back exactly. Literal Page patches published the compact human overview; pre-migration sections are frozen and collapsed by default because structural rewrites were rejected. Root repository guidance, the hub's tracking instruction and prompt, and the friction Page's Agent Instructions now route tracking to the repository and assign overview reconciliation to the agent changing material facts. No monitoring, external session, thesis run or automation was started. Rendered presentation and independent-session upkeep remain untested.
