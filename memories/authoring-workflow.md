# Authoring workflow: a concrete starting experiment

The authoring workflow is still being discovered. Jörn's September 2026
discussion makes his full-PDF review the costly final gate, ideally passed on
the first candidate; it does not supply a trusted automated substitute. He
distinguished adaptable feedback during writing from that gate. Detailed
summary infrastructure is deferred until writing and feedback reveal its needs
([decision context](planning-context.md)).

## Knowledge that changes the approach

`docs/project-facts.md` items 40–44 record Jörn's experience: default model prose
is often overconfident, nonsensical or excessively dense, and producing useful
phrasing and deciding what belongs where are much harder than a successful
LaTeX build. Those are attributed observations, not a measurement of today's
model. Legacy prose needs revalidation under changed claims or structure even
if previously approved.

In the live discussion Jörn singled out proof jumps a human reader cannot
fill. The useful distinction is between an invalid intended bridge and a valid
bridge whose exposition needs intermediate steps. Proving the destination by
an unrelated route does not vindicate the written transition. His willingness
to investigate those distinctions is not an offer to repeatedly review whole
drafts during authoring.

One concrete prior preference is recorded in `docs/project-facts.md` item 85:
for the product-QP six-facet proof, explain why the proof is product-specific,
state theorem and algorithm before their proof, and put scope/use caveats after
the main statements. Item 9 also rejects forcing the thesis's different
research questions into one tight narrative. These are more useful starting
points than inventing a universal style checklist.

## Case used for the first authoring comparison

Inspected 2026-09-07 at `921f3c02efefbfb3919fb2c35545686459123fff`:

- `thesis/07-hko-local-maximum-exact-certificate.tex`, paragraph beginning
  “The finite first-order criterion”: positive relation and row rank imply a
  uniform strict decrease on the transverse slice.
- `thesis/07-hko-local-maximum.tex`, theorem
  `thm:hko-ten-facet-local-maximum` and “Proof spine”: the result the reader
  needs that step to establish.
- `thesis/07-hko-local-maximum-chart-reduction.tex`, final slice construction:
  how the slice conclusion returns to the geometric neighborhood and equality
  set.
- `memories/hko-author-context.md`: already-inspected support route and the
  distinction between feasible upper functions and optimizing branches.

This case was selected for workflow development, not because a mathematical
error had been identified. It is small, central to the main theorem and tests the kind of
reader bridge Jörn described. It needs no new experiment, witness search or
full verifier run. The theorem's established status is not in question.

For a reviewer checking this particular implication, the intended reasoning is:
rank and annihilation make the restricted rows span the slice dual; if every
row evaluated nonnegatively on a nonzero direction, the strictly positive zero
relation would force all evaluations to vanish, contradicting spanning.
The finite minimum of these linear evaluations is continuous and negative on
the unit sphere, hence bounded above by a negative constant. The finitely many
smooth upper functions have a uniform first-order remainder small compared
with that margin. Their touching inequalities give strict decrease of the true
ratio. This is the inspection agent's reconstruction, not an independent audit
of the upper-function hypotheses or the surrounding chart proof.

## Reader observation, alternatives and selected revision

On 2026-09-07 a no-history subagent (`sprint_authoring/hko_reader`) read only
the first two TeX files above, without this memory or its supplied reconstruction.
It recovered the implication and identified two fillable but compressed steps:
the all-nonnegative contradiction from the positive relation, and the uniform
neighborhood obtained from the finite minimum on the unit sphere and a common
Taylor remainder. Its reconstruction agreed with the one above. It required
finite-dimensional linear algebra, compactness and differentiability, not
specialized nonsmooth analysis. This is one agent-reader observation, not
evidence that Jörn or the intended human reader would find the passage adequate.

The first reader proposed an expansion: explicitly exclude all-nonnegative row
evaluations; define `m(h) = min_sigma r_sigma(h)` on the unit sphere and give
`m(h) <= -c`; then state a simultaneous remainder bound
`|U_sigma(a_0+s)-U_sigma(a_0)-r_sigma(s)| <= (c/2)||s||`. This provides a concrete
authoring candidate without changing the theorem or rerunning its certificate.

Two no-history authors then proposed alternatives. Variant A ended with an
explicit linear decrease bound using a direction-dependent row; B used the
minimum over the upper functions and a uniform `-ct+o(t)` estimate. Separate
math-meaning and reader-flow reviewers inspected the original and candidate
arguments. The mathematical reviewer found both correct under the existing
smooth touching-upper-function hypotheses and preferred A's explicit bound.
The flow reviewer preferred the original plus the missing sign argument:
the constants and remainder notation in A/B expanded a routine step more than
the reader needed. This disagreement is useful evidence about the tradeoff,
not a correctness vote.

The integrated edit therefore adds only the sign argument after the spanning
sentence. It fixes the shared concrete omission without introducing new
notation. The original compactness/Taylor sentence remains. That selection is
the integrating agent's judgment; whether the intended human reader needs the
more explicit uniformity argument remains untested. Child task names for this
comparison were `sprint_authoring/variant_a`, `variant_b`, `meaning_review`, and
`flow_review`. No build or certificate run was part of the comparison.

Whether the uniformity sentence remains too compressed was left unresolved.
Jörn subsequently rejected further opportunistic passage checks as the next
priority. This trial is reference for workflow discovery, not a queued task;
see [current planning context](planning-context.md#writing-and-review).

The reusable output is the justified revision, if any, and the few observations
that change the next writing decision: where readers got stuck, which repair
helped, or which apparently helpful edit caused loss or distraction. Useful
source/context notes can grow from that work; no fixed summary schema is
needed in advance.

This local attempt cannot establish whole-chapter coherence, an effective
empirical-results workflow, or final-PDF readiness. Those remain open parts of
authoring-workflow discovery, not prerequisites to trying this case. The
comparison above is one source-level authoring attempt, not a human-reader
before/after test or evidence that a whole-thesis workflow succeeds.
