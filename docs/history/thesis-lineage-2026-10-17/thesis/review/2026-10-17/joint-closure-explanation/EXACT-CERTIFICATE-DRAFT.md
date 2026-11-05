# Optional Chapter 8 passage: compatible factor choices

The product capacity problem reveals a distinction between balancing each factor and choosing factor weights that work together. The normalized closures $\alpha\in C_q$ and $\gamma\in C_p$ from the six-facet reduction describe balanced weights on the two factors' dual vertices $u_i,v_j$. For a fixed facet order, their contribution is bilinear: each pair receives weight $\alpha_i\gamma_j$. The matrix $W=\alpha\gamma^{\mathsf T}$ thus represents independent choices from the two factor distributions. It has rank one.

A linear relaxation replaces this product matrix by any nonnegative matrix of total mass one satisfying
\[
 \sum_jW_{ij}v_j=0\quad\text{for every }i,
 \qquad
 \sum_iW_{ij}u_i=0\quad\text{for every }j.
\]
Every row and column is still balanced, but the choices can now depend on one another. These conditional closure requirements need not admit a common decomposition into independent factor choices. The relaxation can therefore overestimate the true maximum $Q_{\max}$. Combining the six-facet reduction with the two angular orders gives a relaxed maximum $L$ and the capacity lower bound $1/(2L)$. Conversely, normalizing a nonzero row and choosing its best opposite-factor response produces an actual feasible pair with objective $q$, hence
\[
 \frac1{2L}\leq c_{\mathrm{EHZ}}\leq\frac1{2q}.
\]
If the selected coupling attaining $L$ has rank one, this extraction attains $q=L$: the two bounds certify capacity exactly.

In a frozen retrospective panel of 26 upper-tail products and nine bulk controls, this equality was certified on 26 bodies, including 17 upper-tail bodies. The extracted feasible bounds agreed with the previously stored numerical capacities on all 35 bodies. Nine capacities nevertheless remain uncertified by these bounds. One newly extracted witness reduced the strongest previous feasible capacity bound by about $1.50\%$ and met the exact lower bound. These are finite results on previously exposed data, rather than prospective prediction.

HKO shows the limitation precisely. Its relaxed objective exceeds the true maximum by the factor $(1+3/\sqrt5)/2$, even though conditional extraction recovers its exact capacity. Thus joint compatibility supplies both a useful selection mechanism and a concrete obstruction to this relaxation's sharpness. It does not yet supply a simple invariant ordering the upper tail or a bound proving global HKO maximality.

[review: /root/gap_crux]
