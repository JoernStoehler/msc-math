# Qualifications review: variation

- Start UTC: 2026-09-18 13:06:40 UTC
- End UTC: 2026-09-18 13:06:53 UTC
- Prompt: `/workspaces/msc-math/.worktrees/candidate-narrow-flaggers/docs/candidate-narrow-flaggers/prompts/qualifications-prompt.md`
- Complete reviewed input: `/workspaces/msc-math/.worktrees/candidate-narrow-flaggers/docs/candidate-narrow-flaggers/inputs/variation.tex`
- Supplied context: `/workspaces/msc-math/.worktrees/candidate-narrow-flaggers/docs/candidate-narrow-flaggers/inputs/main.tex`; `/workspaces/msc-math/.worktrees/candidate-narrow-flaggers/docs/candidate-narrow-flaggers/inputs/preamble.tex`

## Concerns, ranked by reader cost

1. **Likely concern — an unintroduced numerical shortcut interrupts the envelope argument.**

   Exact span:

   > The hypothesis preceding \eqref{eq:first-order-regular-envelope} contains real
   > mathematical work.  A list returned by one numerical solve, or a tolerance
   > window around its smallest actions, is not by itself a certificate of the
   > displayed identity.

   The preceding argument has raised the question of how derivatives of a finite minimum follow from smooth branch derivatives. It already states the required neighborhood identity explicitly and gives a positive sufficient condition involving a complete word list and persistent gaps. This paragraph switches to refuting two numerical substitutes that have not yet been proposed. Removing it leaves the envelope formula expressly conditional on coverage; no unconditional inference becomes licensed. The later non-generic-boundary discussion also explicitly distinguishes base-point enumeration from neighborhood coverage.

   **Minimal change:** delete this paragraph here. If needed, relocate its concrete warning to the discussion of the chosen branch window in the search subsection, where a numerical subset actually enters the argument.

   **Context that would make it useful:** a preceding description of a solver returning a tolerance window that the reader might reasonably identify with the complete family in the displayed identity, or a subsequent application claiming that identity from such output.

2. **Uncertain concern — an avoided computational procedure is advertised without a local comparison.**

   Exact span:

   > They do not require finite
   > differences of the full capacity computation.

   The preceding sentences answer the practical question of which quantities the derivative formulas require. Their explicit input list already supplies that answer. This added sentence introduces finite differences as an alternative without discussing their cost, accuracy, or use in search. Its removal does not change any derivative statement or computational dependency. This is uncertain because the subsection does promise a practical first-order calculation, and a reader planning an implementation could appreciate this comparison.

   **Minimal change:** delete this sentence; retain the positive list of required inputs.

   **Context that would make it useful:** an actual comparison with a finite-difference search method, an explanation of why repeated capacity solves dominate its cost, or a discussion distinguishing derivative evaluation from the later finite-difference validation tests.

## Qualifications worth retaining

1. Exact span:

   > A nondegenerate branch always gives the upper function \(U_\sigma\), but it gives the
   > actual systolic ratio only while it realizes the global Haim--Kislev maximum.

   This establishes the essential distinction between differentiating a fixed-word optimum and differentiating the capacity-derived ratio. Without it, the preceding branch derivative could be mistaken for the derivative of the true ratio before global coverage is introduced.

2. Exact span:

   > Thus the evidence supports finite-step
   > progress and a measured computation cost on that panel, not endpoint local
   > maximality.

   This specifies the evidential meaning of the immediately reported experiment. Accepted moves might otherwise suggest completed optimization, whereas the iteration caps and remaining above-threshold moves support progress only. The denied conclusion is locally motivated by precisely those results.

3. Exact span:

   > If a maximizing weight is zero, deleting that coordinate is valid for
   >   the capacity value at the base point.  It need not preserve the nearby
   >   slope: the longer word may acquire a positive weight on one side of the
   >   perturbation and define a different branch there.

   This identifies a concrete failure mechanism at the boundary of the positive-weight hypothesis. Equality of base-point capacity values does not justify identifying their nearby derivatives; the possible support appearance explains why. The qualification contributes mathematical content rather than merely warning that assumptions matter.
