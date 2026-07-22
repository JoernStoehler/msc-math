# Sys-Datascience Portfolio Review Contract

This owner-local contract supplements `$empirical-research` for the hostile
`sys` search. It records decisions made after repeated anchoring and
scope-narrowing failures. It does not override the skill's role, evidence,
branch, or review conventions and does not reopen parked experiments.

## One Research Cycle

1. Reconstruct the current question and evidence from the closeout, ledger,
   experiment indexes, and owning artifacts.
2. Generate questions and experiment ideas across different objects, regimes,
   data sources, empirical methods, and empirical/theoretical interfaces.
   During this pass, do not deeply operationalize the first executable idea.
3. Assess serious alternatives across the whole portfolio. State the
   intervention, material outcomes, expected belief/decision updates, thesis
   value path, staged cost, failure modes, and smallest useful observation.
4. Let Jörn review the assessed map before a nontrivial experiment whose
   expected shadow API cost is around USD 20 or more, unless its scientific
   scope and budget were already approved. Minor implementation choices that
   plainly preserve the approved question, population, claim, and cost do not
   require another interruption. Re-ask when one of those changes materially.
5. Execute the approved experiment without silently changing its question.
6. After technical review, interpret the observation and update the question,
   hypothesis, and idea map. Do not launch the successor during interpretation.
7. Generate and compare successors against all live lines, discuss them with
   Jörn, and stop after the agreed cycle.

Cheap plots, table joins, repository inspection, and disposable calculations
used to understand existing evidence do not need a separate approval ritual.
They still must not be promoted into claims beyond what they measure.

## Anti-Anchoring Checks

- The most recent, concrete, or easy-to-code idea has no priority for those
  reasons.
- Facet count, product bucket, exact shape, parameter family, and local
  neighborhood are operational restrictions. Each needs a question-,
  information-, feasibility-, or cost-based reason.
- Before calling an idea promising or selecting it, compare it with serious
  alternatives from other live lines and with another bounded idea-generation
  pass. Discovery value is expected to be heavy-tailed.
- Separate breadth of models, invariant features, generators, and search
  objects. Many models on the same narrow feature vocabulary are not broad
  data science.
- A method name or domain restriction is not an experiment. “Use machine
  learning,” “look at low facet counts,” or “study bounce classes” lacks an
  intervention and outcome-dependent decision.
- Existing packets constrain and inform choices; they are not a queue. A
  closed line records a cost comparison and reopening evidence, not an order
  to preserve the ranking after premises change.

## Cost And Deferral

Compare expected total project cost, not only the next command or agent call.
Include scientific validity, implementation, expected repair, interpretation,
review, promotion, future reconstruction, maintenance, critical-path wall
time, shadow API cost, and Jörn attention where they differ between options.

Deferral is beneficial only when waiting reduces expected total project cost.
For every material deferral, record:

- what work is not being done now;
- the avoided current cost;
- the likely cost created by delay, including context loss or evidence decay;
- the event that would make execution preferable;
- the minimum durable state needed to resume without rediscovery; and
- whether an unmerged branch actually provides a reproducible source.

If delay makes the work harder or risks repeating a correctness failure,
finish the bounded task now or explain why another constraint still dominates.
Do not praise a smaller merge, fewer lines of code, or a stopped task without a
downstream cost mechanism.

## Execution And Promotion

The research-line lead should keep question formation, cheap repository/data
inspection, disposable plots, and small standard pilots in its context when
handoff would cost more than execution. Nontrivial delegated execution follows
the general role split and requires a bounded proposal whose context can be
transferred cheaply enough to repay delegation.

Separate four costs:

1. obtaining a scientifically valid observation;
2. interpreting it and updating the portfolio;
3. making its evidence reproducible for future agents;
4. integrating it into durable code or thesis prose.

An observation does not automatically justify all four. Conversely, calling
code throwaway does not permit target leakage, incomplete candidate sets,
missing controls, stale binaries, unrecoverable inputs, or unsupported claims.

For a clean reconstruction, transfer the research question, frozen inputs,
controls, invalid cases, and failure regression tests. When independence is
valuable, initially withhold prototype architecture and expected numeric
answers; compare the reconstruction to the prototype only after its own result
is fixed.

## Review Packet

Organize proposals for Jörn by decision, not by idea chronology. Keep together:

- question, hypothesis, and current evidence;
- intervention, measured object, population, controls, and comparisons;
- material outcomes and predicted likelihoods when useful;
- how each outcome changes beliefs, later experiments, or thesis claims;
- direct thesis value, information/option value, and opportunity cost;
- staged shadow API cost, compute/wall time, and Jörn attention;
- prerequisites, contradictory assumptions, technical failure modes, and stop
  conditions; and
- the comparison with other research lines and with doing nothing now.

Separate generating ideas from derived plumbing so the scientific choice can
be reviewed before implementation details.
