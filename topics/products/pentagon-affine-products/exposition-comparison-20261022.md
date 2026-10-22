# Candidate exposition: what the 5×7 result adds

22 October 2026. This is a prose comparison for the selected affine-pentagon chapter, not a change to that chapter or a decision to include new material. The first version is a candidate theorem and example for a report or short appendix. The second shows how the existing chapter can close without 5×7.

## Version with 5×7: theorem and example

The pentagon-pair theorem has an unequal-factor counterpart, but its extremal behavior changes. Let
\[
 n_i^{(r)}=(\cos(2\pi i/r),\sin(2\pi i/r)),\qquad
 h_r=\cos(\pi/r),\qquad
 P_r=\{x:n_i^{(r)}\cdot x\le h_r\text{ for }0\le i<r\}\quad(r=5,7).
\]
The polygons have circumradius one. Products below carry the standard Lagrangian symplectic form.

**Theorem (independently affine pentagon–heptagon capacity).** For \(G,H\in GL(2,\mathbb R)\), set \(M=G^{-1}H^{-T}\) and
\[
 m_{57}(M)=\max_{0\le i<5,\ 0\le j<7}
       |(n_i^{(5)})^T M n_j^{(7)}|.
\]
Translations of either factor do not affect capacity, and
\[
 c_{\mathrm{EHZ}}\bigl((a+GP_5)\times_L(b+HP_7)\bigr)
       ={(1+h_5)(1+h_7)\over m_{57}(M)}.                 \tag{1}
\]
Thus thirty-five absolute normal pairings determine capacity even when the factors are independently sheared or reflected.

The imported Haim–Kislev facet-word formula and the sparse closure reduction leave one possible obstruction to (1): a six-block word using a positive closure triple from each factor. The four-block words generate a convex body of \(2\times2\) matrices. An exact rational-interval certificate classifies all ten pentagon and thirty-five heptagon triples, finds five and fourteen positive ones, and proves that each of their \(5\cdot14\cdot12=840\) cyclic six-block matrices lies strictly inside that four-block body. The matrix containment survives independent changes \(G,H\). This explains why the rotation calculation alone could not justify (1): it sees only a two-dimensional projection of the matrix body. The certificate, its checker, and an independent mathematical review are required proof inputs; (1) does not claim a sharp systolic ratio among all affine 5×7 pairs.

There is already a useful consequence. Normalize \(m_{57}(M)=1\), and put \(\theta=\pi/70\), \(t=\tan\theta\), and \(\beta=4\pi/7\). The best rotated pair is represented by
\[
 M_0={R_\theta\over\cos\theta}=
 \begin{pmatrix}1&-t\\t&1\end{pmatrix}.
\]
Now set
\[
 N=\begin{pmatrix}0&0\\\sin\beta&-\cos\beta\end{pmatrix}.
\]
The two normal pairings active at \(M_0\) stay fixed along \(M_0+\varepsilon N\); every other pairing has strict slack. Hence \(m_{57}(M_0+\varepsilon N)=1\) for all sufficiently small positive \(\varepsilon\). Yet
\[
 \det(M_0+\varepsilon N)
 =\sec^2\theta+\varepsilon(-\cos\beta+t\sin\beta)
 >\sec^2\theta.
\]
Every nonsingular \(M\) is realized by taking \(G=I\) and \(H=M^{-T}\). By (1) and area transformation, the systolic ratio at fixed \(m_{57}=1\) is a positive constant times \(|\det M|\). Therefore an independently affine pentagon–heptagon pair beats every rotated 5×7 pair. This does not say it beats the HKO benchmark, and the sharp affine 5×7 maximum and its equality cases remain unknown.

## Version omitting 5×7

The affine-pentagon theorem already shows that a capacity problem with many feasible facet words can collapse to twenty-five normal pairings. Its proof uses a full matrix containment, which is preserved by independent linear changes of the two pentagons. That is the reason the rotation formula extends to shears and reflections and why the HKO ratio remains sharp in this affine family. For other factor pairs, a rotation profile tests only a projection of the matrix containment condition. The present theorem makes no assertion about independent affine changes of those pairs.

## Editorial comparison

The longer version gives a concrete surprise that the existing 5×5 chapter cannot show: optimizing rotations and optimizing independent affine shapes are different tasks for 5×7. Its capacity formula is exact, but a reader needs the sparse-word premise and a trustworthy account of the 840-case certificate to assess it. A one-page insert can state that dependence; it cannot replace the certificate or its review. The shorter version keeps the selected chapter focused on its already sharp 5×5 theorem and avoids a new proof dependency. Given the recorded writeup-first priority and the absence of a selected need for another finite pair, I recommend the shorter version for the main chapter at present. Retain the longer version as a reviewable candidate for an appendix or research report if the affine-versus-rotation contrast earns that additional reader burden. This is an editorial recommendation, not a claim of Jörn's acceptance or a novelty judgment.

Sources: [selected affine chapter](../../../thesis/chapters/10-affine-pentagons.tex), [5×7 certificate review](tensor-certificate-review-20261022.md), and [affine ascent proof](next-obstructions-20261022.md).
