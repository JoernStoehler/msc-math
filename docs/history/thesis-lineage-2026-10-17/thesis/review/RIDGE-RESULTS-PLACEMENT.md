# Ridge results: concrete manuscript choice

17 October 2026. This is an off-manuscript proposal; the selected 105-page
candidate does not contain this paragraph.

## Recommendation and prior answer

**Keep these results in research notes by default.** Jörn said predictor
counterexamples help prune methods but are not useful in the thesis
(pm answer `f5d3dc4f`, 17 October: “i think this isn't useful in the thesis”,
“useful to colelct so 1”).
His more valuable target is a geometric predictor that works on held-out upper
tails and might suggest a maximality conjecture. The construction below now
rules out *all* exact formulas based only on unsigned ridge areas, rather than
just a particular sum or maximum. His answer already anticipated counterexamples generalizing to exclude future
predictors. The broader scope therefore stays within the class he valued for
method pruning; it does not create another inclusion decision.

The batch's end state is a settled, evidence-bound notes-only disposition and
one fewer open question. Recovering his prior avoids re-asking an answered
preference; retaining a short exposition makes the insight usable in research
notes; checking its scope avoids turning an information loss into a claim about
upper-tail prediction.

## Retained short exposition — outside the manuscript

> The complete collection of unsigned symplectic two-face areas need not
> determine capacity. Two explicit Lagrangian products of a hexagon and a
> triangle have the same volume, $330$, identical 27-entry multisets of unsigned
> two-face areas, and the same factor difference bodies, yet their exact
> systolic ratios are $6000/10571$ and $30375/60236$. Thus neither those areas
> nor their combination with directional widths can give a universal exact
> formula for the ratio. The obstruction also applies to weights determined
> solely by the unsigned area values. It does not exclude useful approximate
> prediction or weights using signs, incidence or other geometry. Both ratios
> are below one; no held-out upper-tail test is made.

This exposition accompanies the linked research construction; it is not a
self-contained manuscript proof. A future change of scope would require a
manuscript-local construction/proof reference before integration. No paragraph,
new theorem, proof relocation or appendix is selected for the current thesis.

## Editorial alternatives and choice

| Alternative | Explanatory gain | Cost and risk | Disposition |
|---|---|---|---|
| Research notes only | Prunes area-only methods while preserving chapter focus | Reader does not see the broader obstruction | Recommended from Jörn's explicit prior |
| One paragraph above, with a resolvable proof reference | Separates lack of exact information from bad fitting | Needs a retained manuscript-local proof route; still no upper-tail success | Not selected: still the counterexample class covered by his answer |
| Affine-pentagon identity plus upper/lower counterexamples and full-list construction | Gives the entire sequence of hypotheses and failures | More exposition and proof obligations; rewards the number of results rather than insight | Do not select by default |

The rubric is conceptual gain for the data chapter, truthful scope, reading
cost and distance to submission. Counting exact results would wrongly reward
the longest alternative. Optimizing brevity alone would suppress a useful
obstruction even if Jörn values its stronger information claim. The current
choice applies his stated preference. The pending inclusion question is closed
as answered elsewhere; the other four questions remain useful.

## Evidence and verification boundary

- [Construction, formulas, completeness argument and independent mathematical review](file:///workspaces/msc-math-worktrees/patterns-2026-10-17/patterns/ridge-functionals/unsigned-list/REPORT.md).
- [Independent exact verification](file:///workspaces/msc-math-worktrees/patterns-2026-10-17/patterns/ridge-functionals/unsigned-list/independent-verification.json): volume $330$, 27 areas, identical difference bodies, capacities $600/31$ and $675/37$; two full enumerations already agree on 960 cyclic words per body.
- [Earlier affine identity and envelope failures](file:///workspaces/msc-math-worktrees/patterns-2026-10-17/patterns/ridge-functionals/REPORT.md), retained as research context rather than additional proposed paragraphs.

The paragraph's ratios were recomputed from those retained exact capacities
using $\mathrm{sys}=c^2/(2V)$; both lie strictly between zero and one and are
unequal. All linked local artifacts exist. This batch checks exposition against
existing verification; it does not repeat or enlarge the scientific experiment.
Patterns sources remain on their separate branch. No manuscript-source change
or human acceptance follows from this proposal.

[review: /root/delivery_review]
