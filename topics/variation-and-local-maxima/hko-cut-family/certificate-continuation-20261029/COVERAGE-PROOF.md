# Capacity certificate at t = 7/20: complete proof obligations

This packet concerns the single body `K = H_(7/20)` in the HKO cut family.
Its acceptance requires every checker listed in `README.md` and a matching
`verification-summary.json`; decimal branch probes are not proof premises.
The final audit/replay disposition is recorded separately in `AUDIT.md`.

## Exact object, arithmetic and conventions

`common.py` reconstructs the ten support-normalized HKO rows `N0` and their
specified cut directions `D`, in the label order of the parent family README.
It sets `t=7/20` and `N=[N0;N0+tD]`, so `K={x:Nx<=1}`. Arithmetic takes place
in `Q(tau)`, where `tau^4-10tau^2+5=0`; its real embedding is explicitly
asserted to have `0<tau<1`. Every order test uses this exact embedding into
Sage's real algebraic field `AA`.

For a word `w`, `C` has the four normal-coordinate rows followed by a row of
ones, and `e=(0,0,0,0,1)`. Feasible weights obey `C beta=e`, `beta>=0`.
The symmetric zero-diagonal matrix `G` has entries
`G_ij=omega(N[w_i],N[w_j])` for `i<j`. Thus

    Q = beta^T G beta / 2,
    c_EHZ(K) = 1/(2 Qmax),
    rho(K) = 1/(8 vol_4(K) Qmax^2).

Here `omega(v,w)=v^T J w`, with `J=[[0,I],[-I,0]]`, and the characteristic
velocity convention is `-J` times an outward normal.

## Geometry and the graph reduction

`exact_geometry.py` freshly enumerates all four-row subsets of `N`, solves
each nonsingular system, and retains precisely the solutions satisfying all
twenty inequalities. The origin is strictly interior, and `K subset H` makes
the body bounded. Every vertex has four independent active normals, so this
enumeration is complete. It yields 75 vertices, each with exactly four active
labels. All twenty labels occur, which proves twenty genuine facets: at a
simple vertex each independent active inequality supports a local facet.

The graph written to `graph.txt` has an edge `i -> j` precisely when the
facets meet and `omega(N_i,N_j)>=0`; it has 120 edges. Two facets meet if and
only if some enumerated vertex lies in their intersection. This graph is
computed from the new exact geometry, without trusting the old graph.

Haim–Kislev's geometric simple-minimizer theorem supplies a capacity-minimizing
polygonal characteristic whose nonzero normal segments have distinct labels.
For a segment on facet `i` entering facet `j` from inside the body, the
incoming derivative is `N_j . (-J N_i) = omega(N_i,N_j)>=0`. Consequently
there is a global maximizing word that is a simple directed cycle of this
graph. This is a necessary-transition reduction; no sufficiency claim is
made for arbitrary graph cycles or arbitrary algebraic stationary words.
The underlying theorem is Theorem 1.5 of Haim–Kislev, arXiv:1712.03494v3,
page 3 (published in 2019); its convention mapping and direct source check
are retained with the earlier `hko-t03` mathematical review. The present
body remains within the theorem's general polytope scope.

## Exact symmetry quotient and inherited curvature

Each ten-label block is written as `(f,i)`, `f in {0,1}`, `i in Z/5`.
Simultaneously rotate `i` and optionally exchange `f`; these give ten
symplectic permutations. Reflection `i -> 4-i` is antisymplectic, and is
combined with reversal of word order. `common.py` constructs all twenty
linear transformations from four independent original rows, verifies their
action on all twenty rows at `t=7/20`, and verifies preservation or negation
of the complete pairing matrix. Thus feasibility and objective are preserved.

Cyclic rotation preserves `Q` on closure weights. It also preserves the
quadratic form on `ker C`: the wrap-around difference is a pairing against
the total normal sum, which vanishes. The canonical representative used by
the C++ code is the least word among these twenty actions followed by cyclic
rotation to the least label.

For any word, if `z in ker C` and `z^T G z>0`, no strictly positive feasible
weight can be a local maximum. Padding `z` by zeros preserves both constraints
and its positive quadratic value on every cyclic extension of that word.
Therefore a positive-curvature prefix can exclude the entire DFS subtree.

`proposed-exclusions.txt` is only a proposal bank copied from the old `t=3/10`
packet. `exact_curvature.py` rechecks every proposal at `t=7/20`. It computes
an exact kernel basis `Z` and detects positive directions of `Z G Z^T` by
congruence: a positive diagonal suffices; a negative diagonal is eliminated
by its Schur complement; an all-zero diagonal with a nonzero off-diagonal
has a positive direction on a two-coordinate subspace. A form exhausted by
negative pivots and zeros has no positive direction. Only passing proposals
are written to `base-exclusions.txt`. Nonpassing proposals are discarded and
recorded; they are not accepted pruning certificates or unresolved failures.

`exact_extend_curvature.py` augments this valid but insufficient pruning bank.
For each discarded canonical prefix, it takes every cyclic rotation that is
a graph walk and proposes unused graph-valid labels on either end. Taking
both ends covers the reversal in antisymplectic canonicalization. It checks
every proposed extension with the same exact positive-curvature predicate,
then repeats once from the nonpositive extensions. Passing extensions are
retained in `extension-exclusions.txt`; their union with the original passing
bank is `exclusions.txt`. Proposal completeness is not an acceptance premise:
every retained exclusion is checked exactly, and the final DFS covers all
cycles that survive the resulting finite bank. The helper records both rounds
and all discarded proposals in `curvature-extension-summary.json`.

## Fresh complete cycle enumeration

`coverage_replay.cpp --generate` uses the new graph and only the rechecked
exclusion set. Its DFS starts at each possible least label, then visits only
larger unused labels, and tests closing edges. It has no length cutoff and
makes no floating-point decision. At each prefix it stops only if the canonical full prefix or a contiguous
suffix belongs to the exact exclusion set. The suffix test uses the same
zero-padding curvature inheritance; it does not assume that the excluded
word begins at the globally least label. Every internal contiguous block
was a suffix when its last label was appended, so it is checked then. The
minimum-length optimization skips only suffixes shorter than every word in
the exclusion bank. Every other closed cycle
is inserted into a freshly generated `leaves.txt`; the old leaf list is not
an input. A separate invocation without `--generate` traverses the graph
again and requires zero cycles missing from that leaf set.

A maximizing positive-support word cannot be pruned, because its global
maximum is also a local maximum on its active support. The geometric
simple-minimizer theorem can be applied directly to its nonzero segments,
so the positive support remains a searched graph cycle. This handles boundary
maxima without asserting that every arbitrary subword is dynamically valid.
Thus some global maximum is represented among the checked leaves.

## Every leaf, including singular KKT families

`exact_leaves.py` first tests consistency of `C beta=e`. It then retains
independent rows of `C` and the matching entries of `e`, preserving the
solution set. Every positive-support local maximum has a multiplier solving

    G beta + C^T lambda = 0,   C beta=e.

A nonsingular KKT matrix gives a unique stationary solution. A negative
weight excludes an interior maximum; otherwise its exact value is compared
with the running attained bound. Saddles may be included harmlessly.

For a singular KKT system, exact rank decides consistency. On a consistent
family `(beta0,lambda0)+ker KKT`, the objective is constant. A kernel element
`(z,eta)` obeys `Cz=0`, `Gz+C^T eta=0`; multiplying the equations by the
stationary point gives `e.eta=0`. Hence `Q=-e.lambda/2` is unchanged. These
identities are checked exactly for the full kernel basis.

A constant at or below an already attained `Q` cannot improve the maximum.
A second rejection uses the freshly verified inclusion
`H/(57/50) subset K`: exact vertex support maximization on `H` gives support
1 for the original rows and 57/50 for every moving row. Monotonicity and
quadratic homogeneity of capacity yield
`Qmax(K) <= (tau/5)(57/50)^2`, using the known `Qmax(H)=tau/5`.
A larger singular constant has no feasible member.

Every remaining singular family is resolved by exact feasibility, never by
the sign of one particular solve. Write `beta=beta0+Z y`. Projection of the
KKT kernel to beta is injective: if `z=0`, then `C^T eta=0`, and full row
rank of `C` forces `eta=0`. Thus `Z` has full column rank. Normalization puts
beta in a compact simplex, so `{y:beta0+Zy>=0}` is bounded. If nonempty, this
bounded polyhedron has a vertex with `d=dim y` independent active constraints
`beta_i=0`, even if it is lower dimensional. The checker enumerates every
d-element index set, solves each nonsingular active system, and checks all
weights. Exhaustion proves emptiness; a successful solve retains the actual
nonnegative beta and checks its constraints and constant objective.

The checker starts from an independently solved, strictly positive candidate
in `exact-attainer.json`. It considers both the known seven-label word and
the new eight-label word. The final verifier independently rechecks their
closure, positivity and objectives, and requires the global checked upper
bound to equal the strongest explicit witness.

## Volume, receipt consistency and scope

`exact_geometry.py` applies Lawrence's simple-polytope volume formula with
two exact generic functionals. It checks every slope denominator is nonzero
and both field values coincide. The result is

    vol_4(K) = 6644112217/1156332480
               - (2199353203/5781662400) tau^2.

The formula and exact inclusion argument are detailed in `GEOMETRY.md`.
The formula is a cited mathematical input, not proved merely by agreement
of two evaluations.

`verify_result.py` checks the partition of old proposals into accepted and
discarded words, leaf/exclusion counts across receipts, the complete singular
case accounting, hashes of the leaf checker's inputs, and exact attainment.
It computes the ratio and compares it in `AA` both with HKO and with the
previous certified `t=3/10` ratio. The aggregator alone is not a substitute
for running the underlying checks. `replay.sh` provides a new-directory
execution route using only retained source and proposal inputs.

This certificate proves a value for one fixed body. It does not optimize the
one-parameter family, prove a general upper envelope, establish a global
record, or replace the external theorems and independent audit requirements.

## Completed finite result

The production receipts establish 99,085 positive-curvature exclusions,
2,839 completely checked canonical leaf words, and zero missing cycles.
The eight-label positive witness `(0,6,5,2,19,11,8,4)` matches the checked
upper bound exactly:

    Qmax = tau (8456329321 - 4892944 tau^2) / 41964992141.

Of 1,151 consistent singular systems, 1,087 have objective below this
attained target; 33 violate the inclusion bound; and 31 are exactly
infeasible by complete basic-feasible-parameter enumeration. None remains
unresolved. Together with the exact volume above this gives
`rho(K)=1.052345598871062...`. The exact sign test proves an increase of
`0.00043146004157479465...` over the previously certified `H_(3/10)`.
The audit record separately identifies the independent reproduction and
assembled-proof review supporting final acceptance.
