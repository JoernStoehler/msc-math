# Operation B: reconstruction from reader dependencies

Status: experimental candidate; no new human PASS.

## Reader questions and dependencies

**HKO.** What geometric freedom does the example test? → Add a momentum component to a q-facet normal. How is that direction encoded? → Give coordinates and the verifier index. What does the computed number differentiate? → The feasible-section upper bound U. Why is this direction absent from factor variations? → Those keep q-facet normals in the q-plane.

**DS.** What is the search trying to find? → Ratios above 1. How can measurements guide it? → Look for associations first, then recover their geometric meaning and test possible uses. What was sampled and measured? → Define the product distribution, numerical ratio and invariant ridge descriptor. Does an association help select new examples? → Fresh-candidate selection tests. Does it identify useful changes to an example? → Separate paired height-change and fixed-budget rotation tests. What survives these tests? → Restricted empirical conclusions, leaving conjectures and rigorous explanations as further work.

## HKO prose replacement slots

### Opening: before “Differentiated closure gives”

Starting from the polygon product, vary the first momentum component of the second q-facet normal. In the description \(K(a)=\{x:a_i\cdot x\leq1\}\), this means perturbing \(a_2\) in the \(p_1\)-direction while leaving every other normal fixed. With coordinates ordered as \((q_1,q_2,p_1,p_2)\), take \(h_2=(0,0,1,0)\) and \(h_i=0\) for \(i\ne2\). This is flat coordinate 6 in the verifier’s zero-based storage. Put \(w=c+kr=\beta_{0,1}\).

### Conclusion: after the DU formula

The resulting coefficient of the upper-branch row is approximately \(-0.1801707325\). It is a derivative of \(U\), the smooth feasible-section upper bound for the systolic ratio, and does not by itself give a derivative of capacity. Varying the two polygon factors keeps every q-facet normal in the q-plane. The direction used here instead adds a \(p_1\)-component to one such normal, so factor variations cannot detect this coefficient. The forty-coordinate calculation includes this additional direction.

## DS replacement

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

## Factual-change ledger

- HKO: retained the perturbation, coordinate index, definition of \(w\), and numerical coefficient. Added the supplied normal-versus-factor dependency and the supplied interpretation of \(U\). Middle displayed formulas are deliberately excluded from these replacement slots and must remain unchanged.
- DS: retained the sampling distribution, admissibility conditions, all counts and numerical results, scaling and invariance claims, and numerical-error caveat. No source result was intentionally omitted.
- DS: made the requested exploration route explicit as motivation, with conjecture formulation and rigorous explanation stated as aims, not achieved results. Reordered descriptor interpretation after the reported association and separated selection from interventions.
- DS: restored readable mathematical notation from the extracted PDF, including the absolute value in absolute symplectic area and the square roots in factor normalization. No mathematical claim is intended to change.

## Exact task message

> Bounded writing repair experiment (~3min). Workdir /workspaces/msc-math/.worktrees/review-workflow-design. You own ONLY experiments/writing-quality/runs/20260918-repair/reconstruct.md. Not alone; preserve others. Read hko-baseline.txt ds-baseline.txt criteria.md in same directory. Operation B: reconstruct from mathematical/reader dependencies, not sentence polishing. First record a compact reader-question/dependency map then compose anew around it, preserving source claims/numbers/caveats. For HKO output ONLY replacement opening prose before 'Differentiated closure gives' and replacement final prose after DU formula; formulas unchanged. For DS output full replacement two-page text as clean Markdown with math; structure should enact supplied discovery route without inventing research achievements. Then short factual-change ledger. No other repo exploration or external research. Do not read competing candidates. Save exact prompt as quoted task message at bottom or separate section of owned file. Report completion. No commits; parent commits experimental bundle. User explicitly authorizes this repair experiment; new output has no human PASS.
