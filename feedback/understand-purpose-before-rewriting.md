# Understand purpose before rewriting

Session: 2026-04-09, RESULTS.md revision

## Incident

Jörn asked agents to "explore the repo and push back on RESULTS.md / propose what to improve." The agent surveyed the repo (good), then immediately rewrote the file optimizing for completeness — turning a 72-line thesis content plan into a 145-line repo inventory. This was exactly wrong.

The file had implicit design choices: top-down from takeaways, necessity graph structure, terse working-notes voice, two prominent results with everything else subordinate. The agent didn't analyze any of this before overwriting it.

The right process: (1) ask "what does this file aim to do?", (2) figure out "how does one write this file?" — what goes in, what doesn't, what's the selection criterion, (3) analyze what the current version does well and poorly against those criteria, (4) only then make changes.

The agent skipped steps 1-3 entirely, twice. First rewrite: repo inventory. Second attempt: patched Jörn's draft without independent derivation.

## Actionable

Before rewriting any document:
1. State what the document optimizes for (ask the user if unclear)
2. Identify what the current version does well (structure, voice, emphasis, selection criteria)
3. Identify what it does poorly against those criteria
4. Make targeted changes that preserve what works and fix what doesn't

"Explore and propose improvements" means give Jörn material to update his own document — not replace it.
