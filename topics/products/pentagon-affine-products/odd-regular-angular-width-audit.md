# Independent audit: all odd regular rotation profiles

19 October 2026. Reviewer `/root/plateau_check`, independent of author
`/root/odd_pair`. Target: [the complete proof](odd-regular-angular-width.md),
SHA-256 `4b95cadf428b81c37c8b2a776805728a529bf102249b823e0de849b53d31b434`.

**Verdict: PASS**, including the sharp systolic corollary in Section 8.
No material mathematical gap was found. This is agent review conditional on
the imported HK formula and maintained product reductions; it does not supply
human proof acceptance or a literature-wide novelty assessment.

## Proof and interfaces checked

The circle-deficit bound has the correct labeling and factor. Translating the
point triangle leaves its pairing against a closed partner triple unchanged.
In the obtuse/right case the midpoint-circle distances give
`2|z|<=rS−m(r−d)<=SF/2`, hence `|z|<=SF/4`. The actual minimum-step bound
and the free parameter bound have the correct inequality relation. All twelve
orders are covered by permutations; exchanging factors is legitimate after
conjugation and relabeling.

The extremal-angle reduction is valid: at fixed longest side and perimeter,
increasing the other-side difference increases the median distance while
decreasing the smaller endpoint angle and the largest angle. It reaches either
the minimum endpoint angle or the right-angle boundary, whose maximum is also
attained in the minimum-endpoint-angle family. The half-angle side and median
formulas are correct.

The scalar inequality is proved on its complete stated domain. Its slack is
concave in the partner parameter, the quantity isolated before squaring is
positive, and clearing by `4(t+q)^2/q^2` preserves equivalence. I independently
expanded the entire polynomial with exact rational coefficients; all five
displayed coefficient polynomials and the median identity agree. Their
rational endpoint bounds, including the control of the possibly negative
quadratic coefficient by a positive quadratic, are valid.

The acute circumradius bound follows from the extreme points of the bounded
angle domain. Its conversion to g(q), the bound tan(pi/14)<23/100, and the
partner minimum-side estimate are correct. Thus the chosen free parameter
always satisfies the circle-deficit hypothesis.

For n>=m>=7, the closure triangles have the stated perimeter and minimum
angle. Both acute and obtuse cases give the strict normalized disk bound
`|z|/rho<cos(pi/(2n))`; since lcm(n,m)>=n, this suffices for the four-block
incircle. The normalization of rho, first-factor step perimeter and reciprocal
capacity all agree with the maintained decomposition.

The cases with a pentagon are exhaustive: the maintained 5×5 and 5×7 proofs,
the reviewed 5×9 norm estimate, and the pentagon deficit for every odd m>=11.
All displayed rational comparisons are valid, including the non-strict
`v<=tan(pi/22)` at m=11. The result therefore gives the claimed capacity
formula for every odd n,m>=5. Strict radial containment is established except
for the separately handled 5×5 case. The stated labelled-neighborhood
corollary has the required nondegeneracy hypotheses and correct scope.

## Sharp systolic corollary

The area and capacity formulas give exactly `2f(n)f(m)sec²d_L(theta)`.
For x=pi/j, independent differentiation yields

```
(log F)'=1/x−2tan(x/2)−2cot(2x)
        =1/x−csc x−tan(x/2)+tan x.
```

The first strict estimate follows from `1/x>cot x`, equivalent on this
domain to `tan x>x`. The second follows from
`tan x>2tan(x/2)`. Hence F increases with x and f strictly decreases for real
j>=5. Combining this with `d_L<=pi/(2L)<=pi/10` proves the sharp bound
`(3+sqrt(5))/5`. Equality forces n=m=5 and then exactly
`theta=pi/10+k pi/5`; those angles attain equality by the pentagon profile.
This conclusion is restricted to the rotated odd regular class.

## Sources and limits

The audit checked the complete final proof and the relevant local arguments
and conventions in [research.tex](research.tex), its
[HK source contract](source-contract.md), and the separately reviewed
[nonagon norm proof](nonagon-angular-width.md). It did not repeat the primary
HK literature audit or search exhaustively for prior results. No new numerical
campaign or floating-point computation is a proof input. Angular containment
does not establish independent-affine tensor containment, and no manuscript
selection follows from this review.
