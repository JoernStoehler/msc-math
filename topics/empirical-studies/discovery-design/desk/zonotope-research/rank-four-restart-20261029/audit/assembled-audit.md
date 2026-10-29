# Independent whole-text audit: rank-four Pfaffian note

I checked the complete current text of `rank-four-pfaffian.tex`, including the
normalizations and all equality/finiteness claims. I found no mathematical gap
in the stated proofs, conditional on the cited FMMZ Theorem 5.1. The note does
not claim the unrestricted inequality: the established conclusions are the
cases through seven indices, the arbitrary-size degree-one/two support-row
case, and an eight-generator equality family.

## Checked points

- The planar zonogon area is `4||r wedge s||_1`; its rectangle/strip
  containment and the two-corner area subtraction give the displayed square
  identity. Endpoint `alpha beta=0` reduces to the direct determinant bound.
- In the FMMZ normalization, take the two orthogonal projection normals to be
  the first and third coordinate axes. The intersection projection is the
  middle axis. For generators `(g_i,h_i,k_i)`, the factors are respectively
  `8T`, `2H`, `4P`, `4B`, so the external theorem gives exactly `TH<=PB`.
  The primary source is FMMZ, Theorem 5.1, arXiv:2206.02123; I reopened its
  theorem/proof in this audit. The bibliography title, journal, article number,
  DOI, and arXiv identifier match the source metadata.
- In the decomposable-bivector lemma, if `h != 0` and `B` is simple with
  `h wedge B=0`, then `B=h wedge k`; if `h=0`, the relations give
  `r=alpha g`, `s=beta g`, and `R=||g||_1`. Both branches and `B=0` are covered.
- For the sparse-row theorem, the relevant six-Pfaffian equations force the
  complementary block to have rank at most two and the stated `h wedge B=0`
  relation. Blocks too small to have four-Pfaffians already have rank at most
  two by dimension. The old-minor decomposition and the AM-GM finish have the
  stated factors.
- In the five-index proof, the graph on support edges with adjacency for
  disjointness is triangle-free. The minimum-support maximizer argument is
  the weighted Motzkin--Straus reduction to a clique and yields `Phi^2/4`.
- In the six-index proof, expanding the final Pfaffian gives one linear
  constraint on the new column. Vertices of its intersection with the
  `l1` ball have support at most two. The one-coordinate case, the two-coordinate
  Pfaffian identity, and the scalar dual bound check. If a two-coordinate
  extreme direction has `h_i=h_j=0`, it is not an extreme point on that
  two-coordinate face; the singleton cases handle it. The convexity argument
  then transfers the vertex bound to every feasible column.
- In the minimal-support lemma, the rank-four old block has range codimension
  `n-5`; the column-extension rank criterion is correct. The face-dimension
  bound gives support at most `n-4`. If the new-column Pfaffians all vanish,
  the old block has the same positive objective but smaller edge mass, so
  normalizing it contradicts global maximality. The seven-index parity step
  handles degree zero by the six-index result and degree one/two by the
  sparse-row theorem.
- For the eight-generator family, the Schur complement vanishes, the Pfaffian
  class counts and factorization are correct, and equality includes `t=sqrt(p)`.
  With `xyz != 0`, the eight generator lines are distinct and every triple is
  independent. Each generator line is an intrinsic zonotope edge direction;
  hence no union of two complementary planes can contain all eight. This
  proves the asserted nonproduct property.

## Exposition tightening, not proof blockers

The six-generator dual bound could say explicitly that for a two-coordinate
extreme point `H'=|h_i|+|h_j|>0`; if `H'=0`, that face contributes singleton
vertices instead. The nonproduct paragraph could state that each of the eight
listed directions is an edge direction (choose a generic supporting normal
orthogonal to that generator and to no other generator). Both are short
clarifications; I do not see a mathematical counterexample or missing case.
