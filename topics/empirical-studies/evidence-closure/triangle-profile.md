# Exact profile of two equilateral triangles

Independent mathematical audit, 2026-09-18. No numerical fit is used in the proof. Conventions match `docs/pentagon-chapter-v2/chapter.tex`, equations `pentagon-feasibility`, `pentagon-q-definition`, and `pentagon-hk`.

Let \(P\) be a centred equilateral triangle of circumradius one, let \(K_\theta=P\times \operatorname{Rot}_\theta P\subset\mathbb R_q^2\times\mathbb R_p^2\), and put \(d=\operatorname{dist}(\theta,(\pi/3)\mathbb Z)\in[0,\pi/6]\). With \(\omega((q,p),(q',p'))=q\cdot p'-p\cdot q'\),

\[
R(K_\theta)=8\sqrt3\cos d,\qquad
c_{\mathrm{EHZ}}(K_\theta)=\frac{3\sqrt3}{4\cos(\pi/6-d)},\qquad
\operatorname{sys}(K_\theta)=\frac12\sec^2(\pi/6-d).
\]

Consequently both \(R\) and sys strictly decrease as (d) increases from zero to \(\pi/6\): the endpoints are \((R,\mathrm{sys})=(8\sqrt3,2/3)\) and \((12,1/2)\). Thus distinct ridge values in this whole rotation family have **the same strict ordering** as their systolic ratios. Unrestricted \(\theta\) is periodic, not globally monotone.

## Capacity proof

Choose the three outward unit normals \(n_i=(\cos(2\pi i/3),\sin(2\pi i/3))\), so that \(P=\{q:n_i\cdot q\le1/2\}\). Write \(m_j=\operatorname{Rot}_\theta n_j\). The normalized dual rows are \(a_{q_i}=(2n_i,0)\) and \(a_{p_j}=(0,2m_j)\). In the manuscript's Haim--Kislev formula the weights satisfy \(\sum\beta=1\) and \(\sum\beta a=0\), and the capacity is \(1/(2Q_{\max})\), where
\[
Q=\sum_{r<s}\beta_r\beta_s\omega(a_{\sigma_r},a_{\sigma_s}).
\]
Because the kernel of the three-normal matrix is spanned by \((1,1,1)\), closure forces a common weight \(x\) on the three q facets and a common weight \(y\) on the three p facets, with \(x+y=1/3\). Facets omitted from a word can be inserted with weight zero. A positive objective requires both \(x,y>0\), hence all six facets occur. For their order,
\[
Q=4xyS,\qquad
S=\sum_{i,j}\epsilon_{ij}\,n_i\cdot m_j,
\]
where \(\epsilon_{ij}=1\) when \(q_i\) precedes \(p_j\), and \(-1\) otherwise. Cyclic rotation of a word preserves \(S\), since \(\sum n_i=\sum m_j=0\).

Classify cyclic words by the number of q runs (equal to the number of p runs):

* One run: the entire q and p sums vanish, so \(S=0\).
* Two runs: combine consecutive vectors of the same kind. Each q block has sum \(u=\pm n_i\), the other has sum \(-u\); similarly the p sums are \(v=\pm m_j\) and \(-v\). The reduced word is \(u,v,-u,-v\), and \(S=2u\cdot v=\pm2\cos(\theta+2\pi k/3)\le2\).
* Three runs: the word alternates, say \(q_a,p_b,q_c,p_d,q_e,p_f\). Summing the q vectors preceding each p vector and using closure gives
  \[
  S=2\bigl(n_a\cdot m_b+(n_a+n_c)\cdot m_d\bigr)
    =2(n_a\cdot m_b-n_e\cdot m_d).
  \]
  Set \(c_k=\cos(\theta+2\pi k/3)\). Hence \(S=2(c_k-c_l)\) for some \(k,l\).

For \(0\le\theta\le\pi/6\), \(c_0\ge c_2\ge c_1\); therefore all words satisfy
\[
S\le2(c_0-c_1)=3\cos\theta+\sqrt3\sin\theta.
\]
The right side is at least 3, so also bounds the two-run case. Equality is attained by the alternating word
\[
q_0,p_0,q_2,p_2,q_1,p_1.
\]
Finally \(xy\le1/36\), with equality at \(x=y=1/6\). Thus
\[
Q_{\max}=\frac{3\cos\theta+\sqrt3\sin\theta}{9}
=\frac{2\sqrt3}{9}\cos(\pi/6-\theta).
\]
The stated capacity follows. Since \(\operatorname{area}\(P\)=3\sqrt3/4\), \(\operatorname{vol}_4(K)=27/16\); the convention \(\mathrm{sys}=c_{\mathrm{EHZ}}^2/(2\operatorname{vol}_4)\) gives the stated systolic ratio.

## Ridge proof and reduction of angle

All triangle edges have length \(\sqrt3\). Their directions differ from their corresponding outward normals by the same quarter-turn. The nine absolute dot products therefore sum to
\[
\sum_{i,j}|a_i\cdot b_j|=9\sum_{k=0}^2|c_k|=18\cos\theta
\]
on the displayed sector: \(c_0\ge0\), \(c_1,c_2\le0\), and \(\sum c_k=0\). Dividing by \(\sqrt{\operatorname{area}\(P\)^2}=3\sqrt3/4\) proves the ridge formula.

Both quantities have period \(2\pi/3\) from the triangle's rotational symmetry and are even in \(\theta\): simultaneous reflection in the two factor planes is symplectic and preserves \(P\). The symplectic factor exchange \((q,p)\mapsto(p,-q)\), followed by the same planar rotation \(-\theta\) in both planes, maps \(K_\theta\) to \(K_{\pi-\theta}=K_{\pi/3-\theta}\). It also preserves the ridge expression. Combining this symmetry with evenness gives period \(\pi/3\), justifying the reduced angle (d).

## Connection to the balancing experiment

For a centred triangle with vertex matrix \(X\in\mathbb R^{2\times3}\), isotropic uniform vertex covariance means \(XX^T=3\alpha I\), and centring gives \(X\mathbf1=0\). Therefore \(X^TX=3\alpha(I-\mathbf1\mathbf1^T/3)\), so all pairwise squared vertex distances equal \(6\alpha\). Hence ideal vertex-covariance balancing makes every triangle equilateral. Independent factor dilations do not affect either \(R\) or sys, so the resulting ideal (3\times3\) products are covered by the family above. Actual rounded outputs are numerically close to this family; the theorem does not assert their binary64 coordinates are exactly equilateral.

## Covariance-balancing audit

For a positive definite planar vertex covariance \(A\), the implemented map \(L=(\det A)^{1/4}A^{-1/2}\) has determinant one and \(LAL^T=\sqrt{\det A}I\). A primal map \(x\mapsto Lx\) transforms dual column normals by \(L^{-T}\); row normals therefore transform as the implemented \(d\mapsto dL^{-1}\). Each ideal factor retains area and combinatorial type. The product covariance becomes \(\Gamma=\operatorname{diag}(aI,bI)\), so \((J\Gamma)^2=-abI\), both Williamson scales equal \(\sqrt{ab}\), and \(\rho=1\). The full block map is generally not symplectic; capacity changes are allowed and are the measured response.

`selection-mechanism/balance.py` reconstructs the vertices of the actual rounded binary64 dual input using exact rational pair intersections and exact half-plane feasibility. It checks factor vertex counts, relative area error below \(10^{-9}\), and \(|\rho-1|<10^{-8}\). These certify the reported numerical geometry tolerances, not literal exact equalities for the rounded files. The 2D helper assumes the inherited valid bounded factor setting; it is not a generic standalone boundedness certificate. The current evaluator separately validates the complete products.
