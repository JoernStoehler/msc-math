# Design Experiment Packets

Use this when turning an open question, observation, hypothesis, anomaly, or
method idea into bounded empirical work. The packet should make a research
decision easier; it need not exhaust a method family or complete a checklist.

## Start From The Decision

Establish the current scientific question, the downstream thesis or research
decision, and the evidence already available. Run cheap queries or calculations
when they settle premises needed to design the packet. Do not report an easily
checked uncertainty to Jörn instead of checking it.

Compare serious alternatives before selecting work. Keep separate:

- plausibility or evidential support for a hypothesis;
- possible value if a hypothesis or result is true;
- information value of the proposed packet's outcome branches;
- implementation, compute, review, and attention cost;
- dependencies and option value for other research lines.

The first product of research is a decision-relevant observation and its
interpretation: what was learned and what should be prioritized next. Use KISS
and YAGNI to build the cheapest trustworthy path to that update. After a smoke
or feasibility result, interpret it before hardening the apparatus. Keep
reusable infrastructure, generalization, presentation polish, and exhaustive
validation separate; defer them until the result is valuable enough or a named
consumer requires them. Deferred work must not block interpretation, portfolio
return, or unrelated research progress. Do not defer the correctness, safety,
accounting, freeze, or provenance needed for the observation to be trustworthy.

Treat the scientific method and its implementation architecture as separate
choices. Before building custom infrastructure for a standard or near-standard
method, identify the nearest standard formulation and the thinnest end-to-end
composition of trusted project components. Timebox that vertical spike when it
can cheaply reveal implementation cost, readability, or a real incompatibility;
include the tests and self-review needed for the comparison to be informative.
Compare custom work against this baseline before starting a production/review
cascade. Omit the spike only for a concrete source-backed reason, not because a
more elaborate design is already available or appears more complete.

Numerical probabilities, value estimates, and intervals are useful when they
transmit a real belief or reveal a crux. Prefer rough explicit distributions to
an unlabeled `high/medium/low`; do not spend effort distinguishing near-equal
low-value options when broader search may reveal a better one.

When source inspection or a feasibility measurement has not happened, name the
unknown cost components and the cheapest way to estimate them. Do not invent
agent-hours, compute, savings, or success probabilities merely because a packet
comparison would eventually benefit from numbers.

## Specify The Empirical Object

Name the mathematical/domain quantity, population or generator, sampling and
selection operations, transformations, target timing, controls, and comparison
that the packet measures. State which fields are post-target, target-derived,
or otherwise unable to support independent prediction or mechanism claims.

For each material outcome branch, state the predicted observation under the
competing explanations and what belief, next experiment, proof target, or stop
decision would change. Include plausible confusing outcomes; `fails` is not a
single outcome when different failure modes have different value.

Choose scale and method from the smallest evidence that distinguishes the live
branches. Existing data, a shell query, a tiny smoke run, or a feasibility mock
may dominate a new dataset or full implementation. Conversely, do not reject
new data or methods merely because an older control document declared the line
closed.

## Packet Contract

Before material execution, make clear:

- owner files and source inputs;
- producer/analyzer and expected artifacts;
- smoke and full-run cost where materially different;
- source identity, randomness, resume, and regeneration boundaries;
- stopping conditions and resource-expansion gate;
- interpretation allowed and prohibited from each outcome;
- the transition where fresh review is worth its likely error reduction.

Split packets when their questions or update cycles interfere. Combine work
when shared setup, evidence, or interpretation makes that cheaper and does not
hide an independently failing dimension. A dormant plan, feasible method, or
available dataset is not an execution queue.

When one executor owns a coherent implementation surface with substantial
shared setup, give it a ranked wishlist and a total resource envelope. Separate
fixed setup cost from marginal items, ask for the largest coherent subset that
fits, and require explicit deferrals. Do not batch items whose independent
failure, target timing, or review boundary must remain visible.

Before the first irreversible target evaluation, freeze the evaluator that will
actually run. Prefer a clean committed source and build state. Otherwise retain
an immutable snapshot of every required source, dependency, and build input.
Record repo-relative source identities, revision and lockfile state, input
hashes, and the evaluator source or executable hash before exposure. A hash of
mutable or unretained source is not self-contained provenance.
