# Conditional complete finite proof of the four-simplex bound

Independent lane, 17 October 2026. Geometry and HK normalization were checked against the local source `topics/capacity-and-qp/README.md` and `formal/capacity-derivatives.tex`. No literature novelty assessment was performed.

## Geometry reduction

Let the translated full dimensional four-simplex have facets `a_i · x ≤ 1`, i=0,...,4. There is one strictly positive closure vector beta, normalized by sum beta_i=1 and sum beta_i a_i=0. Define g_i=beta_i a_i. Its affine barycentric functions are

`lambda_i(x)=beta_i(1−a_i·x)`.

They sum to one; at vertex opposite facet i only lambda_i is nonzero, and therefore it equals one. Hence gradients lambda_i=−g_i. Taking any four barycentric coordinates gives

`|det(g_0,g_1,g_2,g_3)|=1/(24 volume(K))`.

With B_ij=omega(g_i,g_j), B is skew and every row sums to zero. The Pfaffian determinant identity, with |Pf J|=1, gives `|Pf B[:4,:4]|=1/(24 volume(K))`.

The local HK convention is Q=sum_{i<j} B_{sigma_i,sigma_j}, maximized over all orders, and c=1/(2Q). Closure makes cyclic shifts leave Q unchanged, so fixing sigma_0=0 leaves exactly 24 orders. Therefore sys=c²/(2V)=3|Pf|/Q².

## Exact polytope lemma

The space of skew row-zero matrices on five vertices has six free coordinates x=(B01,B02,B03,B12,B13,B23), with B_i4=−sum_{j<4} B_ij. Let Q(x) be the maximum of the 24 order sums. Then

`{Q≤1} = convex hull { ± directed triangle circulations on five vertices }`.

This lemma has an exact finite verification in `exact_polytope.py`, independent of numerical hull software. It generates all 24 integer inequality rows and exhausts every six-element subset (134596 subsets). Each nonsingular subsystem is solved with integer Bareiss elimination and Fraction back substitution, then checked against every inequality. Exactly 31616 bases are nonsingular. The complete distinct feasible vertex set is exactly the 20 triangle circulations. All resulting coordinates have an asserted Fraction type. Rows occur in opposite pairs and include a full-rank basis, so the intersection is bounded and full dimensional; every vertex has six independent tight inequalities. Thus this enumeration proves completeness, rather than merely checks the candidate vertices.

The version using explicit Fraction numerators completed in 9.16 seconds and produced `exact_polytope.json`.

## Quadratic bound

Write p(x)=x0*x5−x1*x4+x2*x3. Every triangle vertex v satisfies p(v)=0. Exact checks show p(v+w) belongs to {−1,0,1}, and each graph with an edge when p(v+w)=+1 or −1 is triangle free. These are finite identities on the 20 explicit integer vertices.

For x=sum_i lambda_i v_i with lambda_i≥0, sum lambda_i=1, polarization yields

`p(x)=sum_{i<j} lambda_i lambda_j p(v_i+v_j)`.

Hence p(x)≤sum_{ij in E+}lambda_i lambda_j. A triangle-free graph has the latter sum at most 1/4. For completeness, maximize that sum over the weight simplex and choose a maximizing vector with minimum support. If positive-weight vertices i,j are nonadjacent, moving their combined weight to i or to j changes the objective affinely, so one choice does not decrease it and reduces support. Thus the minimal support is a clique, of size at most two. Its objective is at most t(1−t)≤1/4. Apply the same argument to −p and E−.

Therefore |p(x)|≤1/4 on Q≤1. Homogeneity gives `Q²≥4|Pf|`, and consequently every nondegenerate four-simplex satisfies `sys≤3/4` under the stated HK convention.

## Attainment

The ordinary simplex `x_i≥0`, `sum x_i≤1` in coordinates `(q1,q2,p1,p2)` has volume 1/24. Translating its barycenter to zero gives g_i=−e_i for i<4 and g_4=(1,1,1,1). Consequently B02=B13=1, Pf=−1 and max order sum Q=2. Thus c=1/4 and sys=3/4.

An abstract positive-Pf witness is one-half the sum of the circulations (0→1→2→0) and (0→3→4→0): p=1/4 and Q=1. This supplies geometric insight as two balanced triangle cycles, but it is not a classification of all equality cases.

## Additional observation (not required by proof)

The positive cross-Pf graph is cubic on 20 vertices with 30 edges and distance shells 1,3,6,6,3,1 from one vertex. It resembles a dodecahedral graph; graph isomorphism was not certified, so the proof uses only the verified triangle-free property.

## Analytic replacement for the graph checks

The signed graph facts can also be proved without enumeration. If two triangle circuits have at least two common vertices, their union has at most four vertices. A row-zero skew matrix supported on four vertices has rank at most two (the restricted four-by-four skew matrix has a kernel and therefore even rank at most two). Thus p(v+w)=0. If they meet in exactly one vertex, all pairs are related by permutations and sign flips to (012)+(034), where p=1; relabeling changes the absolute Pfaffian by no factor because the induced permutation on the four-dimensional quotient by constants has determinant ±1. Consequently p(v+w)∈{−1,+1} in that case. Three three-element subsets of a five-element set cannot have every pairwise intersection of size one: inclusion–exclusion would give union size 9−3+t=6+t>5. Both signed graphs are therefore triangle free. The exact script independently checks these identities, but the polytope vertex enumeration is the only computational lemma needed in the proof.

## Signed symplectic ridge magnitude

For any pair of facets i,j, relabel another vertex as v0, translate v0=0, and let T have the remaining four vertex vectors as columns. Then facet barycentric gradients for i,j are T^(−transpose)e_i and T^(−transpose)e_j. The common ridge has the two complementary edge vectors Te_k,Te_l, and symplectic area |omega(Te_k,Te_l)|/2. Write W=T^transpose J T. The inverse of W is −T^(−1) J T^(−transpose). The four-by-four inverse-Pfaffian identity gives

`|W_kl|=|det T| |(T^(−1) J T^(−transpose))_ij|`.

As |det T|=24V, the symplectic ridge magnitude is exactly `12V |B_ij|`. This establishes the magnitude identity; any signed ridge notation A_ij=12V B_ij must explicitly choose the matching coherent orientation, rather than silently use arbitrary stored triangle orders.

## Sharp chirality and ridge envelope

Let S_B=sum_{i<j}|B_ij| and chi=Q/S_B. Convexity of S_B and the triangle-hull lemma give S_B≤3Q, hence chi≥1/3.

For the opposite inequality, orient each nonzero skew edge by its positive B value. Zero row sums make the resulting weighted directed graph a circulation. Successively remove a directed simple cycle with its minimum positive edge weight; this terminates and decomposes the circulation into cycles of lengths ell∈{3,4,5}. Every total order has at least one backward edge on such a cycle, so its cycle score is at most ell−2≤(3/5)ell. Summing with the cycle weights and maximizing over orders gives Q≤(3/5)S_B. Thus

`1/3 ≤ chi ≤ 3/5`.

Define S_phys=sum ridges |integral omega|=12V S_B, M=12V Q, and R=S_phys/sqrt(V). Then c=6V/M and

`sys=18/(chi² R²)` and `50 ≤ sys R² ≤ 162`.

Both bounds are attained by nondegenerate simplices. The ordinary simplex has x=(0,1,0,0,1,0), Pf=−1,Q=2,S_B=6,chi=1/3,sys=3/4. A directed five-cycle 0→1→2→3→4→0 has x=(1,0,0,1,0,1), Pf=1,Q=3,S_B=5,chi=3/5,sys=1/3. Nonzero Pf implies the corresponding alternating form on the four-dimensional quotient is nondegenerate, so a real g realization with sum g=0 exists; taking beta_i=1/5 and a_i=5g_i gives a bounded full-dimensional simplex containing zero. Thus these are actual geometric endpoints, rather than merely formal circulation examples.

## Equal unsigned ridge sum does not determine the ratio

Take g0=(1,0,0,0), g1=(0,0,−2,0), g2=(−1,1,−2,0), g3=(t,0,−2,−4), and g4=−sum(g0,...,g3), with t=0 or t=1/2. Set a_i=5g_i. In both cases positive closure weights beta_i=1/5 establish a bounded simplex with the interior origin. Their free B coordinates are respectively (−2,−2,−2,−2,0,−2) and (−2,−2,−2,−2,1,−1). Both have |Pf|=8 and S_B=22, hence V=1/192 and R²=6 S_B²/|Pf|=363. The 24-order maxima Q are10 and8. Capacities are1/20 and1/16, with ratios6/25 and3/8. `ridge_controls.py` verifies these values independently by exact vertex reconstruction, determinant volume, direct triangle ridge areas and all cyclic orders. Therefore equal R, even together with equal volume, does not determine capacity or ratio on simplices.

[review: /root/simplex_proof]
