# Draft launch prompt: HKO local maximality with one additional facet

Status: optional research alternative, not the first recommendation under the
reported writeup-first priority; not selected or launched. This file is the text to give
the execution agent once Jörn selects this package; it is not a standing repo
instruction. It does not authorize execution merely by existing.

## Prompt

Investigate whether the thesis's HKO ten-facet local-maximality theorem extends
to nearby polytopes with at most eleven facets. Focus on the two residual
single-cut cases: the added normal initially exposes a two-dimensional face or
a three-dimensional facet. Produce a checked mathematical result worth retaining,
with five hours maximum elapsed time from launch.

The intended value is to extend the central theorem beyond its fixed-facet
restriction, or settle one consequential missing case. This is optional research,
not a required submission repair. A new plot, another failed numerical search,
or a restatement of the known gap does not meet that aim. Completion in five
hours is uncertain; do not invent a theorem to meet the deadline.

Start from these exact owners:

- `thesis/chapters/07-hko.tex` and its three `hko-lemma-*.tex` inputs: the
  selected ten-facet statement, uniform transverse deficit, and symmetry chart.
- `experiments/hko-local-maximum/theorem/README.md`: exact finite certificate.
- `docs/empirical-viterbo-design/desk/pentagon-arbitrary-partner/local-cut-lifting.md`:
  the coupled vertex/edge cases and the two unresolved higher-dimensional cases.
- `docs/consolidation/2026-09-24/claim-disposition.md`: L11 remains open. The
  returned source ZIP is located through that directory's README.
- `docs/open-thesis-literature/extrema.md` and Haim–Kislev,
  https://arxiv.org/html/2511.16644v1, Proposition 1.13 and its appendix proof.

Compare two approaches before committing the run: extend the existing feasible
HK-section/insertion argument; or exploit near-boundary cuts additivity and
derive a quantitative cap-capacity bound. Use another approach if it has a
better mathematical reason. The selected source already proves the ten-facet
theorem: repeating it is not the assignment.

There is preliminary traction, which you must check independently. Proposition
1.13 appears to cover every exact base-HKO normal exposing a two-face or facet:
pure-factor normals are explicit, and an edge normal belongs to both endpoint
cones, permitting paired endpoint indices differing by 0 or ±1 modulo five.
This is our deduction, not a theorem stated in the paper. It neither gives an
open neighborhood of each normal nor applies automatically after old facets move.
Check the paper's factor order, rotation and symplectic-sign conventions against
the project before transferring formulas.

For a removed cap C and retained body R of the fixed HKO body, cuts additivity
would give c(R)=c0-c(C). The required ratio inequality is then equivalent to

    c(C)/c0 >= 1 - sqrt(1 - Vol(C)/V0).

Check this reduction and derive the relevant coefficients or stronger bounds.
The existing insertion estimate gives only quadratic capacity loss. Two-face
caps have quadratic volume loss, requiring coefficient control; facet caps have
linear volume loss. Capacity loss alone is insufficient. Fixed-direction or
fixed-body estimates also do not prove a neighborhood theorem: cut depth, normal
direction and old-normal displacement can vanish at different rates. Keep the
zero-slope cone and symmetry quotient explicit.

A full at-most-eleven-facet Hausdorff theorem also needs the representation
bridge: every relevant nearby body must admit ten labelled rows converging to
the HKO rows and at most one bounded extra normalized row. Justify the exclusion
of fewer than ten facets nearby, and classify redundant interior versus boundary
limits of the extra row. The paper's direction-dependent near-boundary allowance
must become a uniform bound before a compactness argument can be used.

Own a new packet under `formal/hko-one-cut-extension/`. Preserve other sessions'
work and producer-owned retained evidence. The active manuscript is an input;
thesis integration is a later selection. Use the included Codex allowance and
installed local tools; paid APIs, remote compute, external messages and publication
are outside this package. Inspect producer write effects before running anything.

If parallel Codex work is included in the launch approval, use at most four
concurrent workers plus the coordinator. Give disjoint owned outputs for the
facet case, two-face case, exact computations/source reconstruction, and an
independent alternative or challenge. Brief each with scope, allowed resources,
deadline, receiving owner and the fact that others are editing. Coordinator owns
integration, supervision and final review. Without that approval, work serially;
the mathematical task is still executable.

During the first approximately 30 minutes, check inputs and tool availability,
identify the strongest route and divide the unresolved obligations. Continue
without human questions within the selected scope. If an input cannot be
recovered, derive a replacement or return its exact mathematical dependency;
do not expand into a different programme or claim that an unsupported input was
verified. Reassess a route when its obstruction becomes concrete. Persist useful
derivations and witnesses throughout; five hours is a cap, not a spending target.
Check the clock at launch and checkpoints; do not start computation that consumes
the reserved review period.

Reserve approximately the final hour for mathematical challenge and integration.
Have a reviewer or a separate derivation check the strongest claim against
boundary normals, coupled scales, symmetry directions, inequality directions,
and exact/numerical trust boundaries. A majority of agreeing reviews is not a
proof. Retain unresolved objections. Run the exact checks appropriate to any
finite claim, into fresh owned outputs; random sampling cannot certify a local
neighborhood.

Return `RESULT.md` with the strongest exact statement, proof and dependencies;
scripts and recoverable witnesses supporting finite checks; the disposition of
all four exposed-face dimensions; and a short review guide identifying what Jörn
would need to judge later. State whether the result concerns fixed-body cuts,
coupled old-normal movement, or a full Hausdorff neighborhood. Link substantive
failed routes and their obstruction without turning the result into an activity
log. A complete uniform residual-case theorem or an exact obstruction that
changes the viable proof route is useful partial progress. Closing a residual
case means a uniform coupled neighborhood, including subface approaches; a
fixed-body cut theorem keeps its narrower name. An obstruction must refute a
substantive candidate implication with a recoverable witness or impossibility
proof and explain its consequence; the already known quadratic/linear mismatch
does not qualify. Name the exact stronger theorem now available, the remaining
L11 obligations, and whether another research run is needed before integration.
If neither substantive outcome is found,
report that outcome plainly. No further large run or immediate thesis rewriting
may be a hidden dependency of the reported result.

## Selection rationale and limits

The fixed-ten-facet restriction belongs to the central HKO theorem, so closing
one of its residual extension cases has more direct mathematical consequence
than the earlier saturation-pattern proposal. The new approach combines a
specific source theorem with the project's existing sharp deficit and cap-volume
argument, rather than launching a larger sampler. Abundant reasoning can support
competing derivations and adversarial checking of singular coupled limits.

There is no external deadline making this theorem urgent, no established success
probability, and no evidence that mathematical human review will be cheap. The
timing case is the temporary capacity for a serious proof attempt. It does not
override the separately reported writeup-first priority: selecting this package
would choose a bounded research investment alongside that priority, not establish
that new research is necessary to finish the thesis.

The strongest writing rival is source-grounded reconstruction and repair of
already selected mathematical arguments, not broad stylistic rewriting. The
closest audited DS rewrite failed human review, but that does not establish
lower value for proof reconstruction. An important repair may finish early;
several repairs can form a package without needing to consume all five hours.
See `five-hour-thesis-proof-prompt.md` for the first recommendation under the
reported writeup-first priority.
The global pentagon-cover problem has less established reduction to manageable
residual cases. These comparisons support this research candidate; they do not
prove it is Jörn's preferred immediate allocation.
