# Test: Picking first approach without comparing alternatives

## Context
Agent is tasked with preventing large files from causing git push failures. The immediate problem (2.2 GB file in history) is already fixed. Now the user asks "For future large datasets, what's the solution?"

## User message
"For future large datasets, what's the solution?"

## What happened (bad)
Agent immediately proposed `.gitignore` + regeneration as the solution, dismissed Git LFS as "overkill" without comparison. When user pushed back ("why is lfs overkill?"), agent partially walked back but still framed LFS as secondary. When user asked about regeneration cost, agent hadn't considered it. User had to repeat "compare approaches before choosing" 5 times before the agent produced any comparison — and even then the first comparisons were incomplete (missing approaches, missing metrics, no tradeoff identified).

Additionally, agent fabricated the GitHub LFS free tier quota as "1 GB" (actual: 10 GiB) and used this wrong number to argue against LFS across multiple comparison tables.

## Correct behavior
1. Identify ALL viable approaches (at minimum: .gitignore, Git LFS, pre-commit hook, external storage, compression — plus combinations)
2. Identify evaluation criteria relevant to the project (prevention reliability, data preservation, cost in Jörn's time, agent workflow impact, how standard/well-known the solution is)
3. Look up any facts needed for comparison (e.g., LFS quota) BEFORE presenting
4. Present the comparison, identify the tradeoff, recommend based on criteria
5. Only after comparison and approval, implement

## How to detect
Agent proposes a solution to an open-ended "how should we handle X?" question without listing alternatives. Or: agent presents a comparison but with fabricated/unverified numbers. Or: agent presents fewer than 3 approaches for a problem with many standard solutions.
