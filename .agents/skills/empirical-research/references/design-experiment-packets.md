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
