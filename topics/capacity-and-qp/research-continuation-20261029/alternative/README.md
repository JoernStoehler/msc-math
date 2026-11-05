# Alternative mechanism: exact directional translation covers

29 October 2026 continuation. The [envelope lemma](envelope-lemma.md) proves
that translating finitely many planar templates independently along a common
direction has a finite linear-programming formulation. For N vertices, k
templates and m distinct perpendicular coordinates, it uses `2m+k-1` variables
and `2N+2(m-2)` inequalities. It includes all degeneracies without artificial
translation bounds or iterative supporting-cut generation. It does not solve
the unrestricted two-dimensional translation-cover problem.

The [producer](parallel_envelope.py) reconstructs exact primal/dual certificates
after numerical basis discovery. The [retained certificates](envelope-results.json)
pass ordered-field feasibility, exact zero gap and a separate exact convex-hull
area check. The four whole directional slices give:

| Fixed perpendicular offsets and direction | Area / HKO area | Variables / rows |
| --- | ---: | ---: |
| HKO placement, horizontal | 1 exactly | 19 / 66 |
| HKO placement, slope 1/7 | 1 exactly | 29 / 76 |
| All centered, horizontal | 1.2230509022726634 | 37 / 84 |
| Specified rational vertical offsets, horizontal | 1.365087217647253 | 57 / 104 |

The decimal entries summarize exact field elements retained in the JSON. Each
row is an optimum over all nine independent parallel translations; the last
two therefore discard entire slices, not just individual placements. No global
fixed-partner bound or newly improving body is claimed. Observed producer time
for these four cases was 3.26 seconds, excluding process setup, on this host;
this is not a broad search cost estimate.

The [independent review](review.md) checks the proof and implementation, with
its exact file identities and limits. The source uses the existing exact
pentagon fixture and its imported capacity criterion unchanged. Jung--Ahn's
ESA 2026 paper already proves directional convexity and gives another exact
algorithm; the present contribution is a finite LP mechanism and executable
certificate route in this repository, with no external priority claim.

From the checkout root, replay all retained certificates without an LP solve:

```sh
uv run topics/capacity-and-qp/research-continuation-20261029/alternative/replay_envelope.py \
  topics/capacity-and-qp/research-continuation-20261029/alternative/envelope-results.json
```

Regenerate numerical proposals and exact certificates into a fresh file:

```sh
uv run topics/capacity-and-qp/research-continuation-20261029/alternative/parallel_envelope.py \
  --out /tmp/msc-math/viterbo-continuation-20261029/alternative/NEW-RUN.json
```

Both commands read the checkout's unchanged fixture. The producer refuses an
existing output path. Inline dependencies declare NumPy, SciPy and SymPy; the
replay imports the producer module but never invokes its LP solver. Numerical
basis discovery can fail even when the mathematical LP is sound; exact checks
reject a bad proposed basis or dual.

The remaining mechanism question is the value function in the nine free
perpendicular offsets after parallel offsets have been eliminated. Its changing
height order and basis are the unresolved coupling. The bounded outer-search and contact-dual continuation are retained below;
the global lower-bound question remains open. No future execution is selected
merely by this note.

A further [global Steiner relaxation](steiner-gap.md) is now exactly solved in
four directions. In the symmetry-axis direction its optimum is only
`(5+sqrt(5))/10=0.7236068...` of the HKO area; a kite attains this value, already
forced by two segment determinants. Thus erasing section centers loses
substantial information. Restoring the original templates at one relaxed
optimizer and optimizing all horizontal shifts has exact area
`(8-sqrt(5))/5=1.1527864...` times HKO. This is a checkable obstruction to that
relaxed placement, not a global original-cover theorem.

The [bounded outer-search record](outer-search/README.md) retains two twelve-start
numerical rehearsals: 2,652 and 2,606 directional LP solves, about 19 seconds
observed wall time each, with no original-hull area below HKO. These runs used
height coalescing at 1e-6 and direct uncoalesced hull rescoring; they do not
establish local convergence or a global lower bound. The second run's search
box is not a proved competitive domain. The original producer paths are
retained as historical configuration, not advertised as portable commands.
