# Skill design notes

## Companion design memory, 5 October 2026

**Accepted request:** Jörn asked to add a place for agents maintaining an individual skill to retain incident references, non-obvious writeup decisions, alternatives and other useful design context. The requested role is companion memory for the skill's design, not only a human-facing explanation.

**Choice:** Use optional `references/design-notes.md`, linked for consequential maintenance. `commentary.md` is a reasonable existing equivalent, but can imply a human-only companion; `meta-comments.md` says little about the content; `adr.md` suggests formal decision records and does not naturally include tentative notes or live alternatives. No formal schema or file-per-decision is required.

**Boundary:** The companion must not become an execution-policy owner or a compulsory history log. Constraints needed during skill use remain in operational guidance. Evidence stays with its source; the companion retains its relevance and design implications. Existing equivalent filenames need not be migrated just for consistency.

**Evidence limit:** This convention follows Jörn's maintenance request. No forward trial has established better agent decisions, lower maintenance cost or reliable conditional reading. Ordinary invocation unnecessarily loading these notes, or maintainers missing consequential context, would be reasons to revise the encounter wording.

## Stable name and revision metadata, 5 October 2026

Jörn requested renaming `harness-engineering-v2` and moving the revision out of its identity. The replacement is `harness-engineering` with `metadata.version: "2"`, retaining the previous revision label without inventing a release history. The [Agent Skills specification](https://agentskills.io/specification) permits string-valued metadata and demonstrates a nested version; [Claude's compatibility guidance](https://code.claude.com/docs/en/skills#using-skill-frontmatter-outside-claude-code) distinguishes that common schema from Claude Code extensions. [OpenAI skill guidance](https://learn.chatgpt.com/docs/build-skills) uses the same skill-package core. These sources were checked on 5 October 2026.

Separately named variants remain an option for deliberately comparable treatments. No variants or evaluation programme were selected here. Source rename and project-local activation do not imply that every installed plugin cache has updated.

## Quality baseline and decomposition, 6 October 2026

Jörn clarified his design heuristic: first write one complete related skill and establish useful behavior, then split it if cost reduction warrants that change. Section decomposition and placement into files are separate decisions; parallel trees are not inherently useful. Both quality and cost matter, but substantial losses in results usually need substantial cost savings to be worthwhile. Preserving useful quality while reducing cost is often easier than restoring capability to an aggressively minimized design. These are accepted design preferences/rationale, not measured universal laws or a claim that the current files are near-optimal.

The communication rollout exposed a review gap: technical accuracy and link/discovery checks were used without assessing the added retrieval/selection burden or maintenance cost of runtime references. Guidance now asks decomposition reviews to compare concrete usage and maintenance effects, report their actual scope, and seek knowledgeable judgment or bounded probes when it can materially affect the design. It does not introduce a mandatory expensive trial or approval gate for every edit. Maintenance-only design memory remains a distinct consumer and does not require ordinary users to follow another execution hop.

## Contextual expert advice, 6 October 2026

Jörn asked whether older situational advice is retained as evidence that can make an idea worth considering, followed by evaluation that promotes, retains or drops it for the current problem. The runtime heuristic now makes that transfer explicit. It preserves the difference between defeasible efficacy advice and authoritative instructions/preferences. A partial evaluation does not necessarily screen off everything the expert judgment supplied; the endorsement and evaluation must not be double counted. This is his articulated reasoning and a design synthesis, not reconstruction of his tacit judgment model or a measured optimum.

## Product-reference ownership, 13 October 2026

Jörn authorized considering whether harness-engineering should be subsumed into
agent-platform-reference, while noting that a specialized design trigger may
remain valuable. Retained harness-engineering's design/maintenance/evaluation
workflow and trigger; replaced its duplicated product-loading account with
conditional links to the new product reference. Its existing source URLs remain
a fallback when the sibling package is unavailable. Jörn's bootstrap/retrieval
advice is attributed in that reference, alongside vendor mechanics and limits.
No broad skill retirement, external deployment or behavioral improvement is
claimed. Link and structural checks cover this source change only.
