# Reduction to configurations that determine the symplectic form

29 October 2026. Original continuation work by `/root/rank_four`, with the
form-face argument and its incidence consequences developed jointly with
`/root/rank_four/structural_dual`. **Independent mathematical audit passed; the audit and its scope are retained
alongside this note.** This is an all-size necessary-condition reduction, not a proof of
the unrestricted inequality and not a classification of its equality cases.

Write `Phi(W)=sum_{i<j}|W_ij|` and
`Psi(W)=sum_{i<j<k<l}|Pf(W_ijkl)|`.

## Proposition

If the inequality `4 Psi <= Phi^2` fails for some real skew matrix of rank at
most four, it fails for a matrix `W=(omega(v_i,v_j))` with generators spanning
an oriented four-dimensional real space `V`, and having all the following
properties:

1. There are no zero rows. For each `i`, the generators `v_j` with
   `omega(v_i,v_j)=0` and `j != i` span the three-dimensional hyperplane
   `v_i^omega`.
2. The zero pairings determine `omega` up to scale. Precisely,

   `L={eta in Lambda^2 V*: eta(v_i,v_j)=0 whenever omega(v_i,v_j)=0}`

   is the one-dimensional space `R omega`.

The number of generators is no larger than that of the original
counterexample. No enumeration of support graphs enters the reduction.

## Selecting an extremizer in a fixed finite size

Fix a size `n` admitting a counterexample. The set of real skew matrices with
`rank(W)<=4` and `Phi(W)=1` is closed and bounded, hence compact. The continuous
function `Psi` has a maximum `M>1/4`. Among its maximizers choose one with the
fewest nonzero upper-triangular entries. Remove its zero rows. This only
decreases the number of generators and changes neither functional. Any
competitor in the reduced size may be padded with zero rows back to size `n`,
so the original global maximum remains applicable.

The matrix has rank exactly four, since rank at most two makes all four-index
Pfaffians vanish. Real skew normal form realizes it as the pairing matrix of
vectors spanning `V=R^4` with a nondegenerate alternating form `omega`. Choose
the orientation and volume form `vol` so that

`omega wedge omega / 2 = vol`.

Define the Pfaffian quadratic form `q` on `Lambda^2 V*` by
`eta wedge eta / 2 = q(eta) vol`. Its inertia is `(3,3)` and `q(omega)=1`.

Every block obtained by deleting one row and column has rank four. Indeed,
if such a block `W_0` had rank at most two, its old four-index Pfaffians would
vanish; expansion of the new Pfaffians gives

`Psi(W) <= a Phi(W_0)`, where `a=sum_j |W_ij|`.

Each product of an incident edge and a disjoint old edge occurs at most once
in that bound. Consequently
`4 Psi <= 4 a Phi(W_0) <= (a+Phi(W_0))^2=1`, a contradiction.
Thus the generators other than any fixed `v_i` still span `V`.

## Each zero-neighbor set spans a hyperplane

Fix `i` and hold all other generators fixed. Write `a>0` for the incident
pairing mass at `v_i`. On replacing `v_i` by `u`, the function

`a_i(u)=sum_{j != i}|omega(u,v_j)|`

is a norm because the other generators span `V`. Let `K_i` be the subspace
cut out by all zero pairings incident to `i`. Impose on `u in K_i` the weak
sign inequalities of the currently nonzero pairings, together with
`a_i(u)=a`. Within this sign cone the last condition is linear. The resulting
set is a compact polytope, and `v_i` is in its relative interior: every
nonzero pairing has strict prescribed sign there.

If the zero-neighbors spanned dimension at most two, `dim K_i>=2` and this
polytope would have positive dimension. Its vertices would therefore acquire
at least one additional zero pairing. On it, the old contribution to `Psi`
is constant and the contribution using `i` is a sum of absolute linear
functions of `u`, hence convex. A vertex has value at least the value at
`v_i`. The total `Phi` remains one. By global maximality the vertex also
attains `M`; by minimal support it cannot acquire an additional zero.
Contradiction.

The zero-neighbor span is contained in `v_i^omega`, which has dimension three.
It therefore equals that hyperplane, proving property 1.

## The form-face restriction has Lorentz signature

Let `L` be as in the proposition and let `d=dim L`. Near `omega` within `L`,
the function

`N(eta)=sum_{i<j}|eta(v_i,v_j)|`

is one linear functional `ell(eta)`, with `ell(omega)=1`. The generator span
makes `N` a norm on `Lambda^2 V*`: the wedges of a spanning set span
`Lambda^2 V`.

For every form `eta`, each principal four-Pfaffian of its pairing matrix is
`q(eta)` times the corresponding Pfaffian for `omega`. Thus on `q(eta)=1`,
`Psi` remains `M`. After division of the pairing matrix by `N(eta)`, its normalized objective
is `M/N(eta)^2`. Global maximality therefore implies `N(eta)>=1` there. In particular,
`omega` minimizes `ell` locally on `q=1` inside its sign face.

Let `B` be the polarization of `q`, so `B(omega,omega)=1`, and put
`T=L intersect omega^{perp_B}`. For `z in T`, the curve

`eta(t)=(omega+t z)/sqrt(1+t^2 q(z))`

lies on `q=1` near zero and stays in the sign face. Its first derivative at
zero gives `ell(z)=0`; its second derivative then gives `q(z)<=0`. Hence
`L=R omega orthogonal_direct_sum T`, with `q|T` negative semidefinite.

The restriction `q|L` is nondegenerate. Otherwise take a nonzero vector `z`
in its radical. Then `B(omega,z)=q(z)=ell(z)=0`, so `q(omega+t z)=1` and
`ell(omega+t z)=1` along the whole line. The portion of this line inside the
closed current sign cone has a finite endpoint, since `N` is a norm. At
that endpoint another nonzero pairing becomes zero; all old zero pairings
remain zero. The pairing matrix still has rank at most four and has
`Phi=1`, `Psi=M`, contradicting minimum support.

We have proved that `q|L` has signature `(1,d-1)`.

## Active simple bivectors give d <= 3

Call a pair active when its pairing is zero. The volume form identifies a
bivector `v_i wedge v_j` with the form
`c_ij = i_{v_j} i_{v_i} vol`. With the contraction convention
`(i_v alpha)(w_1,...)=alpha(v,w_1,...)`, the evaluation is
`eta(v_i,v_j)=2 B(eta,c_ij)`. This identification preserves decomposability
and the wedge pairing, up to a fixed nonzero scalar. Every active `c_ij`
is therefore `q`-null, and forms from two edges sharing a vertex are
`B`-orthogonal.

Let `F=L^{perp_B}`. The active forms `c_ij` span `F`, by definition of `L`.
Nondegeneracy and the inertia calculation above give

`signature(q|F)=(2,4-d)`.

In particular `d<=4`. If `d=4`, the space `F` is positive definite, so it
contains no nonzero null vector, contrary to being spanned by active
`c_ij`. Thus `d<=3`.

## The case d=3 is impossible

If `d=3`, then `F` has signature `(2,1)`, whose largest totally isotropic
subspaces have dimension one. For any fixed `i`, all its active incident
forms `c_ij` lie in `F`, are null, and are pairwise `B`-orthogonal because
their bivectors share `v_i`. Their span has dimension at most one.

On the other hand, property 1 says that the zero-neighbors span the
three-dimensional space `v_i^omega`, which contains `v_i`. The map
`w -> v_i wedge w` on this hyperplane has kernel `R v_i`, hence image of
dimension two. The incident active bivectors therefore span dimension two,
a contradiction.

## The case d=2 is a product and satisfies the inequality

If `d=2`, the form plane `L` has signature `(1,1)`. Its two independent null
lines have representatives `eta,theta`. These are nonzero rank-two alternating
forms and `eta wedge theta != 0`. Their kernels
`U=ker eta` and `V'=ker theta` are complementary two-dimensional subspaces.
The form `eta` is nondegenerate on `V'`, while `theta` is nondegenerate on `U`.

Every zero pairing of `omega` also vanishes for both `eta` and `theta`.
If a generator `v=u+w` had nonzero components in both `U` and `V'`, the
common kernel in `x` of `eta(v,x)=theta(v,x)=0` would be the two-dimensional
space `R u + R w`. Its zero-neighbors could not span `v^omega`, contrary
to property 1. Thus every generator belongs to `U` or `V'`.

Because `omega in L`, its cross pairings between `U` and `V'` vanish. Let
`A` and `C` be the total absolute pairings within the two groups. Then
`Phi=A+C`. Each nonzero principal four-Pfaffian has two indices from each
group and is the product of the two within-group pairings, so `Psi=A C`.
This gives `4 Psi<=Phi^2`, contradicting `M>1/4`.

Only `d=1` remains, proving property 2.

## Exact remaining obstruction

The surviving case has active zero-pairing bivectors spanning the full
five-dimensional primitive hyperplane, of signature `(2,3)`. At every
generator their incident span is a maximal two-dimensional isotropic
subspace. This configuration is possible: the retained eight-generator
paired-inverse equality examples have the same row-spanning behavior and,
for generic parameters, determine the form up to scale.

Accordingly this proof does not eliminate all counterexamples. It shows that
an all-size proof can concentrate on form-rigid orthogonality configurations
with the stated local spanning condition. A quantitative estimate or a further
rank-preserving deformation is still required in that case. No novelty claim
or EHZ-capacity consequence is made.
