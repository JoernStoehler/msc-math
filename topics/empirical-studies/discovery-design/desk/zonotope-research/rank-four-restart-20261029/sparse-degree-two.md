# Rank-four inequality with a vertex of degree at most two

Draft, 28 October 2026, 23:26 UTC. This extends the six-generator proof mechanism to an arbitrary number of generators whenever the pairing graph has a vertex of degree at most two. It does not establish the unrestricted conjecture. The one cited input is the established three-dimensional zonoid projection inequality (source trace below).

Use the coordinate exterior l1 norm, with increasing-index wedge coordinates. For a skew matrix W, write Phi(W)=sum_{i<j}|W_ij| and Psi(W)=sum_{|I|=4}|Pf W_I|.

**Proposition.** Let W be any finite real skew matrix with rank at most four. If some row has exactly one or two nonzero entries, then 4 Psi(W)<=Phi(W)^2. An isolated zero row alone supplies no new case: it can be discarded, leaving the same inequality on the remaining matrix.

## Three-dimensional input

For arbitrary real row vectors g,h,k of the same finite length, let T=||g wedge h wedge k||1, P=||g wedge h||1, B=||h wedge k||1 and H=||h||1. The local projection inequality for three-dimensional zonoids says

    T H <= P B.                                                    (A)

Indeed apply |Z| |P_{span(e2)} Z|_1 <= |P_{span(e1,e2)}Z|_2 |P_{span(e2,e3)}Z|_2 to the zonotope with half-generators (g_i,h_i,k_i); the four factors are 8T, 2H, 4P and 4B. Degenerate cases follow directly or by continuity.

Primary source: Fradelizi–Madiman–Meyer–Zvavitch, *On the volume of the Minkowski sum of zonoids*, Journal of Functional Analysis 286 (2024), 110247, Theorem 5.1, https://arxiv.org/html/2206.02123. Its theorem allows arbitrary unit directions with an additional angle factor; orthogonal coordinate directions give (A). The 2026 paper arXiv:2608.12681 recalls it as Conjecture A(iii), true in dimension three, but cites a secondary reference. Do not confuse this result with that paper's stronger projection conjecture B, false already in dimension three. See `priorart/dimension-three-source.md` for the citation trace.

## Auxiliary consequence of the planar strip lemma

Let alpha,beta>=0, alpha+beta=1, r,s arbitrary row vectors, g=r+s, h=alpha s-beta r, R=||r||1+||s||1 and H=||h||1. Let B be a decomposable bivector satisfying h wedge B=0. Then

    ||g wedge B||1 <= ||B||1 (R-H).                                (B)

If h!=0, write B=h wedge k (including B=0). The planar strip lemma in `six-vertex/theorem.md` gives P=||g wedge h||1=||r wedge s||1<=H(R-H); (A) therefore implies (B). If h=0, then r=alpha g,s=beta g, hence R=||g||1; the ordinary wedge triangle inequality gives (B).

## Degree two

Reorder so the final row pairs only with indices 1,2. Flip the first two coordinates to make W_1n=t alpha, W_2n=t beta, where t>0, alpha,beta>0 and alpha+beta=1. Let B be the skew block on indices 3,...,n-1, let r=(W_1j), s=(W_2j) on this index set, and c=W_12. Define g,h,R,H as above, and Bm=||B||1.

The rank assumption gives B wedge B=0 and h wedge B=0. For the first, expand the six-Pfaffian on indices {1,n} together with any four indices of B: it is, up to sign, t alpha times the corresponding four-Pfaffian of B. For the second, expand on {1,2,n} and any three indices of B: it is, up to sign, t times the corresponding coordinate of h wedge B. Thus B is decomposable and the auxiliary bound (B) applies.

The old four-Pfaffians have total

    C=||g wedge B||1+||c B-r wedge s||1.

Indeed the all-B terms vanish. The terms with exactly one of 1,2 have total ||r wedge B||1+||s wedge B||1=||g wedge B||1, since r=alpha g-h, s=beta g+h and h wedge B=0. The terms with both 1,2 are cB-r wedge s. The new four-Pfaffians, those using n, have total t(Bm+H).

Since r wedge s=g wedge h, the planar lemma and (B) give

    C <= Bm(R-H)+|c|Bm+H(R-H)
      <= (Bm+H)(|c|+R-H).

Put U=Bm+H and V=|c|+R-H; both are nonnegative. Then

    Phi=t+U+V,
    Psi<=tU+UV=U(t+V),
    Phi^2-4Psi >= (t+V-U)^2 >=0.

## Degree one

Suppose the final row has its sole nonzero entry a at index 1. The complementary block B on indices 2,...,n-1 has rank at most two by the same six-Pfaffian expansion. If r is the old first row on that block, then

    Psi=||r wedge B||1+|a| ||B||1
        <= (||r||1+|a|)||B||1,
    Phi=||r||1+|a|+||B||1.

AM–GM proves the inequality. This case needs no three-dimensional projection theorem.

## What this accomplishes

The result covers unlimited generator counts with a nonisolated pairing-graph vertex of degree <=2. It combines a two-dimensional strip estimate with the established three-dimensional projection inequality. The six-generator theorem remains completely elementary: in that case (B) has a direct l1-section proof and needs no literature input.

General ℓ1 extreme-direction reduction only gives degree <=n-4 for an n-generator target, so this proposition alone does not prove n>=7 or the full conjecture.
