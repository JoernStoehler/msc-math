# Operation A: narrow repair

Experimental candidate; no human PASS judgment has been recorded for this output.

## HKO: replacement opening prose

Vary the first momentum component of the second \(q\)-facet normal. In coordinates \((q_1,q_2,p_1,p_2)\), take \(h_2=(0,0,1,0)\) and \(h_i=0\) for \(i\ne2\). This is flat coordinate 6 in the verifier’s zero-based storage. Put \(w=c+kr=\beta_{0,1}\).

## HKO: replacement final prose after the DU formula

This is an actual coefficient of the upper-branch row for the smooth feasible-section upper bound \(U\), approximately \(-0.1801707325\); it is not an assertion about the derivative of the actual capacity. Varying the two polygon factors keeps each \(q\)-facet normal in the \(q\)-plane. The perturbation above instead adds a \(p_1\) component to one such normal, so the forty-coordinate calculation includes a direction that differentiating only within products would miss.

## DS: full replacement text

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

## Factual-change ledger

- HKO: identified the varied object as the facet normal; made explicit why its added momentum component leaves the product-preserving directions; identified \(U\) as an upper bound without claiming an actual-capacity derivative. The displayed-formula block is excluded from these replacement slots and must remain unchanged.
- DS: replaced the opening with the supplied exploratory motivation, expressed as a route toward conjectures and rigorous explanations rather than a claim to have achieved them. Added three headings and the explicit limit that selection success does not explain the association. Preserved paragraph order after the opening, numerical results, sampling details, and mathematical caveats. No reported experimental result was added or omitted.
- DS: restored mathematical typesetting obscured by text extraction, including the absolute value in absolute symplectic area and the square roots in the factor-normalization map.

## Exact task message

> Bounded writing repair experiment (~3min). Workdir /workspaces/msc-math/.worktrees/review-workflow-design. You own ONLY experiments/writing-quality/runs/20260918-repair/narrow.md. Not alone; preserve others. Read hko-baseline.txt ds-baseline.txt criteria.md in same directory. Operation A: narrow edit, preserve overall source order and wording where possible, repair supplied objections with minimal necessary changes. For HKO output ONLY replacement opening prose before 'Differentiated closure gives' and replacement final prose after DU formula; formulas unchanged. For DS output full replacement two-page text as clean Markdown with math, preserving factual content, using minimal edits/reordering/headings needed. Then short factual-change ledger. No other repo exploration or external research. Do not read competing candidates. Save exact prompt as quoted task message at bottom or separate section of owned file. Report completion. No commits; parent commits experimental bundle. User explicitly authorizes this repair experiment; new output has no human PASS.
