# Proof obligations for the two-parameter descendant

This packet uses the complete coverage argument in the independently reviewed
[sibling proof](../certificate-continuation-20261029/COVERAGE-PROOF.md), with
freshly checked finite predicates for a different body. Production and final
review status are recorded in README and AUDIT; until they pass, the feasible
word below supplies only a capacity upper bound.

## Exact body and changed parameter

Use the exact original HKO matrix `N0`, cut-direction matrix `D`, number field
`Q(tau)`, real embedding, symplectic matrix `J` and label order reconstructed
in `common.py`. Set

    a = 99/125,    t = 7/20,
    E = D + (3/5) N0,
    N = [N0; a N0 + t E],    K = {x:Nx<=1}.

The new moving block is the old `H_(7/20)` block plus `N0/500`. Its radial
coefficient varies independently of the transverse cut coefficient, so this
body is not the old one-parameter `H_(7/20)`. The first ten labels are still
the original HKO inequalities; the last ten retain their original pairing.
The body identifier in every receipt is `a=99/125,t=7/20`.

The exact objective remains `Q=beta^T G beta/2` under closure and
normalization `C beta=(0,0,0,0,1)`, with `G_ij=omega(N_i,N_j)` for i<j.
Capacity is `1/(2 Qmax)` and ratio `1/(8 V Qmax^2)`. `common.py` verifies
all twenty symmetry actions on all rows and on the pairing matrix at these
new parameters. No symmetry is assumed merely from the preceding certificate.

## Fresh geometry, graph, inclusion and volume

`exact_geometry.py` enumerates every nonsingular four-row system and tests
all twenty slacks exactly. Boundedness follows from containment in the
original HKO body, and the origin is strictly interior. Every vertex is
therefore represented; all 75 vertices are simple, and every one of the
twenty labels occurs in an active set. Thus all twenty facets are genuine.
The exact coactivity/nonnegative-pairing graph has 120 directed edges and is
written afresh. Its necessary-transition role is unchanged.

The support maxima of every moving row on the complete exact HKO vertex set
are checked against `a+t=571/500`. Therefore

    H/(571/500) subset K,
    Qmax(K) <= (tau/5) (571/500)^2.

This is the coarse rejection bound for singular stationary families. It is
fresh geometric evidence, not the old one-parameter support formula.

Two exact Lawrence evaluations with different generic linear functionals
agree on

    V(K) = 12188906067413/2122975008000
           - (7700705900357/20264761440000) tau^2.

All slope denominators are checked nonzero. The independently source-checked
Lawrence formula cited in the sibling geometry note is a mathematical input;
the two evaluations supply arithmetic confirmation at this new body.

## Complete capacity route and unchanged implications

The published Haim–Kislev geometric simple-minimizer theorem applies to this
bounded full-dimensional polytope with genuine facets. It supplies a global
maximizing word among simple directed cycles in the freshly computed graph.
The exact symmetry quotient, cyclic closure invariance, equality-row reduction,
KKT stationarity on positive supports and constant-objective identity for
singular families are unchanged from the sibling proof.

The proposal bank is the sibling's 99,085 accepted exclusions. Every one is
rechecked against the new body's exact tangent quadratic form; only passes
are retained. Two descendant rounds can add further exact exclusions. The
C++ DFS tests the full prefix and contiguous suffixes against this newly
verified bank. The zero-extension curvature lemma applies to each match;
checking a block when it first becomes a suffix excludes its entire future
subtree safely. There is no length cutoff. Fresh generation writes every
surviving canonical cycle, and a separate traversal requires no unknown cycle.
The old leaf list is not an input.

`exact_leaves.py` checks every new leaf, including all singular KKT families.
Any consistent singular family has constant objective. Constants below an
already attained target need no feasibility decision; constants above the
fresh inclusion bound cannot be feasible. For every other family, the exact
parameter-polytope vertex enumeration decides nonnegative feasibility. The
projection of the KKT kernel to beta is injective, and normalization puts beta
in a compact simplex, so the parameter set is bounded and the exhaustive
active-constraint basis test is complete, including for lower-dimensional
families. No sign of an arbitrary particular solution is used to infer
infeasibility. Any feasible family above the target would retain an actual
nonnegative witness.

The independently solved eight-label word `(0,6,5,2,19,11,8,4)` has strictly
positive weights and objective

    Q8 = tau (5328025872481 - 3162037824 tau^2) / 26435588952481.

The final verifier requires the global checked upper bound to equal an exact
attaining witness and checks all receipt and input-hash interfaces. It then
compares the ratio exactly with the earlier certified t=3/10 and t=7/20
values and with the rational scalar benchmark `1.052432`. Interpreting that
benchmark as a whole-family envelope bound requires the separately owned
continuation-envelope proof; this packet does not import such a claim into
its capacity computation.

## Scope

A successful completion establishes this single new body's exact capacity,
volume and ratio. It does not optimize the two-parameter family, cover the
original family outside any separately proved interval, establish a global
record, or replace the cited external theorems with a finite test.
