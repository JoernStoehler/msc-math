# Blind review of the requested repairs

Scope: I read only `blind-packet.md`, `hko-baseline.txt`, and `ds-baseline.txt`. This is a comparison of the supplied prose and baseline meanings, not an audit of the underlying computations, surrounding exposition, or rendered pages. I did not use author identities or methods. PDF extraction artifacts in the baselines are not treated as writing defects. No human acceptance or general fluency score is inferred.

## HKO

| Candidate | Geometry before coordinate specification | Normal explicitly identified | Why outside factor variations | U versus capacity |
|---|---|---|---|---|
| J | Yes: starts from the product and names the normal component before giving h | Yes, including the half-space representation | Yes: factor changes keep q-normals in the q-plane; this adds p1 | Explicitly calls the coefficient a derivative of the feasible upper bound U, not by itself a capacity derivative |
| K | No: starts with h2 and only then says what it varies | No: “coordinate of the second q-facet” leaves the object unclear | Merely asserts that the larger calculation is stronger | No local distinction |
| L | Yes: first names the normal component, then gives coordinates | Yes | Yes, with the same plane/component explanation as J | Explicitly distinguishes the upper bound U from actual capacity |

**J.** The sentence introducing `K(a)={x:a_i·x≤1}` makes the connection between the geometric object and `h_2` particularly explicit: a reader need not guess whether h moves a point, a support height, or a normal. The conclusion replaces the baseline's unsupported “stronger” with the concrete direction that factor changes cannot detect. It preserves the selected index, vector, coordinate order, flat storage coordinate 6, definition of w, and approximate coefficient −0.1801707325. The added explanation correctly limits the claim to U. The half-space representation is added mathematical setup; it is consistent with the intended normal perturbation, but its exact convention cannot be independently checked from the two baseline prose slots and displayed derivative calculation alone.

Strongest remaining objection: “first momentum component of the second q-facet normal” still asks the reader to track several indices before seeing why this direction is useful. “Upper-branch row” and the verifier's storage coordinate remain unexplained local machinery. These are remaining accessibility problems, not contradictions of the repair. The opening identifies geometry before coordinates, although the actual contrast with product variations comes only after the calculation.

**L.** The normal is named in the first sentence, and the final two sentences give a sufficient geometric reason for leaving the product family. All numerical and coordinate information in the baseline prose is retained. The upper-bound caveat preserves the intended distinction without claiming differentiability of capacity. Compared with J, the opening omits the extra K(a) convention and is shorter, but relies more heavily on surrounding definitions to connect h with normal data. “Actual coefficient” does little explanatory work and sits next to “actual capacity,” creating avoidable verbal emphasis.

Strongest remaining objection: a reader who does not already know what h parameterizes gets less help than in J. Like J, L leaves branch/row and storage terminology in the foreground and delays the reason for choosing the direction until the conclusion.

**K.** The vector and coefficient are preserved, but this candidate retains the baseline's local deficiencies. “Momentum coordinate of the second q-facet” does not identify the normal; a facet is not itself a coordinate vector. The assertion that forty coordinates are “stronger” supplies neither the excluded direction nor the implication's limits. It also leaves the derivative's object implicit, despite the nearby DU formula. The formula might help an informed reader recover the meaning, but it does not perform the requested prose clarification.

Strongest remaining objection: K invites precisely the ambiguity the repair was meant to remove—what is moving and what the derivative establishes.

**Targeted repair versus readiness.** J and L both satisfy the four stated local criteria; K does not. J supplies more explicit object-to-coordinate linkage; L is adequate when that setup is already clear nearby. The packet does not establish whether the inserted K(a) notation in J duplicates or conflicts with the surrounding chapter. Neither successful local repair demonstrates that the full derivation is reader-ready or mathematically verified.

## Data science

**M.** The opening now gives the requested route: varied random polytopes and measurements, statistical patterns, geometric interpretation, then tentative conjectures and questions for rigorous explanation. This purpose precedes the sampling distribution. “Can then suggest” does not claim conjectures or proofs were achieved. The later sentence that selection did not supply a rigorous explanation reinforces this boundary.

The bulk of the empirical exposition remains in baseline order. It preserves the ratio and descriptor formulas, dilation and symplectic invariances, sampling law and rejection condition, numerical-volume caveat, within-side-count association, and fresh-candidate selection result. It also preserves all support-height and rotation counts and numerical outcomes listed below. The added headings make the two interventions easier to locate.

Strongest remaining objection: the discovery route is mostly an opening statement layered onto the old sequence. The reader still passes through the sampling specification and full descriptor construction before encountering the observed association. No concrete bridge explains why the symplectic-area geometry might produce that association. It would be wrong to invent such a bridge, but the text should be understood as introducing a research route rather than completing a geometric explanation.

**N.** The opening clearly states a search purpose before distribution details, and distinguishes selecting bodies from changing them. The subsequent association-to-selection passage is coherent and empirically cautious. However, it does not state the requested broader sequence from diverse measurements through statistical patterns to geometric interpretation, tentative conjectures, and possible rigorous explanation. Its account is primarily of two search tactics. This is a substantive missing element under the supplied request, even though the baseline already contains a reasonable local narrative.

No empirical additions or losses relative to the baseline are apparent. Strongest remaining objection: a reader is not told how these computational observations belong to a discovery process aimed at mathematical understanding, beyond their use in finding better examples.

**P.** After defining the objective and its scale invariance, P states the full requested route and explicitly says that the reported experiments reach its earlier stages. This is a useful boundary: the reader learns both the intended destination and the actual accomplishment before distribution details. It then gives the descriptor and association, followed by the descriptor's geometric interpretation and selection test. That order makes the statistical observation available before discussing why its geometry is interesting. The support-height and rotation sections preserve the distinction between choosing a new body and changing an existing one.

The final paragraph accurately collects the three reported outcomes and denies a general improvement rule or rigorous explanation of the association. It does not invent a conjecture or a proof. Its final rotation conclusion additionally specifies “these four starting bodies and prescribed rotations,” a faithful narrowing of the existing evidence.

Strongest remaining objection: “The geometry of S makes this pattern worth examining” introduces invariance and symplectic-area facts, not an explanation of the association. The limits are correctly acknowledged, but the reader still lacks a concrete conjecture-generating interpretation. The concluding recap also repeats findings already stated at section ends; it is useful for the evidence boundary but adds length.

### Preservation and additions

All three DS candidates retain the substantive empirical inventory in the supplied baseline:

- Independent uniform angles; independent heights in [0.8,1.2); boundedness and every proposed side present; numerical volume caveat despite bounded capacity error.
- Lower S associated with higher ratios, including within fixed side counts; low-ridge selection could improve fresh-candidate ratios relative to controls.
- Sixteen polygon pairs, eight 4×4 and eight 4×6; four products per pair; joint validity selected before ratios; factor area normalization preserves the mathematical ratio.
- Mean increase 0.0153; eleven improved and five worsened; subgroup means about 0.004 and 0.027.
- Sixteen evaluations per allocation; one source plus one symplectic control plus fourteen prescribed rotations versus sixteen fresh products with a shared first body; four comparisons split evenly between 3×3 and 4×4; choices fixed before evaluation.
- Symplectic control agreement within 4·10^-16; rotation improved three starts but lost all four maxima comparisons; example 0.328 to 0.529 versus 0.774; rotation took longer.

M and P add the requested broader methodological account, including symplectic and non-symplectic measurements and a route toward conjectures. Those historical details are not independently evidenced by the supplied baseline, which mentions geometric quantities more generally. They follow the supplied repair brief, but baseline comparison alone cannot verify the broader research history. Neither candidate upgrades the reported results into a theorem or an accomplished conjecture.

**Targeted repair versus readiness.** M and P supply the missing route; P integrates it more consistently into the exposition and makes the boundary between research ambition and reported achievements particularly explicit. N preserves the facts but does not deliver that requested addition. On these criteria P is the strongest DS repair, with M also a meaningful repair. None of these judgments establishes overall thesis readiness: the underlying association, measurements, sampling provenance, and computations remain outside this bounded review.

## Exact review prompt

> Bounded independent review (~4min). Workdir /workspaces/msc-math/.worktrees/review-workflow-design. Own ONLY experiments/writing-quality/runs/20260918-repair/blind-review.md. Not alone preserve others. Read ONLY blind-packet.md hko-baseline.txt ds-baseline.txt in that directory; do not inspect other experiment files or provenance/review files. Compare anonymized HKO J/K/L and DS M/N/P; unchanged baseline may be included and may be best. No author identities or methods supplied. HKO requested criteria: geometry before coordinate specification, explicitly identify normal rather than point, explain why adding p1 to q-facet normal lies outside changing polygon factors, preserve derivative of feasible upper bound U distinction from actual capacity. Assess concrete criterion satisfaction and mathematical preservation, residual confusing/distracting prose; no required preference. DS criteria: coherent discovery route from varied polytopes and measurements to statistical patterns to geometric interpretation, tentative conjectures and potential rigorous explanation; put purpose before distribution details; preserve actual reported empirical accomplishments/numbers/caveats without inventing conjectures/proofs. Give candidate-specific evidence, any fact losses/additions, and strongest remaining objection. Separate targeted repair from overall readiness. Do NOT infer human PASS or score generic fluency. Record exact prompt at end. No commits. Parent will preserve report and commit bundle.
