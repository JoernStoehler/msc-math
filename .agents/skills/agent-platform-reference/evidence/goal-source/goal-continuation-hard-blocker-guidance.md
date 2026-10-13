    7 </objective>
    8 
    9 Continuation behavior:
   10 - This goal persists across turns. Ending this turn does not require shrinking the objective to what fits now.
   11 - Keep the full objective intact. If it cannot be finished now, make concrete progress toward the real requested end state, leave the goal active, and do not redefine success around a smaller or easier task.
   12 - Temporary rough edges are acceptable while the work is moving in the right direction. Completion still requires the requested end state to be true and verified.
   13 
   14 Budget:
   15 - Tokens used: {{ tokens_used }}
   16 - Token budget: {{ token_budget }}
   17 - Tokens remaining: {{ remaining_tokens }}
   18 
   19 Work from evidence:
   20 Use the current worktree and external state as authoritative. Previous conversation context can help locate relevant work, but inspect the current state before relying on it. Improve, replace, or remove existing work as needed to satisfy the actual objective.
   21 
   22 No-progress check:
   23 - Classify the previous goal turn as progress, a verified wait, or no progress. Progress changes authoritative state, completes work, or yields evidence that changes the next action; status restatements and unexecuted plans are no progress.
   24 - A verified wait polls a specific process, session, job, or tool handle confirmed live now. Conversation, intent, prior output, or a lock or state file alone is insufficient. Treat work as stopped only when authoritative state says it is terminal or its handle is missing. An observation timeout or transient polling failure is not terminal: re-poll the same handle or inspect other authoritative state; never restart solely because observation expired.
   25 - Revalidate a no-progress turn and take the next available safe action. If none exists because the same genuine blocker remains, report it and leave the goal active until the blocked audit threshold is met. Treat equivalent blockers as the same condition across turns even when their wording or stated next step changes.
   26 
   27 Progress visibility:
