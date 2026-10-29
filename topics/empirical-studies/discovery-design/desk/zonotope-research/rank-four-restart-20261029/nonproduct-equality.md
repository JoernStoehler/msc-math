# An eight-generator nonproduct equality family

Retained 29 October 2026 from the 28 October research draft. The self-dual family below has the independent component audit in `audit/nonproduct-equality-audit.md`. The final broader paired-inverse section was added later and is not covered by that audit. This is an equality construction for the proposed inequality, not a proof in general.

Let x,y,z be real, p=x^2+y^2+z^2>0 and s=|x|+|y|+|z|. Put

    A = [[0,x,y,z],[-x,0,z,-y],[-y,-z,0,x],[-z,y,-x,0]].

Then A^2=-p I, Pf(A)=p and upper-triangular l1 mass(A)=2s. For t>=0 form

    W(t) = [[A,t I],[-t I,(t^2/p)A]].

This skew 8x8 matrix has rank four: its Schur complement relative to invertible A is (t^2/p)A+t^2 A^{-1}=0. Denote Phi=sum_{i<j}|W_ij| and Psi=sum_{|I|4}|Pf W_I|. Directly,

    Phi=2s(1+t^2/p)+4t,
    Psi=(p+t^2+2ts)^2/p.                                      (1)

Here is a complete count for Psi. Partition a four-set according to its number of indices in the first/second four-block.

- Types 4+0 and 0+4 contribute p+t^4/p.
- Type 3+1 contributes 4ts: each index in the second block pairs only with its matching first-block index, and the other two first-block indices contribute one A entry. For each matching index the complementary three indices have edge mass s. Type 1+3 contributes 4t^3s/p.
- Type 2+2 has 36 possibilities. If the first-block pair differs from the second-block pair, the absolute Pfaffian is |A_ij| (t^2/p)|A_kl|. If the pairs coincide, it is t^2(1-A_ij^2/p), because 0<=A_ij^2<=p. Thus this class contributes

      (2s)(2st^2/p)+6t^2-(2t^2/p) sum_{i<j} A_ij^2
      =4s^2t^2/p+2t^2,

  since sum_{i<j}A_ij^2=2p.

Adding these terms gives (1).

Write u=sqrt(p). Then

    Phi/2 - sqrt(Psi)
      = [s(p+t^2)+2tp-u(p+t^2+2ts)]/p
      = (s-u)(t-u)^2/p >=0.                                  (2)

Therefore the conjectured inequality holds on this entire one-parameter family, and equality holds whenever t=sqrt(p), or whenever at most one of x,y,z is nonzero (equivalently s=sqrt(p)). In particular all matrices

    [[J,I],[-I,J]],  J=A/sqrt(p),

are equality cases.

## Geometric interpretation

J is an orthogonal complex structure and Pf(J)=1. Endow R4 with the symplectic form omega_J(v,w)=v^T Jw, and let C=[-1,1]^4. The eight half-generators e_1,...,e_4,-Je_1,...,-Je_4 realize the matrix [[J,I],[-I,J]]. Hence

    Z=C+JC,
    Phi(Z)=4(1+s/sqrt(p)),
    vol_4(Z)=64(1+s/sqrt(p))^2=4Phi(Z)^2.

An orthogonal coordinate change identifies omega_J with the standard symplectic form, so these are ordinary standard-symplectic equality examples after that change.

When xyz!=0, every triple among the eight generating directions is linearly independent. For triples involving two coordinate vectors and one J-coordinate vector, this follows because each column of J has three nonzero coordinates. Applying invertible J gives the same assertion for triples involving one coordinate vector and two J-coordinate vectors. The triples entirely in either basis are independent.

Consequently these zonotopes are not linearly isomorphic to Cartesian products of two planar bodies: a product zonotope's generating directions lie in two complementary two-dimensional subspaces, while here each such subspace can contain at most two of the eight distinct generating directions. The unique generating-direction representation of a zonotope can also be read directly from its edge directions. Thus the equality set is materially richer than the previously proved planar-product families.

For x=y=z=1, p=3 and t=sqrt(3), the support graph is two K4 cliques joined by a perfect matching, equivalently the complement of the cube graph. It is connected and nonbipartite; all vertices have degree four. This is exactly the first degree pattern left by the extremal sparsification bound at n=8.

## Limits and next discriminator

No literature-wide novelty claim is made. The eight-generator equality family supplies a sharp new test for any proposed global proof or equality classification. The theorem for at most seven generators does not cover it. A local perturbation within or transverse to its zero-pairing chamber could reveal either a stable new extremal mechanism or a counterexample; no such conclusion is claimed here.

## Broader paired-inverse equality chamber

The same count gives a larger exact family. Let A be any 4x4 skew matrix with all six entries nonzero and with its three Pfaffian matching terms

    A12 A34,  -A13 A24,  A14 A23

positive. Put p=Pf(A)>0, t=sqrt(p), and B=-t^2 A^{-1}=*A (the Hodge star in the fixed orientation). Then the rank-four matrix

    W=[[A,tI],[-tI,B]]

satisfies Phi=2L+4t and Psi=(L+2t)^2, where L=||A||1 counts upper entries. To verify, note ||B||1=L. The pure-block Pfaffians contribute2p; the 3+1 and1+3 types together contribute4tL. Each diagonal pair product AijBij is one of the positive matching terms (each occurs twice), lies in[0,p], and their sum is2p. Thus the2+2 contribution is L^2+6p-2(2p)=L^2+2p. Summing proves equality. All signs and t factors are accounted for by the same expansion above.

The orthogonal-complex-structure construction is the subfamily *A=A. The broader family is also nonproduct: with all entries of A and A^{-1} nonzero off the diagonal, the generators consisting of one basis and its scaled symplectic dual again have every triple independent. This paragraph has direct algebraic checking but is not yet part of the separate audit file for the earlier selfdual family.
