# Independent audit: two regular nonagons

19 October 2026. Reviewer: `/root/plateau_check`, independently of the proof
author `/root/odd_pair`. Target: [the nonagon angular-width proof](nonagon-angular-width.md).

**Verdict: PASS.** No material mathematical gap was found in the complete
argument. This is agent proof review, conditional on the imported capacity
formula and maintained sparse-product reduction; human acceptance and novelty
remain unestablished.

The proof gives the exact capacity profile for two regular circumradius-one
nonagons at every relative angle and strict containment of all six-block
coefficients in the four-block incircle. It consequently supports an open
labelled-polygon neighborhood through the maintained stability proposition.
It does not establish independent-affine nonagon tensor containment.

## Checks

The zero-sum triple estimate is correct for arbitrary complex triples:

```
z=(x1 conjugate(y1)-x3 conjugate(y2))/2,
|z|² <= (Σ|xi|²)(Σ|yj|²)/12.
```

An independent matrix verification uses A with A11=1, A32=−1 and other
entries zero, and P=I−11ᵀ/3. Then

```
PAP = (1/3) [[1,0,−1],[−1,1,0],[0,−1,1]],
(PAP)(PAP)ᵀ=P/3.
```

Its norm on the complex sum-zero plane is 1/sqrt(3), which gives the stated
1/12 after z's factor 1/2. The proof's orthonormal-coordinate derivation agrees.
Permutations preserve sums and squared norms, so the estimate covers all
twelve cyclic six-block orders.

Positive nonagon closure gaps are integers adding to nine and at most four.
The unordered types (1,4,4), (2,3,4), (3,3,3) are exhaustive; the scalene type
has two cyclic orientations with the same squared step norm. Oddness excludes
two-facet closures. Thus no closure vertex is omitted.

For h=cos(pi/9), s=1/(1+h), all types satisfy Σ|xi|²<(5/3)s². The
isosceles and equilateral calculations are correct. For the scalene type, the
angles pi/9, pi/3, 5pi/9 give the stated sine sum and squared sine sum.
The rational norm expression has positive denominator 3(4h²−1)². Independent
exact polynomial reduction modulo 8h³−6h−1 gives precisely

```
5h²(48h⁴−24h²+3)−3(1+h)²(16h⁴−16h²+9)
 =3(14h²−5h−7).
```

The estimate h>cos(pi/8)>11/12 and the stated rational endpoint comparisons
prove the needed strict positivity.

With rho=s²/2, the triple estimate gives |z/rho|²<25/27, while the squared
normalized four-block inradius is (1+h)/2>23/24>25/27. The reciprocal capacity
normalization is correct: Qmax=rho cos d9 and c=1/(2Qmax). The strict gap
excludes genuine six-block sparse maximizers even at angular switches; the
proof correctly makes no classification of all minimizers.

The stability hypotheses hold for the regular pair: the dual vertices lie on
a circle, no three are collinear, no two are opposite, and the origin lies on
no line through a pair. The maintained strict-containment proposition therefore
applies. The general sufficient criterion M_A M_B<3r² is also correct; only
genuine three-facet closure pairs can produce six-block words.

## Exact limit of the sufficient criterion

The final infinite-family obstruction is valid. For n=4k+3>=23, the positive
closure gaps (k+1,k+1,2k+1) have angles a,(pi−a)/2,(pi−a)/2. Their sine-rule
side ratios are 2u,1,1, where u=sin(a/2). Hence their normalized squared step
norm is exactly the proof's expression. The facts h_n<1, u<1/13, and

```
d/du [(u²+1/2)/(1+u)²]=(2u−1)/(1+u)³<0
```

give N>171/98>sqrt(3). Thus N²/3>1 for the equal-factor pair, while the
normalized squared incircle radius is below one. This proves failure of the
aggregate-norm sufficient criterion for that infinite family. It says nothing
adverse about actual six-block containment, as the proof explicitly states.

## Sources and access limits

Reviewed the complete elementary proof and the relevant local sparse,
four-/six-block and stability arguments in [research.tex](research.tex),
together with its [source contract](source-contract.md). The contract records
the Haim–Kislev Theorem 1.1 convention bridge. This check verified the local
mathematical interfaces; it did not repeat the primary HK source audit or an
exhaustive literature search. The elementary proof and this audit have no
numerical-certificate dependency.
