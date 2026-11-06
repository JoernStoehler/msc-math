# Independent structural verification — 6 November 2026

This bounded review concerns the structural material absorbed by commit
`14c02346` from `research/viterbo-continuation-20261029` (tip `7b6463fa`).
Entry points were the live `STATUS.md`, `PLAN.md#absorbed-viterbo-continuation`,
and this directory's README and handoff. Baseline HEAD when inspected was
`e5975fab6053c7ffab4b74b13fb9bfb20bb6901a`. Existing shared working-tree edits
were left alone. Only this report is selected for commit. In particular, the
status/plan's broad warning is not silently replaced by universal acceptance.

The t=7/20 capacity certificate and two-parameter partial certificate are
excluded, as requested; no predicates from either were replayed. The older
six/seven-generator theorems in the parent rank-four packet predate this
absorption and are not reviewed here. No novelty, thesis acceptance, unrestricted
rank-four inequality or EHZ consequence is established.

## Inventory, dependency priority and verdicts

Priority ranks reflect how much a future research route could depend on the
claim, rather than apparent difficulty. Rank 1 is the all-size reduction; rank 2
comprises reusable obstruction/formulation/extension mechanisms; rank 3 their
concrete examples and candidate-selection machinery; rank 4 finite searches and
unfinished leads. Within a rank, checking favored proofs and cheap independent
exact calculations over replaying already recorded large computations.

`Holds` means the stated scoped argument checked here; qualifications in the
last column are part of the verdict. `Undetermined` means a decisive obligation
remains or the asserted universal target is open. `Unchecked` means no fresh
substantive verification. There are no confirmed failures.

Source abbreviations (all relative to `/workspaces/msc-math`):

- **R**: `topics/empirical-studies/discovery-design/desk/zonotope-research/rank-four-restart-20261029/continuation-20261029/`.
- **H**: `topics/variation-and-local-maxima/hko-cut-family/`.
- **A**: `topics/capacity-and-qp/research-continuation-20261029/alternative/`.

| ID | Priority | Structural claim and source | Verdict and coverage |
| --- | --- | --- | --- |
| S1 | 1 | Counterexample reduction to a no-larger rank-four, zero-neighbor-spanning, form-rigid extremizer; R `rigid-form-reduction.md` | **Holds.** Full proof checked step by step; independent Sol agreement. |
| S2 | 2 | Any finite retained-row bank of invariant cuts on one HKO facet has `R >= 5 nu/4`, hence `D+ log sys <= -nu`; H `single-facet-obstruction-20261029/PROOF.md` | **Holds conditional on frozen HKO orbit/variation inputs.** New proof and constants checked; not a new independent certification of those upstream inputs. |
| S3 | 2 | Exact parallel-translation envelope LP, variable/row counts, bounded optimal face under the stated anchor condition, elimination to nine perpendicular offsets; A `envelope-lemma.md` | **Holds.** Full elementary argument checked; does not solve the remaining nine-variable problem. |
| S4 | 2 | Contact plus homothetic rotated projection preserves equality under Lagrangian addition; R `lagrangian-addition.md` | **Holds.** Coefficients and planar-area normalization checked; concrete example recomputed exactly. |
| S5 | 2 | Matched-projection affine deficit, exterior formulation, and sufficient fixed-C operator-extension LP; R `projection-slope.md`, `slope-lp/README.md` | **Holds as identities/formulations.** Does not prove the universal slope estimate; no LP solution is claimed. |
| S6 | 3 | Two nonproduct ten-direction equality families; R `equality-extension.md` | **Holds.** Fresh exact polynomial expansion, ten distinct directions, all old triples independent. |
| S7 | 3 | 32 norm-extreme directions up to sign are contact directions; retained equality base has zero-neighbor rank three and form-constraint rank five; R `equality-extension.md`, `rigid-form-reduction.md` | **Holds for this explicit base.** Fresh rational enumeration and ranks; generic-parameter rigidity not checked. |
| S8 | 3 | Every pairwise sum of six nonnegative equality additions has positive deficit for both positive parameters; R `contact-calculus.json`, `equality-extension.md` | **Holds.** Independently expanded all 15 deficits from proposed input rays; no saved gap polynomial used. |
| S9 | 3 | Contact quadratic form's negative eigenvalues, Schur elimination, completeness of twelve zero quadratic pairs, and signed rays' fourth-order deficits; same sources | **Unchecked.** Enumeration and S8 do not establish this larger contact-calculus package. |
| S10 | 3 | Positive seven/eight-label feasible branches throughout `(0,1)`; H `continuation-20261029/README.md` | **Holds.** Fresh exact KKT solves and algebraic root/sign checks. These are feasible witnesses, not capacities. |
| S11 | 3 | 75-simple-vertex chamber and exact rational volume on `[.3,.4]`; same source | **Undetermined.** First-event proof mechanism is sound, but the 75 determinant and 1,200 inactive-slack predicates were not replayed. Fresh floating volumes agree at .3, .34 and .4. |
| S12 | 3 | Increasing/decreasing witness ratio branches, unique crossover, cusp envelope maximum; same source | **Holds conditional on the stated volume formula.** Exact derivative/root checks; independent numerical crossover and cusp value. |
| S13 | 3 | Exact sampled volumes and exclusion of `.4,.45` as improvements; same source | **Undetermined at exact-certificate level.** Fresh floating geometry and KKT reproduce the exclusions with clear margins; exact volumes not independently certified. |
| S14 | 3 | Four exact directional original-template slice optima; A `README.md`, `envelope-results.json` | **Unchecked.** S3 verifies the formulation, not these primal/dual certificates. |
| S15 | 2 | Steiner section-centering yields a global lower relaxation, and two transverse segments force half their determinant area; A `steiner-gap.md` | **Holds.** Both geometric proofs checked. |
| S16 | 3 | Sharp axis relaxed optimum `tau/5`, three other directional optima, kite attainment; same source, `steiner-results.json` | **Undetermined.** Segment lower bound and scalar ratio algebra check; exact template containment/primal-dual attainment not independently replayed. |
| S17 | 3 | Restoring centers at one relaxed optimizer costs `(8-sqrt(5))/5` times HKO area; A `center-restoration.json` | **Unchecked.** No independent slice certificate replay. This is not a uniform correction over all relaxed minimizers. |
| S18 | 3 | Constant profiles cannot ascend; specified three-facet constant profile is dilation/translation and has response=volume=`1/10`; H `three-hinge-continuation-20261029/README.md` | **Holds.** Symmetry proof checked; fresh exact affine identity plus mass-balancing argument verifies this triple without importing the catalogue. |
| S19 | 4 | Finite hinge, outer-offset and twelve-case slope searches found no ascent/violation; H three-hinge README, A `outer-search/README.md`, R `projection-slope.md` | **Unchecked as recorded numerical experiments.** No claim of universal nonexistence follows. |
| S20 | 4 | Disjoint-support slope expansion and energy-distance reduction; R `rank-four-stop-state.md` final analytic lead | **Unchecked.** Sign/normalization derivation remains an explicitly unfinished lead. |
| S21 | 4 | Universal rank-four/slope inequality, global original-template partner bound, arbitrary three-hinge obstruction | **Undetermined.** None is established by the scoped results above. |
| S22 | 4 | Two-parameter branch formulas and finite-bank candidate estimates outside its partial certificate; H `two-parameter-continuation-20261029/` | **Unchecked.** Left to the parallel two-parameter verification scope; no complete capacity claim adopted. |

## S1: step-by-step proof verification

1. `rank <= 4`, `Phi=1` is a compact subset of finite-dimensional skew
   matrices; rank is closed by minors, and each entry is bounded. A positive
   maximum of `Psi` exceeding `1/4` exists. Minimal support is an integer
   secondary choice. Removing zero rows preserves the objective and padding
   competitors preserves global maximality. Rank at most two has zero `Psi`.
2. Real skew normal form supplies four-dimensional spanning generators. If a
   deletion block has rank at most two, its four-Pfaffians vanish. Expanding
   each remaining Pfaffian bounds `Psi` by `a Phi(old)`: each incident edge
   times a disjoint old edge is counted once, not repeatedly. Thus
   `4 Psi <= 4a Phi(old) <= (a+Phi(old))^2`, contradiction. Every other-generator
   list therefore spans four-space.
3. Its incident pairing sum is consequently a norm. Preserving old incident
   zeros, signs and incident mass gives a compact polytope with the old
   generator in its relative interior. If the zero-neighbor span were at
   most two, dimension would be positive. Convexity of the incident absolute
   determinant sum supplies a vertex with no smaller `Psi`; it acquires a new
   zero without losing old zeros. This contradicts minimal support. The
   zero-neighbors therefore span precisely the three-dimensional perpendicular.
4. The Pfaffian form `q` on six-dimensional alternating forms has signature
   `(3,3)`. Every four-generator Pfaffian scales by `q(eta)` relative to
   `q(omega)=1`. On the current zero face `L`, the pairing norm is locally
   linear `ell`. Global maximality implies a local minimum of `ell` on `q=1`.
   The displayed normalized curve gives `ell(z)=0` and `q(z)<=0` for the
   `B`-orthogonal tangent directions. There is no assumption of a positive
   Hessian or isolated extremizer.
5. A radical direction has `q=ell=1` along its affine line through omega.
   Its segment in the closed sign cone is bounded because the pairing sum
   is a norm. An endpoint preserves the objective and old zeros and adds a
   new zero: contradiction. Therefore `q|L` is nondegenerate of signature
   `(1,d-1)`.
6. Under volume duality the active simple bivectors are null and span
   `F=L^perp`; its signature is `(2,4-d)`. Hence `d<=4`; `d=4` is excluded
   by positive definiteness and nonzero null spanning vectors. For `d=3`,
   Witt index one makes any mutually orthogonal null incident span at most
   one-dimensional. Step 3 instead gives dimension two after wedging by the
   nonzero incident generator. Contradiction.
7. For `d=2`, the two null forms have rank two and complementary kernels.
   A generator with components in both kernels has common annihilator of
   dimension two, contradicting Step 3. All generators lie in the two
   complementary planes; cross pairings vanish. `Psi=AC`, `Phi=A+C` excludes
   a counterexample. Only `d=1` survives.

Independent reviewer `/root/sol_review` (gpt-6.1-sol, high effort) checked the
entire proposition separately and returned **HOLDS**, explicitly confirming
these steps. The reviewer did not verify the final generic-example assertion;
our fresh rational rank computation covers the explicit displayed base only.
Neither verdict resolves the surviving rigid case.

## S2–S5 and S15: normalization and implications

For S2 the two exact scalar identities are
`(b/A)(k+1)=4` and `(C/B)(1-k)=4` in `Q(sqrt(5))`.
The literal orbit families bound `f(+/-1,y)` by `R` divided by their facet
weights. Integration against the two density pieces gives at most `8R` on
each edge. Convexity in the first coordinate bounds the interior mean by the
average edge values. Hence `E f <= 8R`, `nu=E f/10 <= 4R/5`, and
`4(nu-R) <= -nu`. Nonnegative nonzero continuous profiles have positive
integral. This last strictness assumes the profile is nonzero on the facet,
not merely that one of the ambient cut velocities is nonzero. We read the
frozen Appendix C and simultaneous-splitting proof to check the inputs and
normalization; their HK capacity/shortest-orbit foundation was not re-proved
or externally source-checked in this bounded review.

For S3, the discrete slopes make interpolated `L` convex and `R` concave;
point constraints enforce nonnegative width at every height. Thus the LP
constructs a convex cover with objective equal to its area. Conversely hull
endpoint breakpoints have input heights, so every shifted hull is represented
exactly. Feasibility and nonnegative objective ensure an attained LP minimum.
A zero-cost recession has equal endpoint derivatives; both convexity and
concavity force an affine sequence. Two distinct anchored heights kill that
affine freedom and then every shift derivative. Counts and nine remaining
perpendicular offsets follow. This neither requires genericity nor asserts a
global optimum of the original placement problem.

For S4, `Phi(t)=Phi+tM`. The linear `Psi` coefficient is `Phi M/2` by
contact; the quadratic is `area(P) area(Q)/16`. The cross term in
`area(P+sQ)` is `4Ms`; when `Q=cP`, this implies
`area(P) area(Q)=4M^2`. All higher determinants vanish because three added
vectors lie in a plane. These constants give precisely `Phi(t)^2/4`.
The degenerate zero-area case also satisfies the same identities.

For S5, dropping contact leaves the linear coefficient `B` unrestricted;
choosing `P=Q` gives `M=area(Q)/2`, so the quadratic deficit cancels exactly.
The exterior constraint and dimension count follow by extending independent
`g,h` to a basis: all components of `W` outside their two coordinate directions
vanish. Thus `W=g wedge r+h wedge s`, rank at most four, and dimension
`2n-3`. The sufficient LP is valid since its column bounds are exactly the
ambient induced l1-norm bound. Failure to find such an extension would not
refute the restricted-space inequality. No universal estimate was supplied.

For S15, every section of a containing body has width at least that of each
contained template. Centering preserves this inclusion and area; the width
of a convex body is concave, so the centered body is convex. Taking a hull
and then minimizing perpendicular translations gives a lower bound. The
two-segment triangle proof is valid even if the perpendicular segment's
abscissa lies outside the horizontal segment. With the stated lengths,
`wh/2=tau/5` and division by `tau(5-sqrt(5))/10` gives
`(5+sqrt(5))/10`. This scalar check is not an attainment certificate.

## Fresh computations and observed results

No continuation producer, checker or helper code was imported or executed.
The three fresh scripts are in `/tmp/structural-verify/`; retained JSON contact
rays were used only as candidate inputs. Existing branch audits were not used
as proof authority. Ordinary Python supplies rational arithmetic; the existing
Sage interpreter supplies exact number fields and real algebraic root isolation.

`python /tmp/structural-verify/recompute.py` uses the displayed matrix A,
`e_i`, `w_i=-Ae_i/3`, and the two literal addition lists. For each subset of
size two or four it directly evaluates the pairing or
`W12 W34-W13 W24+W14 W23`, takes its absolute value, and accumulates by the
number of added generators. Because the parameter is nonnegative, this gives
whole-ray coefficients without sampling. Output, in ascending parameter degree:

| Family | Phi coefficients | Psi coefficients |
| --- | --- | --- |
| Symplectic parallelotope addition | `32,168,144` | `256,2688,9360,12096,5184` |
| Lagrangian addition | `32,168,0` | `256,2688,7056,0,0` |
| Scaled Lagrangian example in S4 | `32,14/3,0` | `256,224/3,49/9,0,0` |

Every coefficient agrees with `4Psi=Phi^2`. The four unscaled S4 directions
have `(a,b)` equal to `(8,128),(8,128),(36,576),(12,192)`.
All 56 triples of old generators are independent (positive rational Euclidean
Gram determinant), and each family has exactly ten projective directions.
A product of two planar bodies would put all generating edge directions into
two planes, each accommodating at most two of these old directions; impossible.

The same script enumerates all rank-three intersections of the eight norm
hyperplanes, deduplicating projective rays: exactly 32. Direct determinant
sums give `b=16a` on every ray. Zero-pairing equations on the displayed base
have rank five in the six form coordinates; every zero-neighbor list has rank
three. For S8 the six nonnegative retained rays were interpreted in normalized
directions `u_i/a(u_i)`; the old weights are eight on the original directions.
Fresh bivariate expansions give nonnegative coefficients and a nonzero
positive mixed term for each of the 15 pairs, with all pure terms zero.
An initial scratch assertion failed before this normalization was applied;
the corrected run passed all assertions. It was not evidence against S8.

`/workspaces/msc-math/.local-environments/miniforge/envs/sage/bin/python
/tmp/structural-verify/hko_exact.py` constructs the support rows from the
mathematical parent HKO README and solves the augmented KKT linear systems
over `Q(tau)(t)`. It checks that every weight numerator and denominator has
no root in the **open** interval `(0,1)` and weights are positive at `1/2`.
Endpoint zero weights are permitted. The seven-label objective simplifies to
`tau(15t-25)/(77t-125)`. The eight-label objective is quadratic over quadratic.
Using the stated rational volume formula, the ratio derivative numerators
and denominators have no roots in `(.3,.4)`, with signs respectively positive
and negative. The objective difference has exactly one root there. The script
also solves the five exact equations `c_i=a+n_i dot u` for the constant triple,
obtaining `a=1/5`.

For that triple every maximizing HKO word has total weight `1/2` in each
factor: rescaling the factors to half-mass multiplies the positive bilinear
objective by `1/[4m(1-m)]`, so maximality forces `m=1/2`. Closure therefore
makes response `a/2=1/10`, equal to `(c0+c1+c2)/10`. This proves the triple
identity without depending on a chart enumeration. The general constant-profile
nonascent follows separately by facet-transitive symmetry averaging.

`hko_numeric.py`, run in the same interpreter, independently uses halfspace
intersection and a four-dimensional convex hull, rather than Lawrence volume,
plus dense numerical KKT solves. It deliberately skips `t=7/20`.

| t | Fresh volume | Seven-label ratio bound | Eight-label ratio bound |
| --- | --- | --- | --- |
| .3 | 5.5622215707788145 | 1.0519141388294866 | 1.0527003147527902 |
| .34 | 5.548548824159201 | 1.0524272049215329 | 1.052434687499303 |
| .4 | 5.526997404381517 | 1.0531573781194865 | 1.0517836035256893 |
| .45 | 5.507886858529556 | 1.0537400754185162 | 1.051063913213574 |

All four have 75 numerically reconstructed vertices. Fresh crossover:
`0.3403557218557086`; ratio `1.0524316650246788`. Floating volumes at the
three in-chamber samples agree with the proposed formula to less than `1e-11`.
These observations support the numbers but do not replace exact slack,
combinatorics, volume or capacity certificates. No library tolerance was
promoted to a proof.

## Scope closure and next action

Luna performed bounded read-only inventory triage; the primary reviewer did the
mathematical checking and wrote all scratch code. One Sol independently reviewed
S1. Resource allocation was kept to these two children and small local runs;
there is no reliable per-task dollar meter, so the approximately $1 API-equivalent
budget cannot be certified as a measured charge. The review stopped after the
ranked proof checks and inexpensive recomputations, within the approximately
30-minute time allowance; it did not start a broad archive or literature audit.

The next useful verification is S11's exact geometry: independently enumerate
all bases, establish the chamber's determinant/slack predicates, and derive the
volume, then remove S12's volume qualification. For the translation-cover route,
independently reconstruct template inputs and verify the exact primal/dual
certificates for S14/S16/S17. S9 and S20 remain separate lower-priority algebra
checks. The unrestricted research targets remain open. This report records the
bounded result for the receiving coordinator; existing shared PLAN/STATUS files
were deliberately not edited under this assignment's one-new-file constraint.
