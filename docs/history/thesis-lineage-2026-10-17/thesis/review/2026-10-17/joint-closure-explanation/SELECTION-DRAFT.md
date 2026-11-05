# Optional lower-cost Chapter 8 passage: selecting factor weights together

A useful support-selection rule can use more geometry than a scalar summary of either factor. In the product Haim--Kislev objective, the normalized closures $\alpha\in C_q$ and $\gamma\in C_p$ contribute through products $\alpha_i\gamma_j$ and signed pairings of the dual vertices. The signs depend on the facet order. Thus even balanced weights that work well with one partner need not work well with another. Selecting the two factors together is a different task from minimizing ridge area.

One way to retain this joint information is to replace the product matrix $W=\alpha\gamma^{\mathsf T}$ by a nonnegative matrix of total mass one. Require every row to close in the second factor's dual vertices and every column to close in the first factor's dual vertices. Such a matrix allows the choice in one factor to depend on the other. It need not describe one independent pair of factor closures, so its objective cannot simply be treated as an attainable capacity witness.

Its conditional distributions nevertheless suggest actual witnesses. Normalize each nonzero row to obtain a closure of the second factor, then choose the first factor's best response for the same facet order; also exchange the factors. Each resulting pair satisfies the original closure equations. A positive HK objective $q$ therefore gives the feasible capacity upper bound $1/(2q)$ directly.

On a frozen retrospective panel of 26 previously selected upper-tail products and nine bulk controls, these extracted bounds agreed with the stored numerical capacities on all 35 bodies. One bound improved the strongest previous feasible bound by about $1.50\%$. The HKO control also recovered its known capacity. These observations support joint signed geometry as a useful way to select supports; numerical agreement on exposed data establishes neither optimality for every tested body nor future predictive accuracy. The construction remains an optimization procedure, rather than a simple invariant ordering the upper tail or a proof of global HKO maximality.

[review: /root/gap_crux]
