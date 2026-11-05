# Uniform vertex cuts cannot expose the combined ceiling at exact HKO

17 October 2026. A distinct width calculation settles the role of the exact-HKO facet counterfamily. Simultaneously truncating all five vertices of both HKO factors gives a strict two-bounce ratio ceiling below HKO for every intermediate truncation depth. Thus a failure of the standalone star ceiling on that family is protected by the minimum with width. This does not prove the proposed combined ceiling for arbitrary factors; that implication remains open.

## Exact family and width formula

Let $n_i=(\cos(2\pi i/5),\sin(2\pi i/5))$, $h=\cos(\pi/5)$, $s=\sin(\pi/5)$, $L=1+h$, and $m=\sin(2\pi/5)$. Choose the regular inradius-one HKO factors

$$P_0=\{x:n_i\cdot x\le1\},\qquad Q_0=JP_0,$$

where $J$ is a positive quarter-turn. Define the simultaneous midpoint cuts

$$P_\delta=\left\{x:n_i\cdot x\le1,\quad
 \frac{n_i+n_{i+1}}{2}\cdot x\le1-\delta\quad (i\bmod5)\right\},
 \qquad Q_\delta=JP_\delta.$$

For $0<\delta<s^2$, all ten facets of each factor are genuine. At a cut vertex, each removed edge segment has length $\delta/(hs)$; the original edge length is $2s/h$. Thus neighboring cuts are disjoint precisely until $\delta=s^2$. A removed triangle has area $\delta^2/(hs)$, because its two cut edge segments meet at angle $3\pi/5$, whose sine is $2hs$. Since $|P_0|=5s/h$, summing the five disjoint triangles gives

$$\boxed{|P_\delta|=|P_0|(1-\delta^2/s^2).}\tag{1}$$

For each old normal $n_i$, the surviving old edge gives $h_{P_\delta}(n_i)=1$. The opposite cut edge is perpendicular to $-n_i$ and gives

$$h_{P_\delta}(-n_i)=(1-\delta)/h.$$

Hence, writing $D_C=C-C$,

$$d_\delta:=h_{D_{P_\delta}}(n_i)=1+(1-\delta)/h
       =d_0(1-\delta/L).\tag{2}$$

The ten facet normal directions of $P_\delta$ are exactly $\{\pm n_i\}$, because each new cut bisector is opposite an old normal. The planar Minkowski sum $P_\delta+(-P_\delta)$ has no other edge normals: its edges are the angular merge of the summands' edge lists. Each of the ten directions remains a genuine facet direction of the sum. By (2), every one of its ten support heights is $d_\delta$. Therefore $D_{P_\delta}$ is the regular decagon with those normals and that inradius, and its polar has precisely the vertices $\pm n_i/d_\delta$. Similarly, $D_{Q_\delta}^{\circ}$ has precisely the vertices $\pm Jn_j/d_\delta$. Their maximum absolute pairing is $m/d_\delta^2$. The imported two-bounce polar-pairing identity therefore gives

$$\boxed{c_2(P_\delta,Q_\delta)
 =\frac{d_\delta^2}{m}
 =c_2(P_0,Q_0)(1-\delta/L)^2.}\tag{3}$$

The edge-normal description proves equality, rather than assuming that a retained old-normal pair remains optimal. At the uncut HKO pair, $c_2^2/(2|P_0||Q_0|)=\rho_H=(3+\sqrt5)/5$ by the known regular-pentagon formula (or its direct two-bounce equality).

Combining (1)–(3) proves

$$\frac{c_2(P_\delta,Q_\delta)^2}{2|P_\delta||Q_\delta|}
 =\rho_H\left[
 \frac{(1-\delta/L)^2}{1-\delta^2/s^2}\right]^2<\rho_H,
 \qquad0<\delta<s^2.\tag{4}$$

The strict comparison is elementary:

$$1-\delta^2/s^2-(1-\delta/L)^2
 =\frac{2\delta}{L}\left(1-\frac{\delta}{s^2}\right)>0,$$

using $s^2+L^2=2L$. Thus width protects the combined proposal over the entire nondegenerate uniform-cut family, not merely for infinitesimal cuts. In particular, at zero the width ratio has right derivative $-4\rho_H/L<0$, while the product volume changes only to second order.

For the algebraic HKO coordinates in the existing code, let $T$ satisfy $a_i=Tn_i$. Its B normals obey

$$b_i=JTn_i=(\det T)T^{-T}Jn_i,\qquad
 a_i\cdot b_j=(\det T)n_i\cdot Jn_j.$$

Thus the normal transformations are $T$ on A and $(\det T)T^{-T}$ on B: a paired symplectic linear change plus a scalar factor on B. They are not the same map applied independently to both rotated normal lists. Product symplectic invariance and homogeneous scalar factors preserve the ratio; linearity preserves the midpoint-cut constraints. This transfers (4) to the facet worker's exact quadratic-field family.

## Other geometric routes tested on paper

Options were compared by whether they furnish the missing absolute threshold, whether their premises follow from width-hardness, and whether they survive the known facet-birth obstruction. A rubric rewarding HKO agreement alone would accept width without explaining hard-width bodies; a rubric rewarding relative certificates would conceal degeneration of their factor $\eta$.

1. **Canonical mixed area:** the star includes each canonical center. The corresponding edge polygons are $-JP/(2|P|)$ and $Q/(2|Q|)$, so $|U|=1/(4|P|)$ and $|V|=1/(4|Q|)$. The same mixed-area inequality imported in `thesis/chapters/08-data-science.tex:311` gives $M(U,V)\ge\sqrt{|U||V|}=1/(4\sqrt{|P||Q|})$. Equivalently, this follows by expanding the planar Brunn–Minkowski inequality $|U+tV|^{1/2}\ge|U|^{1/2}+t|V|^{1/2}$, canceling the $t^0$ and $t^2$ terms, and dividing by $2t>0$. Since the star response maximum dominates the center-center payoff, $q_\star\ge1/(4\sqrt{|P||Q|})$ and $S_\star\le2$. The required HKO bound would instead need $q_\star\ge1/\sqrt{8\rho_H|P||Q|}$, larger by a factor $\sqrt{2/\rho_H}>1$. The basic inequality is genuinely too weak; replacing it with a sharp lower bound matching the target would be an additional substantive lemma, not a consequence of nonnegativity.
2. **Difference-body containment and area:** width-hardness gives $c_2D_Q^\circ\subseteq D_P$. Taking area supplies only $c_2^2|D_Q^\circ|\le|D_P|$, and the swapped analogue. These constrain centrally symmetric difference bodies; they do not identify a star chord or a maximizing closure support. An implication from these inclusions to a quantitative star payoff is precisely the absent bridge. No equality-case or near-HKO localization follows merely from the strict scalar width threshold.
3. **Relative coverage:** $q_\star\ge\eta q_*$ is valid but supplies neither a universal positive lower bound on $\eta$ (the known facet path disproves one) nor an absolute lower bound on $q_*\sqrt{|P||Q|}$ at the desired constant. Inserting global HKO maximality as that latter bound assumes the sought theorem, and even then the factor $\eta<1$ loses the required sharp constant.
4. **Uniform regular facet refinement:** (4) explicitly excludes this exact-HKO family as a combined-surrogate counterexample. The standalone-star obstruction and this protective width inequality are compatible; neither can be promoted to the other claim.

Disposition: retain the exact width protection lemma as the branching result. Park the proposed combined width/star inequality absent a new bridge from width-hardness to an absolute star payoff. Do not add cohorts, rays, centers, pruning variants or a full capacity oracle on the basis of this lane.

## Verification and source limits

`width-check.py` checks the displayed algebra in $\mathbb Q(\sqrt5)$ with exact rational coefficients, including the sharp HKO normalization, area coefficient, and strict-inequality factorization. It uses no capacity solver or geometry cohort. The universal geometry is proved above; a symbolic PASS verifies the algebra, not every geometric implication on its own.

Inspected local sources: repository README, research guide, AGENTS commentary and collaboration rationale; ceiling-crux REPORT; chord-star REPORT and facet/pruning amendments; two-bounce proof packet; fixed-closure REPORT; full-partner-containment REPORT; two-shadow mixed-area-order REPORT; focused construction helpers. This is bounded source inspection with local shell access, not a literature or exhaustive repository search. No network, installation, cloud, new population, full capacity oracle, PM change, repository edit or commit occurred. Root owns independent review, integration, retention and commits.

[review: /root/message_reviewer]
