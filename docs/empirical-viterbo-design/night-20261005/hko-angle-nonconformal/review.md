# Independent review of the HKO-angle nonconformal slice

Receiving lead: `direct_cover_lead`. New slice only; the accepted aligned
theorem and its retained evidence are unchanged. Reviewed the selected
`../comparison/hko-angle-contract.md`. This initial note verifies coordinates,
normalization, and transfer of the vertex theorem. The completed candidate
audit below establishes the sharp bound and equality classification,
conditional on the separately owned complete denominator envelopes.

## Covectors and determinant

Let \(\theta=\pi/10\), \(\tau^2=5-2\sqrt5\), and
\(T=\operatorname{diag}(1,\tau)\). Computational points are
\(q_c=T^{-1}q_{\rm phys}\), so computational covectors are
\(n_c=T n_{\rm phys}\): their pairing is unchanged. Put
\(u=\tan\theta/\tau=1/\sqrt5\). Then the scaled physical covector rotation is

\[
T\frac{R_\theta}{\cos\theta}T^{-1}
=\begin{pmatrix}1&-u\\\tau^2u&1\end{pmatrix}=C.
\]

This is the correct covector matrix. The point-coordinate conjugate has
off-diagonal entries \(-\tau^2u,u\), and would be incorrect here.

Since \(G_x\) commutes with \(T\), the computational normal transform
\(G_x C\) is the conjugate of the physical transform

\[
M'=G_xR_\theta/\cos\theta.
\]

Its determinant is

\[
\det M'=x d,\qquad
d=\det C=1+\tau^2u^2=2-\frac2{\sqrt5}=\sec^2\theta>0.
\]

All these coefficients belong to \(\mathbb Q(\sqrt5)\). Replacing physical
normals \(G_xR_\theta n_i\) by \(M'n_i\) does not change the arbitrary-support
family: divide the corresponding physical support offsets by the same
positive \(\cos\theta\). For fitting computations, the new offsets and all
template support evaluations must use this same scaled convention.

## Area and product normalization

For physical K defined with normals \(M'n_i\), set
\(Y_{\rm phys}=M'^T K\). Exactly, its inequalities become
\(n_i\cdot y\le h_i\), and

\[
\operatorname{area}K=\frac{\operatorname{area}Y_{\rm phys}}{xd}.
\]

With \(a=\cos(2\pi/5)=(\sqrt5-1)/4\), canonical supports give

\[
P(h)=\sum_i h_i h_{i+1}-a\sum_i h_i^2,\qquad
\operatorname{area}Y_{\rm phys}=\frac{P(h)}{\sin(2\pi/5)}.
\]

Therefore

\[
\operatorname{area}K=\frac{P(h)}{xd\sin(2\pi/5)}.
\tag{1}
\]

The same identity can be checked wholly in computational coordinates:
\((G_xC)^Tq_c=T^{-1}M'^Tq_{\rm phys}\), so computational Y area is physical
Y area divided by \(\tau\), computational K area is that area divided by
\(xd\), and multiplying K area by \(\tau\) recovers (1). Thus there is no
unaccounted coordinate Jacobian.

Regular Q has physical area \(\tfrac52\sin(2\pi/5)\). At complete capacity
one the product systolic ratio is consequently

\[
\operatorname{sys}=\frac1{2\operatorname{area}K\operatorname{area}Q}
=\frac{xd}{5P(h)}.
\tag{2}
\]

This matches the contract's target. A claimed HKO bound
\(\operatorname{sys}\le(3+\sqrt5)/5\) is equivalent to
\(P(h)-xd/(3+\sqrt5)\ge0\) at capacity one. The determinant \(d\) is
essential: omitting it would test a different ratio.

## Closure and canonical vertex reduction

For every \(x>0\), the physical and computational normal transforms are
invertible. Thus their nonnegative closure cone is precisely that of the
regular normals, with the same five normalized circuit weights. Translating
any genuine template to contain zero makes its transformed support values
positive. Closure makes each weighted sum translation invariant, establishing
positive denominators in the complete fitting formula.

Raw redundant supports must first be lowered to actual canonical supports.
The closed canonical Y type cone remains exactly

\[
h_{i-1}+h_{i+1}-2a h_i\ge0\qquad(i\bmod5),
\]

including zero edges. Its translation quotient has dimension three, and the
gauge \(h_3=h_4=0\) remains valid. At fixed \(x\), capacity at least one
adds five inequalities \(\lambda_l\cdot h\ge D_l(x)\). The complete ten
genuine templates guarantee a contained triangle and hence positive area and
coercivity modulo translation, exactly as in the accepted aligned vertex
review. The fitted Y triangle is now \(M'^T T_j\), still nondegenerate.

Canonical Minkowski linearity on this cone and Brunn–Minkowski equality
therefore imply that every area minimizer at fixed \(x\), modulo translation,
is a vertex of this ten-inequality polyhedron. Every minimizing vertex has
capacity exactly one by homogeneous dilation. Zero-edge candidates remain
legitimate; strict five-sidedness cannot be imposed before enumeration.
The determinant \(xd\) is fixed and positive in each such optimization, so
it does not change this reduction. The same 120-triple upper bound applies;
actual cells, feasible counts, and complete equality cases still require the
new exact arithmetic.

This is a transfer of the independently reviewed canonical-vertex theorem,
not a new unrestricted global argument. Its source is
`../nonconformal-fan/vertex-premise-review.md`.

## Completed exact candidate audit

The new `denominators.json` and `body-candidates.json` were audited with the
retained standard-library rational-pair/Cramer checker `review_candidates.py`.
It imports no producer code and writes no files. Replay:

```sh
python -B docs/empirical-viterbo-design/night-20261005/hko-angle-nonconformal/review_candidates.py
```

The five denominator cells have boundaries
\(0,\sqrt5-2,\sqrt5/5,1,\sqrt5,\infty\). There are exactly 100 nonsingular
and 20 singular triples. The independent calculation reproduced all 500
nonsingular cell systems and every affine support vector, clipped feasibility
interval, and area coefficient in all 65 retained distinct candidate records.
The per-cell (feasible bases, distinct candidates) counts are
\((20,9),(20,17),(20,17),(20,11),(20,11)\).

The exact target is

\[
P(x)-\frac{d}{3+\sqrt5}x\ge0,\qquad
\frac{d}{3+\sqrt5}=2-\frac{4\sqrt5}{5}.
\tag{3}
\]

All 123 necessary endpoint/stationary-minimum checks passed. The checks include
ten singleton intervals and eleven unbounded intervals; infinite tails were
tested by the leading quadratic coefficient and, if it vanishes, the linear
coefficient. Stationary values were calculated independently as the
discriminant expression \((4AC-B^2)/(4C)\) for positive quadratic coefficient
\(C\). No candidate target quadratic is identically zero.

Every zero in these interval tests occurs at \(x=1\). The eight records that
contain such a zero all evaluate to exactly one quotient support class:

\[
h_*=(p,q,p,0,0),\qquad
p=-1+\frac{3\sqrt5}{5},\quad q=1-\frac{\sqrt5}{5}.
\tag{4}
\]

All five edge-length numerators at (4) equal \(p>0\). Since (2) is the edge
formula for the regular unit-normal Y fan, Y is a regular pentagon. At
\(x=1\), \(M'=R_{\pi/10}/\cos(\pi/10)\), so
\(K=\cos(\pi/10)R_{+\pi/10}Y\): the physical equality body is precisely a
regular partner in the selected positive HKO-angle fan. The rotation sign is
positive when recovering K from Y.

The checker also independently intersects successive transformed support
lines from the denominator packet and computes their computational shoelace
area at (4). It gives
\(\operatorname{area}K_c=P_*/[d\tau\sin(2\pi/5)]\), with
\(\tau\sin(2\pi/5)=(5-\sqrt5)/4\) and
\(P_*=d/(3+\sqrt5)\). Multiplying by \(\tau\) verifies the physical Jacobian
and (1) directly at equality.

By the proven all-minimizers-are-vertices implication, (3) supplies
\(\operatorname{sys}\le(3+\sqrt5)/5\) over the whole selected \(x>0\) family.
Its only possible equality parameter is \(x=1\), and its only capacity-one
equality support class is (4), modulo translation. The complete primal/dual
optimizer check described below confirms capacity exactly one, so the bound
is attained and equality consists of translations and positive dilations of
the regular HKO partner. There is no additional nonvertex
equality body and no above-HKO body in this slice. The complete envelope
derivation and optimizer fit certificate remain separately owned inputs;
this audit does not replace those checks with a partial template fit.

## Attainment and physical optimizer certificate

Read-reviewed the completed lead-owned `verify_optimizer.py` and
`optimizer-certificate.json`. The ten primal placements each test all five
halfplanes using the maximum support over all three template vertices. An
active circuit and a template attaining its denominator maximum give the
dual fitting bound of one. These together establish complete capacity exactly
one; no action-only upper bound is being mistaken for a witness capacity.

The retained independent `review_candidates.py` now also reads this optimizer
certificate and independently recalculates all ten translations' five exact
support slacks with rational-pair arithmetic, checking their retained values
and nonnegativity. It independently finds a template whose circuit-zero
weighted support equals the active numerator. The latest read-only replay
passed all candidate checks and reports `equality_ten_primal_and_active_dual:
PASS` as well as the independent shoelace check.

The lead's physical regularity tests are valid: computational point lengths
use the metric \(\operatorname{diag}(1,\tau^2)\), not the computational
Euclidean metric. All five physical sides and all five physical distances
from the vertex-average center are equal and positive. Independently of that
metric test, the exact identity
\(h_i=D+n_i\cdot z_Y\), where \(D=(\sqrt5-1)/5\), proves canonical Y is a
translate of a regular pentagon. Its inverse transformation at \(x=1\) is
\(\cos(\pi/10)R_{+\pi/10}\), confirming the selected positive-angle regular
HKO equality body. Thus the optimizer is attained and its equality
classification is closed, subject only to the separately owned completeness
of the denominator envelopes and imported ten-template capacity theorem.
