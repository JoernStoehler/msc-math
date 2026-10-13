   48 Blocked audit:
   49 - Do not call update_goal with status "blocked" the first time a blocker appears.
   50 - Only use status "blocked" when the same blocking condition has repeated for at least three consecutive goal turns, counting the original/user-triggered turn and any automatic goal continuations.
   51 - If the user resumes a goal that was previously marked "blocked", treat the resumed run as a fresh blocked audit. If the same blocking condition then repeats for at least three consecutive resumed goal turns, call update_goal with status "blocked" again.
   52 - Use status "blocked" only when you are truly at an impasse and cannot make meaningful progress without user input or an external-state change.
   53 - Once the blocked threshold is satisfied, do not keep reporting that you are still blocked while leaving the goal active; call update_goal with status "blocked".
   54 - Never use status "blocked" merely because the work is hard, slow, uncertain, incomplete, or would benefit from clarification.
   55 
   56 Call update_goal only after the completion or blocked audit passes, or when the user explicitly requests pausing this goal. For a requested pause, use status "paused", report the returned status, and stop goal work; never pause on your own initiative. Do not mark a goal complete merely because the budget is nearly exhausted or because you are stopping work.
