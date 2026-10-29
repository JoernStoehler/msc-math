# Source pinpoints and coverage — 29 October 2026

## External theorem used in the proof

Matthieu Fradelizi, Mokshay Madiman, Mathieu Meyer and Artem Zvavitch,
*On the volume of the Minkowski sum of zonoids*, Journal of Functional Analysis
286 (2024), 110247, [DOI 10.1016/j.jfa.2023.110247](https://doi.org/10.1016/j.jfa.2023.110247),
[primary arXiv text, Theorem 5.1](https://arxiv.org/html/2206.02123#S5).

Theorem 5.1, displayed equations (36)–(39), proves the three-dimensional local
projection inequality. For orthogonal coordinate directions it reads
`vol_3(Z) length(P_middle Z) <= area(P_first,middle Z) area(P_middle,last Z)`.
For half-generators `(g_i,h_i,k_i)`, these factors are respectively
`8||g wedge h wedge k||1`, `2||h||1`, `4||g wedge h||1`, and
`4||h wedge k||1`. Hence the precise input to the sparse-row proof is

`||g wedge h wedge k||1 ||h||1 <= ||g wedge h||1 ||h wedge k||1`.

The primary theorem and normalization were checked independently in the
[sparse-row audit](audit/seven-sparse-audit.md). The
[original source-trace note](priorart/dimension-three-source.md) also records
why a secondary citation in a later 2026 paper should not replace this source.
No claim that the present packet proves this external theorem is intended.

## Background and neighboring results

Breiding–Bürgisser–Lerario–Mathis, *The zonoid algebra, generalized mixed volumes,
and random determinants*, Advances in Mathematics 402 (2022), 108361,
[arXiv:2109.14996](https://arxiv.org/abs/2109.14996): Theorems 4.1 and 4.5
provide the exterior-zonoid framework; Theorem 5.4 concerns expected absolute
determinants. Its mixed J-volume in Section 6 is a different functional from
the fixed absolute symplectic-pairing sum here. The existing
[parent topic account](../README.md) owns the broader background audit.

Fradelizi–Hubard–Manui–Ndiaye–Wang–Zvavitch,
[*Volume and Projection Inequalities I: Zonoids and Courtade's Conjecture*](https://arxiv.org/html/2608.12681v1),
August 2026, and Skorupinski,
[*Zonoid volumes are not log-submodular*](https://arxiv.org/html/2608.07702v1),
August 2026, contain adjacent counterexamples to volume/projection claims.
They neither supply a proof nor a counterexample to this pairing inequality.
Their numerical examples are not dependencies of the retained arguments.

The [Pfaffian-polynomial note](priorart/pfaffian-polynomial.md) gives primary
pinpoints for the AASV fixed-cardinality principal-minor theorem (Corollaries
62–63), Rincón's signed Wick coordinates, and the EFLS transformed U-polynomial
result (Theorem B). The distinction between signed Pfaffians, squared
Pfaffians and absolute Pfaffians prevents the checked results from settling
the target discriminant inequality.

## Novelty and scope limits

The source investigation was bounded and targeted. It did not find the exact
rank-four absolute-Pfaffian inequality or the retained equality construction,
but no comprehensive citation-graph, MathSciNet or zbMATH novelty audit was
performed. Absence from these searches is not a priority claim. No source
checked here establishes a capacity implication for the retained results.
