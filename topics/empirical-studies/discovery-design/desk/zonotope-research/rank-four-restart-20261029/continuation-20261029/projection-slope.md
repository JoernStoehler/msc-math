# Matched Lagrangian projection: an affine deficit and a linear-space discriminator

29 October 2026. Working derivation, not yet independently audited. This
continues the proved equality-addition mechanism but **does not prove** the
unrestricted rank-four inequality or the slope inequality below.

Use symplectic volume `mu=omega^2/2`. Fix a Lagrangian plane `L` with an
oriented area form, and define `q(z) in L` by

`det_L(h,q(z))=omega(h,z)` for every `h in L`.

Let `Q=sum_i[-q(z_i),q(z_i)]`, the rotated dual projection of the zonotope
`Z=sum_i[-z_i,z_i]`. Set

`M=sum_{i,j}|omega(z_i,q(z_j))|`,
`B=sum_j b_Z(q(z_j))`,
`Delta=Phi(Z)^2-4Psi(Z)`.

The general expansion in `lagrangian-addition.md`, with `P=Q`, gives

`Phi(Z+tQ)=Phi(Z)+Mt`,
`Psi(Z+tQ)=Psi(Z)+Bt+(M^2/4)t^2`,
`Delta(Z+tQ)=Delta+(2Phi(Z)M-4B)t`.

For the last coefficient, `M=sum_{i,j}|det(q(z_i),q(z_j))|=area(Q)/2`,
so the two-added-vector contribution `area(Q)^2/16` is `M^2/4`.
The formulas hold for every `t>=0`, with no equality/contact assumption.
If `M>0`, the ratio `4Psi/Phi^2` tends to one as `t->infinity`.

Consequently a single exact pair `(Z,L)` with

`B>Phi(Z)M/2`

would generate counterexamples at all sufficiently large parameters. The
universal target implies the opposite slope inequality, but no converse is
established here. A reverse addition could support induction only if the
matched projection directions already occur among the removable generators;
that hypothesis is not automatic, even at the retained eight-direction
equality body.

## Exterior-algebra version

Let `W_ij=omega(z_i,z_j)` and `C_ij=omega(z_i,q(z_j))`. Then `C` is a
rank-at-most-two skew matrix, `W wedge C=0`, and `M=2||C||_1`. Conversely,
for fixed decomposable `C=g wedge h`, the linear constraint `W wedge C=0`
is equivalent to

`W=g wedge r+h wedge s`

for some vectors `r,s` (when `g,h` are independent). In particular every
such `W` already has rank at most four. The proposed slope estimate is

`sum_l ||W wedge C_col(l)||_1 <= ||W||_1 ||C||_1`.       (S)

The exterior norm on the left sums triples. Explicitly its summands are

`|W_ij C_kl-W_ik C_jl+W_jk C_il|`, `i<j<k`, all `l`.

Thus for fixed `C`, (S) is an induced `l1`-operator-norm bound on a linear
space, rather than an optimization over a nonlinear rank variety. For fixed
signs of the output wedge coordinates its inner optimization is a linear
program subject to `||W||_1<=1`, `W wedge C=0`. Sign-ascent through such LPs
is a bounded candidate search, not a proof of the convex maximum.

## Bounded preliminary diagnostic

The scratch `slope_lp.py` used twelve integral pairs `(g,h)` at sizes eight
and ten, eight numerical starts per pair, with at most twelve sign-LP steps
per start. It found no ratio above one; the largest returned ratio was
`0.9921924333525527`. This is finite numerical evidence only. The seed,
inputs and returned matrices are retained in `slope-lp-results.json` so an
independent worker can avoid repeating the same initial probe. No exact
counterexample or dual proof was obtained.

The next useful step is an exact dual/cancellation proof of (S), or an exact
negative-slope witness; more generic random starts do not establish either.
