# HKO cut family: next-sprint handoff

Retained 29 October 2026 by `geometry_frontiers`; status: **open research
follow-up**, outside the selected thesis. This handoff preserves the next
bounded discriminator, not a newly executed experiment or a claim that this
family has been optimized. The coordinator selects and assigns any next sprint;
its execution owner is currently unassigned.

The question is whether the already certified twenty-facet example can be
improved within the same one-parameter family. Continuing the existing seven-
and eight-label candidates is a concrete inexpensive first discriminator.
A further certified improvement would strengthen the explicit counterexample;
a change of minimizing word could explain the family's capacity behavior.
Neither outcome is established by the retained probes.

## Exact object and label convention

Use coordinates `x=(q1,q2,p1,p2)`, `J=[[0,I],[-I,0]]`, and
`omega(v,w)=v^T J w`. Let `tau=tan(pi/5)`, the root in `(0,1)` of
`tau^4-10 tau^2+5=0`, `s=sqrt(5)=(5-tau^2)/2`,
`alpha=(3-s)/2`, and `beta=tau(1+s)/2`. The support-normalized rows of
`H={x:N0 x<=1}` are, in this exact zero-based order:

```text
0  (1, tau, 0, 0)          5  (0, 0, tau, -1)
1  (-alpha, beta, 0, 0)    6  (0, 0, beta, alpha)
2  (-(s-1), 0, 0, 0)      7  (0, 0, 0, s-1)
3  (-alpha, -beta, 0, 0)   8  (0, 0, -beta, alpha)
4  (1, -tau, 0, 0)        9  (0, 0, -tau, -1)
```

For a row `n=(n0,n1,n2,n3)`, define

```text
i < 5: D_i = (-3 n0/5, -3 n1/5, -n1/(s-1), n0/(s-1))
i >=5: D_i = (n3/(s-1), -n2/(s-1), -3 n2/5, -3 n3/5).
```

The twenty rows of `N(t)` have labels `0..9` equal to `N0`, and labels
`10+i` equal to `N0_i+t D_i`. Thus `H_t={x:N(t)x<=1}`. Equivalently,
`H_t=H intersect L_t^{-1}H`, with
`L_t(q,p)=(a q-b J2 p, a p+b J2 q)`, `J2(x,y)=(-y,x)`,
`a=1-3t/5`, and `b=t cos(pi/5)`. The inverse-image notation means
`L_t x in H`; no matrix inversion is needed to construct the inequalities.
The [family source](../../capacity-and-qp/convex-systolic-optimization/sections/hko-ascent.tex)
and [exact normal reconstruction](../../capacity-and-qp/convex-systolic-optimization/certificates/hko-t03/exact_certify.py)
fix these conventions.

For an ordered word `w`, nonnegative weights satisfy
`sum beta_k=1`, `sum beta_k N(t)_{w_k}=0`; the objective is
`Q=sum_{i<j} beta_i beta_j omega(N(t)_{w_i},N(t)_{w_j})`.
Then `c_EHZ=1/(2 Qmax)` and `rho=1/(8 volume Qmax^2)`.
The characteristic velocity convention is `-J` times an outward normal.

## What is already certified

The frozen [t=3/10 packet](../../capacity-and-qp/convex-systolic-optimization/certificates/hko-t03/README.md)
proves twenty genuine facets, seventy-five simple vertices, and

```text
Qmax = 205 tau/1019
c(H_0.3) = 1019/(410 tau) = (1019/1025)c(H)
volume = 300748771/52178240 - 99664289 tau^2/260891200
rho = 1.0519141388294875...
rho-rho(H) = 0.0047005433295293...
```

See its [exact attainer](../../capacity-and-qp/convex-systolic-optimization/certificates/hko-t03/exact-attainer.json),
[coverage proof](../../capacity-and-qp/convex-systolic-optimization/certificates/hko-t03/COVERAGE-PROOF.md),
[analytic review](../../capacity-and-qp/convex-systolic-optimization/reviews/hko-t03-mathematics.md),
and [independent execution review](../../capacity-and-qp/convex-systolic-optimization/reviews/hko-t03-independent-replay.md).
The [reader-facing theorem](../../capacity-and-qp/convex-systolic-optimization/sections/hko-moderate.tex)
is in the separate research report. These checks do not optimize `t`, prove a
family-wide maximum, or establish a world record. Preserve that packet, its
report PDF and companion unchanged; adaptations belong in a fresh run.

## Retained exploratory observations

These small JSON files are byte-for-byte copies of the 29 October discovery
outputs from agent `/root/geometry_frontiers`, before exact certification.
Their historical scratch location was the `geometry-frontiers/hko-beyond-infinitesimal`
subdirectory of the research restart run; no file there is a required input.
The filenames identify the parameter; the raw probe objects do not store it.

| Parameter | Literal candidate word | Floating Q | Floating volume | Derived rho estimate |
| --- | --- | --- | --- | --- |
| 0.1 | `(0,6,1,7,3,12,9)` | 0.14555626093884094 | 5.624608359960745 | 1.0489527919876482 |
| 0.3 | `(0,1,6,7,3,12,9)` | 0.14616410033473914 | 5.562221570778817 | 1.0519141388294833 |
| 0.5 | `(0,6,5,2,19,11,8,4)` | 0.14727325135323913 | 5.487491027910012 | 1.0502397221314772 |

[probe-0.1.json](probe-0.1.json), [probe-0.3.json](probe-0.3.json), and
[probe-0.5.json](probe-0.5.json) preserve the candidate weights, traversal
counts and termination flags. [geometry-summary.json](geometry-summary.json)
preserves numerical geometry at `t=0.01,0.05,0.1,0.2,0.3,0.5,1.0`.
It reports 85 vertices/130 directed transitions through `t=0.1`, and 75/120
at the sampled parameters from `0.2` onward. This is sampled numerical
combinatorics, not an exact chamber decomposition.

Discovery used floating four-row vertex solves and convex-hull volume, followed
by simple-cycle traversal with numerical tangent-curvature pruning and
nonsingular KKT solves. `stopped:false` means the traversal did not reach its
three-million-prefix cap; it does not validate its numerical pruning or cover
singular stationary families. The discovery producer is not retained here and
these raw outputs have no standalone replay promise. The exact certified
`t=0.3` producer is retained at the links above.

A truly feasible positive-Q word supplies an **upper bound** on capacity and
ratio, not a lower bound. The displayed quantities are floating approximations
to such witness bounds, not rigorous bounds until feasibility, arithmetic and
volume are validated. In particular the `t=0.5` value does not certify capacity
or demonstrate any improvement over the certified `t=0.3` body. Only `t=0.3`
has the separate complete certificate. The seven-to-eight-label change suggests
a useful investigation; it does not locate a transition or prove that further
gain exists between the samples.

## Minimal next discriminator and stopping rule

1. On a fresh run copy, reconstruct `N(t)` and continue the literal seven-label
   `t=0.3` word and eight-label `t=0.5` word at a small rational panel, for
   example `7/20,2/5,9/20`. Recompute weights, closure/normalization residuals,
   positivity, Q and volume. Solve `G beta+C^T lambda=0, C beta=e`, with
   `G_ij=omega(n_i,n_j)` for `i<j`, symmetric completion and zero diagonal.
   Here `C` contains the four normal coordinates and a row of ones,
   `e=(0,0,0,0,1)`. Rank changes and negative weights require explicit handling;
   neither a particular negative solution nor singularity excludes the entire
   family. This panel is proposed, not executed.
2. Compare the strongest available feasible-word upper estimates with the
   certified `t=0.3` ratio. A validated upper bound below that value can reject
   a parameter. An upper estimate above it merely leaves room for improvement.
   If promising room remains, perform numerical complete-word discovery at
   one selected parameter before deciding whether to spend on exact acceptance.
3. End this bounded discriminator if the sampled parameters are rejected or
   complete-search candidates show no gain. Report that limited outcome;
   it does not prove optimality between or outside samples. If a candidate
   improves numerically, select at most one exact certification and independent
   review as the next decision. Stop and reassess if graph/leaf complexity or
   unresolved singular families makes the existing cost model inapplicable.

A successful exact certificate must freshly establish geometry and graph,
curvature exclusions, cycle coverage, every stationary family, an attaining
word, exact volume and receipt consistency. The old scripts hardcode parameter,
counts, graph filename, target Q and the `28/25` inclusion bound. All these
must be recomputed/revalidated at a new `t`; old exclusions or leaf lists are
only proposals until checked and shown complete there. There is no ready
parameterized full-search command. The first two-word discriminator needs
only the definitions and retained candidate words above; a new general discovery
producer or adaptation of the retained acceptance machinery remains work.

## Measured costs and limits

For the successful `t=0.3` run, exact geometry took about 7.2 seconds,
77,927 exact curvature exclusions 360 seconds, 2,326 complete leaf checks
69 seconds, and integer cycle coverage a few seconds. Another comparable
certificate is plausibly several machine-minutes, not a guaranteed runtime
at new parameters. Discovery timings and model-token costs were not retained.
An abandoned Fourier–Motzkin feasibility attempt reached about 8 GB; the
accepted route instead enumerates vertices of the bounded free-parameter
polyhedron for unresolved singular families. Do not revive the failed route
by default. No research producer, new proof or parameter search was run to prepare
this handoff; the displayed rho estimates were arithmetic combinations of
existing probe values.
