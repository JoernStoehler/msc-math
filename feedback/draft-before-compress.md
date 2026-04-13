# Draft before compressing

Session: 2026-04-09, RESULTS.md revision

## Incident

When writing the "How this file works" section for RESULTS.md, the agent tried to write compressed text directly. This produced misleading output — ambiguous phrases, implicit claims that weren't intended, missing qualifications. Multiple rounds of Jörn corrections were needed.

When told to "write what you mean first," the agent wrote a longer version but still half-compressed, not a true draft. The agent didn't understand the purpose of draft → prune → compress: the draft phase lets you optimize for correctness without the conciseness constraint interfering. Conciseness actively degrades the agent's writing quality when applied simultaneously with getting the content right.

## Actionable

When writing non-trivial text (process descriptions, documentation, analysis):
1. **Draft**: Write the full meaning without worrying about length. Be redundant where helpful. Spell out examples. The goal is to get every claim right.
2. **Prune**: Remove wrong or redundant parts.
3. **Compress**: Shorten what remains, checking that each compression preserves meaning.

Do NOT skip to step 3. The draft phase is where correctness lives.
