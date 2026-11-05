# Candidate comparison

IDs are anonymized. Some candidates may be unchanged source; no preference is required. Source text is separately available for meaning checks.

# HKO prose slots

## Candidate J


### Opening: before “Differentiated closure gives”

Starting from the polygon product, vary the first momentum component of the second q-facet normal. In the description \(K(a)=\{x:a_i\cdot x\leq1\}\), this means perturbing \(a_2\) in the \(p_1\)-direction while leaving every other normal fixed. With coordinates ordered as \((q_1,q_2,p_1,p_2)\), take \(h_2=(0,0,1,0)\) and \(h_i=0\) for \(i\ne2\). This is flat coordinate 6 in the verifier’s zero-based storage. Put \(w=c+kr=\beta_{0,1}\).

### Conclusion: after the DU formula

The resulting coefficient of the upper-branch row is approximately \(-0.1801707325\). It is a derivative of \(U\), the smooth feasible-section upper bound for the systolic ratio, and does not by itself give a derivative of capacity. Varying the two polygon factors keeps every q-facet normal in the q-plane. The direction used here instead adds a \(p_1\)-component to one such normal, so factor variations cannot detect this coefficient. The forty-coordinate calculation includes this additional direction.


## Candidate L


Vary the first momentum component of the second \(q\)-facet normal. In coordinates \((q_1,q_2,p_1,p_2)\), take \(h_2=(0,0,1,0)\) and \(h_i=0\) for \(i\ne2\). This is flat coordinate 6 in the verifier’s zero-based storage. Put \(w=c+kr=\beta_{0,1}\).

## HKO: replacement final prose after the DU formula

This is an actual coefficient of the upper-branch row for the smooth feasible-section upper bound \(U\), approximately \(-0.1801707325\); it is not an assertion about the derivative of the actual capacity. Varying the two polygon factors keeps each \(q\)-facet normal in the \(q\)-plane. The perturbation above instead adds a \(p_1\) component to one such normal, so the forty-coordinate calculation includes a direction that differentiating only within products would miss.


## Candidate K

Opening: Take h2 = (0, 0, 1, 0) and hi = 0 for i ≠ 2: vary the first momentum coordinate of the second q-facet. This is flat coordinate 6 in the verifier’s zero-based storage. Put w = c + kr = β0,1.

Conclusion: This is an actual coefficient of the upper-branch row, approximately −0.1801707325. It illustrates why the forty-coordinate calculation is stronger than differentiating only within products.

# DS candidates

## Candidate M


# Choosing and changing examples

We used data science to search for convex bodies with large systolic ratios. The exploratory route was to sample varied random polytopes and compute readily available geometric measurements, both symplectic and non-symplectic. Temporarily setting their meanings aside lets us look for statistical patterns; interpreting those patterns geometrically can then suggest tentative conjectures and questions for rigorous explanation. The experiments below report a descriptor association and tests of ways to choose and change examples within the AI-assisted exploration.

For a convex body \(K\subset\mathbb R^4\), write

\[
\operatorname{sys}(K)=\frac{c(K)^2}{2\operatorname{vol}(K)},
\]

where \(c(K)\) is its Ekeland–Hofer–Zehnder capacity and \(\operatorname{vol}\) denotes four-dimensional volume. The search sought values above 1. Multiplying all coordinates by \(t>0\) multiplies capacity by \(t^2\) and volume by \(t^4\), so the ratio does not change. Simply making a body larger cannot help.

## Sampling and finding a descriptor

Many of the examples were products of two planar polygons. In coordinates \((q_1,q_2,p_1,p_2)\), we formed

\[
K=P\times Q,\qquad
P=\{q\in\mathbb R^2:\langle n_i,q\rangle\le h_i,\ i=1,\ldots,k\},
\]

with an analogous description of \(Q\) using \(m\) inequalities. The unit normals \(n_i\) came from independent uniform angles, and the heights \(h_i\) were sampled independently from \([0.8,1.2)\). We retained bounded polygons for which every proposed side was present. Thus a \(k\times m\) sample means a product with \(k\) sides in the first factor and \(m\) in the second; the rejection rule is part of its distribution. The computations below report numerical ratios, obtained by substituting computed capacity and volume into the displayed formula. The new experiments bound the error in capacity, but their volume calculation still uses floating-point arithmetic.

A useful descriptor emerged from the two-dimensional faces of these bodies. Such a face is called a ridge in dimension four. If a ridge \(R\) has cyclically ordered vertices \(v_0,\ldots,v_{r-1}\), its absolute symplectic area is

\[
A_\omega(R)=\frac12\left|\sum_{i=0}^{r-1}\omega_0(v_i,v_{i+1})\right|,
\qquad v_r=v_0,\qquad
\omega_0=\sum_{j=1}^2 dq_j\wedge dp_j.
\]

This is the polygonal area formula with the symplectic pairing in place of the planar determinant. It measures the area seen by \(\omega_0\), which can vanish even when the face has positive Euclidean area. Summing over the ridges gives the descriptor

\[
S(K)=\frac{\sum_R A_\omega(R)}{\sqrt{\operatorname{vol}(K)}}.
\]

Both numerator and denominator scale by \(t^2\), so \(S\) is unchanged by a common dilation. It is also unchanged by translation and linear symplectic maps.

In the original samples, smaller values of \(S\) tended to accompany larger ratios. The association persisted when products with the same numbers of sides were considered separately. This suggested a simple use: generate many bodies, compute their ridge areas, and select those with low \(S\) before calculating their capacities. Tests on fresh candidates showed that low-ridge selection could raise the ratios relative to controls. That success supplied a way to choose examples. It did not yet tell us which change to make to a given body, or give a rigorous explanation of the association.

## Changing support heights

One concrete change is to replace all support heights of a polygon by 1, keeping its normals fixed. Every resulting side lies on a tangent line to the unit circle. We tested this operation on sixteen independently generated pairs of polygons: eight of type \(4\times4\) and eight of type \(4\times6\). Each pair supplied four bodies: the original product, the product with only the first factor changed, the product with only the second changed, and the product with both changed. We accepted a pair only if all four constructions were valid, before evaluating any of their ratios. The comparison therefore concerns this jointly admissible sample.

Each planar factor was then scaled to area one. This normalization does not alter the mathematical ratio: separate factor dilations have the form \(\operatorname{diag}(aI_2,bI_2)\), which is a common dilation composed with the symplectic map \(\operatorname{diag}(\sqrt{a/b}\,I_2,\sqrt{b/a}\,I_2)\). The shape change comes from replacing unequal heights, not from adjusting the areas. Changing both factors increased the mean numerical ratio by 0.0153. Eleven pairs improved and five became worse. The mean increase was about 0.004 for \(4\times4\) and 0.027 for \(4\times6\). These paired comparisons show why an average gain should not become a rule that the operation always helps: even with the same normals held fixed, its effect varied across bodies.

## Changing symplectic alignment by rotation

Rotation asks a different question. An orthogonal map preserves Euclidean shape, but it need not preserve \(\omega_0\). Rotating a Lagrangian product can therefore change its capacity; the rotated body need no longer be a Lagrangian product for the fixed symplectic form. This gives a way to explore symplectic alignment without changing the body’s Euclidean geometry.

To test whether that freedom was worth using in a search, we compared two allocations of sixteen evaluations. One used a single fresh product, one symplectic rotation as a control, and fourteen prescribed orthogonal rotations. The other evaluated sixteen fresh products, sharing the first body with the rotation experiment. We made this comparison four times, twice for \(3\times3\) and twice for \(4\times4\). The rotations and all source bodies were fixed before their ratios were evaluated; the symplectic controls preserved the ratios to within \(4\cdot10^{-16}\).

Rotation improved three of the four starting bodies. Nevertheless, fresh sampling produced the larger maximum in every comparison. In one \(4\times4\) case, rotation raised the ratio from 0.328 to 0.529, while the sixteen fresh products reached 0.774. The rotation calculations also took longer. For these four starting bodies, spending sixteen evaluations on fresh products was more effective than spending them on the prescribed rotations.


## Candidate N

                           Choosing and changing examples
We used data science to search for convex bodies with large systolic ratios. One approach
selected bodies using geometric quantities that were easier to compute than capacity. Another
changed bodies already sampled, for example by moving their supporting hyperplanes or rotating
them. The experiments below examine both approaches within the AI-assisted exploration.
   For a convex body K ⊂ R4 , write
                                                    c(K)2
                                        sys(K) =            ,
                                                   2 vol(K)
where c(K) is its Ekeland–Hofer–Zehnder capacity and vol denotes four-dimensional volume.
The search sought values above 1. Multiplying all coordinates by t > 0 multiplies capacity by
t2 and volume by t4 , so the ratio does not change. Simply making a body larger cannot help.
   Many of the examples were products of two planar polygons. In coordinates (q1 , q2 , p1 , p2 ),
we formed
                  K = P × Q,        P = {q ∈ R2 : ⟨ni , q⟩ ≤ hi , i = 1, . . . , k},
with an analogous description of Q using m inequalities. The unit normals ni came from
independent uniform angles, and the heights hi were sampled independently from [0.8, 1.2). We
retained bounded polygons for which every proposed side was present. Thus a k × m sample
means a product with k sides in the first factor and m in the second; the rejection rule is part
of its distribution. The computations below report numerical ratios, obtained by substituting
computed capacity and volume into the displayed formula. The new experiments bound the
error in capacity, but their volume calculation still uses floating-point arithmetic.
   A useful descriptor emerged from the two-dimensional faces of these bodies. Such a face is
called a ridge in dimension four. If a ridge R has cyclically ordered vertices v0 , . . . , vr−1 , its
absolute symplectic area is

                        1 r−1
                          X                                              2
                                                                         X
               Aω (R) =       ω0 (vi , vi+1 ) ,       vr = v0 ,   ω0 =         dqj ∧ dpj .
                        2 i=0                                            j=1

This is the polygonal area formula with the symplectic pairing in place of the planar determinant.
It measures the area seen by ω0 , which can vanish even when the face has positive Euclidean
area. Summing over the ridges gives the descriptor
                                               P
                                                  Aω (R)
                                        S(K) = pR        .
                                                      vol(K)

Both numerator and denominator scale by t2 , so S is unchanged by a common dilation. It is
also unchanged by translation and linear symplectic maps.
   In the original samples, smaller values of S tended to accompany larger ratios. The association
persisted when products with the same numbers of sides were considered separately. This
suggested a simple use: generate many bodies, compute their ridge areas, and select those
with low S before calculating their capacities. Tests on fresh candidates showed that low-ridge
selection could raise the ratios relative to controls. That success supplied a way to choose
examples. It did not yet tell us which change to make to a given body.
   One concrete change is to replace all support heights of a polygon by 1, keeping its normals
fixed. Every resulting side lies on a tangent line to the unit circle. We tested this operation on
sixteen independently generated pairs of polygons: eight of type 4 × 4 and eight of type 4 × 6.
Each pair supplied four bodies: the original product, the product with only the first factor
changed, the product with only the second changed, and the product with both changed. We
accepted a pair only if all four constructions were valid, before evaluating any of their ratios.
The comparison therefore concerns this jointly admissible sample.

                                                  1
   Each planar factor was then scaled to area one. This normalization does not alter the
mathematical ratio: separate factor dilations havepthe formpdiag(aI2 , bI2 ), which is a common
dilation composed with the symplectic map diag( a/b I2 , b/a I2 ). The shape change comes
from replacing unequal heights, not from adjusting the areas. Changing both factors increased
the mean numerical ratio by 0.0153. Eleven pairs improved and five became worse. The mean
increase was about 0.004 for 4 × 4 and 0.027 for 4 × 6. These paired comparisons show why an
average gain should not become a rule that the operation always helps: even with the same
normals held fixed, its effect varied across bodies.
   Rotation asks a different question. An orthogonal map preserves Euclidean shape, but it
need not preserve ω0 . Rotating a Lagrangian product can therefore change its capacity; the
rotated body need no longer be a Lagrangian product for the fixed symplectic form. This gives
a way to explore symplectic alignment without changing the body’s Euclidean geometry.
   To test whether that freedom was worth using in a search, we compared two allocations of
sixteen evaluations. One used a single fresh product, one symplectic rotation as a control, and
fourteen prescribed orthogonal rotations. The other evaluated sixteen fresh products, sharing
the first body with the rotation experiment. We made this comparison four times, twice for
3 × 3 and twice for 4 × 4. The rotations and all source bodies were fixed before their ratios
were evaluated; the symplectic controls preserved the ratios to within 4 · 10−16 .
   Rotation improved three of the four starting bodies. Nevertheless, fresh sampling produced
the larger maximum in every comparison. In one 4 × 4 case, rotation raised the ratio from 0.328
to 0.529, while the sixteen fresh products reached 0.774. The rotation calculations also took
longer. For these four starting bodies, spending sixteen evaluations on fresh products was more
effective than spending them on the prescribed rotations.




                                              2

## Candidate P


# Choosing examples, then testing changes

We used data science to search for convex bodies with large systolic ratios. For a convex body \(K\subset\mathbb R^4\), the ratio is

\[
\operatorname{sys}(K)=\frac{c(K)^2}{2\operatorname{vol}(K)},
\]

where \(c(K)\) is its Ekeland–Hofer–Zehnder capacity and \(\operatorname{vol}\) is four-dimensional volume. The search sought values above 1. A common dilation by \(t>0\) multiplies capacity by \(t^2\) and volume by \(t^4\), leaving the ratio unchanged. Size alone therefore offers no way to improve it.

The exploratory route was to sample varied random polytopes and compare their ratios with readily available geometric measurements, both symplectic and non-symplectic. We could temporarily set the meanings of those measurements aside and look for statistical patterns. A promising pattern would then need a geometric interpretation: why might the measured quantity matter, and could it help choose or change examples? The longer-term aim was to turn such interpretations into tentative conjectures and seek rigorous explanations. The AI-assisted experiments reported here reach the earlier stages of that route: a descriptor associated with high ratios, tests of its use for selection, and two experiments on changing bodies.

## From measurements to selection

Many examples were products of planar polygons. In coordinates \((q_1,q_2,p_1,p_2)\), we formed

\[
K=P\times Q,\qquad
P=\{q\in\mathbb R^2:\langle n_i,q\rangle\leq h_i,
\ i=1,\ldots,k\},
\]

with \(Q\) described by \(m\) analogous inequalities. The unit normals came from independent uniform angles, and the heights were sampled independently from \([0.8,1.2)\). We retained bounded polygons in which every proposed side was present. Thus a \(k\times m\) sample has \(k\) sides in the first factor and \(m\) in the second; this rejection rule is part of the sampling distribution.

The ratios below are numerical values obtained from computed capacities and volumes. The new experiments bound the capacity error, but volume is still computed in floating-point arithmetic.

One useful descriptor came from the two-dimensional faces, called ridges in dimension four. For a ridge \(R\) with cyclically ordered vertices \(v_0,\ldots,v_{r-1}\), define its absolute symplectic area by

\[
A_\omega(R)=\frac12\left|\sum_{i=0}^{r-1}\omega_0(v_i,v_{i+1})\right|,
\qquad v_r=v_0,\qquad
\omega_0=\sum_{j=1}^{2}dq_j\wedge dp_j.
\]

Summing these areas and normalizing gives

\[
S(K)=\frac{\sum_R A_\omega(R)}{\sqrt{\operatorname{vol}(K)}}.
\]

In the original samples, smaller \(S\) tended to accompany larger systolic ratios. The association persisted within groups having the same numbers of sides in their two factors.

The geometry of \(S\) makes this pattern worth examining. The formula for \(A_\omega\) replaces the planar determinant in polygonal area by the symplectic pairing. A ridge can therefore have positive Euclidean area but zero symplectic area. Both the numerator and denominator of \(S\) scale by \(t^2\); the descriptor is unchanged by a common dilation, by translation, and by linear symplectic maps.

The association suggested a practical selection test: generate many bodies, compute their ridge areas, and calculate capacities for those with low \(S\). On fresh candidates, this selection could raise ratios relative to controls. It supplied a way to choose examples, but no prescription for changing a given body. We examined that separate question through support heights and rotations.

## Changing support heights

One possible change is to replace all support heights of a polygon by 1 while keeping its normals fixed. Its sides then lie on tangent lines to the unit circle. We tested this operation on sixteen independently generated pairs of polygons: eight of type \(4\times4\) and eight of type \(4\times6\).

Each pair supplied four products: the original, one with only the first factor changed, one with only the second changed, and one with both changed. Before evaluating any ratios, we accepted a pair only if all four constructions were valid. The comparison therefore applies to this jointly admissible sample.

We then scaled each planar factor to area one. This leaves the mathematical ratio unchanged: separate factor dilations \(\operatorname{diag}(aI_2,bI_2)\) are a common dilation composed with the symplectic map

\[
\operatorname{diag}\!\left(\sqrt{a/b}\,I_2,\sqrt{b/a}\,I_2\right).
\]

Thus replacing unequal support heights produces the shape change; the area normalization does not account for a change in ratio.

Changing both factors increased the mean numerical ratio by 0.0153. Eleven pairs improved and five became worse. The mean increase was about 0.004 for \(4\times4\) and 0.027 for \(4\times6\). The average gain consequently does not support a rule that the operation always helps: even with the normals fixed, its effect varied across bodies.

## Changing symplectic alignment

Rotation offers a different experiment. An orthogonal map preserves Euclidean shape but need not preserve \(\omega_0\). A rotated Lagrangian product can cease to be a Lagrangian product for the fixed symplectic form, and its capacity can change. Rotations therefore let us explore symplectic alignment while holding Euclidean geometry fixed.

To test whether this freedom was useful for the search, we compared two allocations of sixteen evaluations. The first evaluated a fresh product, one symplectic rotation as a control, and fourteen prescribed orthogonal rotations. The second evaluated sixteen fresh products, sharing its first body with the rotation experiment. We ran four comparisons, twice for \(3\times3\) and twice for \(4\times4\). All source bodies and rotations were fixed before evaluation. The symplectic controls preserved the ratios to within \(4\cdot10^{-16}\).

Rotation improved three of the four starting bodies, but fresh sampling produced the larger maximum in all four comparisons. In one \(4\times4\) case, rotation increased the ratio from 0.328 to 0.529, whereas sixteen fresh products reached 0.774. The rotation calculations also took longer. For these four starting bodies and prescribed rotations, fresh sampling was the more effective use of sixteen evaluations.

Together, the experiments separate a useful selection pattern from proposed ways to improve a body. Low-ridge selection could help on fresh candidates; equalizing support heights helped on average but sometimes hurt; and the tested rotations lost to fresh sampling at the same evaluation count. These observations provide questions for geometric explanation. They do not establish a general improvement rule or a rigorous explanation of the descriptor association.


