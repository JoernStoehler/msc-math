# Equality-preserving Minkowski additions with ten distinct directions

29 October 2026. Original continuation work by `/root/rank_four`, with the
geometric interpretation and the Lagrangian-addition lemma developed jointly
with `/root/rank_four/paired_inverse`. **Scoped independent mathematical audit passed; see `equality-audit.md`.**
The exact checker establishes the stated polynomial identities; the all-size
rank-four inequality remains open.

## Two explicit families

Use the alternating form with matrix

```
A = [[ 0, 1, 2, 2],
     [-1, 0, 2,-2],
     [-2,-2, 0, 1],
     [-2, 2,-1, 0]].
```

It satisfies `A^2=-9I` and `Pf(A)=9`. Put `w_i=-Ae_i/3` and let `Z_0` have
half-generators `e_1,...,e_4,w_1,...,w_4`. Its pairing matrix is
`[[A,3I],[-3I,A]]`, with `Phi(Z_0)=32`, `Psi(Z_0)=256`.

For `s>=0`, define `Z_s=Z_0+sP`, where the four half-generators of `P` are

```
p_1=(6,0,0,0),    p_2=(-2,0,4,-4)=6w_2,
p_3=(0,4,-1,-1),  p_4=(0,0,3,3).
```

The first two add length to existing generator directions; the last two are
new. The only nonzero pairings of `P` are
`omega(p_1,p_4)=72` and `omega(p_2,p_3)=-72`. Thus `P` itself is a balanced
symplectic parallelotope and an equality case. For its sum with `Z_0`, exact
coefficient expansion gives

```
Phi(Z_s) = 32 + 168s + 144s^2,
Psi(Z_s) = (16 + 84s + 72s^2)^2.
```

Consequently `4Psi(Z_s)=Phi(Z_s)^2` for all `s>=0`.

A second equality ray has the same first two generators but replaces the
last two by

```
p_3=(4,0,1,-1),   p_4=(0,0,3,-3).
```

Now all four half-generators of `P` lie in the Lagrangian plane
`L=span(e_1,(0,0,1,-1))`. For this family,

```
Phi(Z_s) = 32 + 168s,
Psi(Z_s) = (16 + 84s)^2.
```

For every `s>0`, each family has ten distinct generating directions. All
original eight directions remain, and every triple of those eight is
linearly independent. Therefore these zonotopes cannot be linear products
of two planar bodies: such a product could place at most two of the original
eight directions in each factor plane.

These are genuine additions of directions, not splitting parallel copies.
They disprove an equality-classification conjecture restricted to products
and the original eight-direction paired-inverse family, but no such
classification had been claimed by the retained packet.

## Normalization and exact verification

The function `Psi` here is the sum of absolute four-Pfaffians. Since
`Pf(A)=9`, ordinary Euclidean volume is `16Psi/9`. With symplectic volume
form `omega^2/2`, volume is `16Psi`. No capacity statement follows.

Run the standard-library checker from this directory:

```
python equality-extension.py
```

For each family it expands all 45 pair terms and all 210 four-Pfaffian terms
as rational-coefficient polynomials. The ten generator weights are
`1+6s` on `e_1,w_2`, `1` on the other six old directions, and `s,3s` on the
two listed new primitive directions. These weights are nonnegative for every
`s>=0`, so their products factor out of each absolute value. The resulting
coefficient identity proves the claim for the whole interval, not merely
sampled parameters. The checker also verifies the ten directions are
pairwise distinct and all original triples are independent.

## Discovery and remaining discriminator

The equality base has 32 extreme directions up to sign (64 vertices of its
unit ball) for the norm
`a(u)=||Au||_1+3||u||_1`. Each satisfies `b(u)=16a(u)`, where `b` is the sum
of absolute three-old-plus-new Pfaffians. This was checked by enumerating
rank-three intersections among its eight norm hyperplanes, with exact integer
cofactors. The directional equality does not hold on every direction in R4.

The quadratic deficit on the 32 contact directions up to sign has negative eigenvalues, but
new segment weights are constrained to be nonnegative. Eliminating signed
length changes of the original eight directions gives a 24-variable
quadratic form with twelve exact zero pairs. Six corresponding rays have
nonnegative original changes and give exact equality families; the two above
are representatives of their distinct geometric types. The other six are
locally feasible signed variations with a positive fourth-order deficit.
These facts alone do not prove copositivity or local maximality.

Exact bivariate expansion now proves that every sum of two distinct members
of the six nonnegative equality additions has positive deficit whenever
both parameters are positive. All 15 deficit polynomials have nonnegative
coefficients. The saved `contact-calculus.json` and `contact-replay.py` retain
and independently replay those rational coefficients, the 32 contact directions up to sign
and the quadratic form. Three-or-more additions and general copositivity
remain separate questions.
A general Lagrangian-addition criterion explains the second family and is
being retained separately. Further iteration may expose a broader equality
mechanism; no unrestricted conclusion is claimed.
