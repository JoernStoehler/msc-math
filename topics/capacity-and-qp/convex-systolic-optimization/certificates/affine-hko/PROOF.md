# Global affine optimality of the HKO product: theorem and certificate

28 October 2026. The complete argument received independent mathematical
review in [`../../reviews/affine-hko-mathematics.md`](../../reviews/affine-hko-mathematics.md)
and independent exact replay in
[`../../reviews/affine-hko-exact-replay.md`](../../reviews/affine-hko-exact-replay.md).
The finite predicates have an exact Sage verifier. Those scoped checks do not
approve later assembled edits or establish bibliographic priority.

## Statement and conventions

Let P be the regular planar pentagon of circumradius one used below and let
H=P×R(-π/2)P⊂R²_q×R²_p. For every invertible real 4×4 matrix A,

ρ(AH)=c_EHZ(AH)²/(2 Vol₄(AH)) ≤ ρ(H)=(3+√5)/5.

Equality holds precisely when AH is a positive dilation of a linear symplectic image of H. Translations can be added on either side without changing the result. This enlarges independent affine changes of the two factors to the full GL(4) orbit; it makes no claim about arbitrary nearby convex bodies.

Use J=[[0,I],[-I,0]] so ω(a,b)=aᵀJb, and write a skew form Ω as
x=(Ω₀₁,Ω₀₂,Ω₀₃,Ω₁₂,Ω₁₃,Ω₂₃). Define

P(x)=−Pf(Ω)=x₀₂x₁₃−x₀₁x₂₃−x₀₃x₁₂.

Thus J has coordinates (0,1,0,0,1,0) and P(J)=1.

## 1. Affine action body (general mechanism)

For a polytope K={x:a_i·x≤1}, a distinct-facet word σ and weights β≥0 satisfying Σβ_i=1 and Σβ_i a_{σ_i}=0 give the HK value

Q_{σ,β}(Ω)=Σ_{i<j}β_iβ_j a_{σ_i}ᵀΩ a_{σ_j}.

Let W be the convex hull of all coefficient bivectors
b_{σ,β}=Σ_{i<j}β_iβ_j(a_{σ_i}∧a_{σ_j}). Reversing the word reverses its bivector, so W is centrally symmetric. It is compact, since there are finitely many words and each feasible weight set is compact. For transformed normals A^{-T}a_i the same β remain feasible, and

Qmax(AK)=h_W(A^{-1}JA^{-T}),   c_EHZ(AK)=1/(2h_W(A^{-1}JA^{-T})).

The source HK formula uses the opposite pair ordering. Reversing each word and its weight list identifies that maximum with our earlier-first convention. Subset words are equivalent to full permutations because zero weights can be inserted, or deleted, without changing feasibility or Q. No optimizing-branch continuation is involved.

Moreover W has nonempty interior for every full-dimensional bounded K. Choose a strictly positive feasible β on all facets (the normals positively span the ambient space). If h_W(Ω)=0, every ordering has zero objective. Swapping adjacent facets i,j changes the objective by 2β_iβ_j a_iᵀΩa_j. Hence all pairings vanish. Since the normals span, Ω=0. Thus h_W is a norm. This observation also makes affine capacity optimization a compact Pfaffian optimization on W°.

For det A>0,

P(A^{-1}JA^{-T})=1/det A,
ρ(AH)=P(Ω)/(8 Vol₄(H) h_W(Ω)²).

## 2. Exact finite rows

Put t=tan(π/5), the root in (0,1) of t⁴−10t²+5=0, and s=√5=(5−t²)/2. The ten normals are precisely the explicit vectors in verify.py. Their factor pentagons have circumradius one. The HKO benchmark is

c₀=5t−t³/2,  q₀=1/(2c₀),  V₀=25(5+s)/32,
1/(8V₀q₀²)=(3+s)/5.

The certificate supplies 18 distinct-facet words and exact feasible β vectors. For every row b in its matrix B, the verifier checks

q₀ b = Σ_{i<j}β_iβ_j(a_{σ_i}∧a_{σ_j}).

Therefore b·x ≤ h_W(x)/q₀ for every x. Define C={x:Bx≤1}. We do not need C to equal the true action polar, or the words to be exhaustive, or the weights to remain optimizing under deformation.

The witnesses were discovered from the repository's fixed-facet HKO feasible-section rows and their polygon symmetries, together with pure-factor five-facet loops. The standalone certificate verifies them directly and does not depend on the old derivative theorem or generator.

## 3. Selfdual projection and an explicit three-dimensional outer polytope

Set

U(x)=((x₀₂+x₁₃)/2,(x₀₃−x₁₂)/2,(x₀₁−x₂₃)/2),
V(x)=((x₀₂−x₁₃)/2,(x₀₃+x₁₂)/2,(x₀₁+x₂₃)/2).

A direct expansion gives P(x)=|U(x)|²−|V(x)|².

Let D be the regular 20-gon in R² with vertices at angles kπ/10, circumradius one. Let E be the regular decagon consisting of its even-indexed vertices. Put

a=(5−√5)/4,  r=143/200,
T=conv((E×{0}) ∪ (rD×{a}) ∪ (rD×{−a})).

Every vertex of E×{0} has norm one. Every vertex in the two other levels has norm squared r²+a²<1. Consequently T lies in the unit ball, and its intersection with the unit sphere consists of exactly the ten vertices of E×{0}. The last conclusion uses strict convexity of the Euclidean unit ball, not just a vertex count.

Here is a complete halfspace description of T. In addition to |z|≤a, use every outward edge-normal n of E and every outward edge-normal n of D. For d₀=h_E(n), d₁=r h_D(n), impose

n·u+(d₀−d₁)|z|/a≤d₀.

These are 62 inequalities (two horizontal ones and two for each of 30 planar normal directions). To see completeness, first rD⊂E. The slice at |z|=aτ is (1−τ)E+τrD. Its edge normals are the union of the two polygons' edge normals, and its support in each such direction is (1−τ)d₀+τd₁. This proves both the slice description and the stated halfspace description. In verify.py the edge normals of E are the odd vertices of D; the edge normals of D are sums of adjacent vertices of D. Normalization of those vectors is unnecessary.

Write these halfspaces as h₀+h·y≥0, where h₀>0. For four representative halfspaces (indices 0,6,22,23 in the explicit list), the certificate gives nonnegative coefficients λ_j summing to one and verifies the exact identity

Σ_j λ_j B_j = −hᵀL/h₀,   Lx=U(x).

For the full action polar C_W={x:h_W(x)≤q₀}, all B_jx≤1. Thus the four representative inequalities hold on U(C_W). The group generated on U-space by rotation through 2π/5 in its first two coordinates, reflection (u,v,z)↦(u,−v,z), and total inversion preserves U(C_W). Indeed, these arise respectively from congruence by a 2π/5 rotation of the q pentagon (with the opposite rotation sign under projection), the negative of congruence by simultaneous q-horizontal/p-vertical reflection, and Ω↦−Ω. The congruences preserve H and therefore h_W; negation preserves h_W by word reversal. The verifier checks the normal permutations and the intertwining of each six-dimensional congruence with U. This group carries the four representative halfspaces to all 62 halfspaces, with orbit sizes 2,20,20,20. Hence U(C_W)⊂T. No numerical hull, nonlinear optimizer, or unverified facet enumeration is used in the final proof. Note that the selected 18-row polyhedron itself need not be group invariant; the argument uses the full action polar's symmetry.

## 4. Capacity bound and equality

For any nonzero skew form x, let q=h_W(x)/q₀>0. By definition x/q belongs to C_W. Hence

P(x)≤|U(x)|²≤q².

Substituting the affine action formula proves ρ(AH)≤ρ(H) for det A>0.

If equality holds, then V(x)=0 and U(x)/q is a vertex of E×{0}. Thus x/q has zero same-factor entries and cross block

[[cos(kπ/5),sin(kπ/5)],[-sin(kπ/5),cos(kπ/5)]],  k=0,…,9.

These ten forms are Ω_T=T^{-1}JT^{-T} for linear automorphisms T of H: rotations by 2π/5 in one factor give five, and the simultaneous factor reflections (q₁,q₂)↦(q₁,−q₂), (p₁,p₂)↦(−p₁,p₂) preserve H and send J to −J, yielding the other five. Consequently Ω_A=qΩ_T. The matrix S=√q A T^{-1} satisfies SJSᵀ=J, so S is symplectic, and AH=q^{-1/2}SH. Conversely these images have the same ratio.

For det A<0, compose on the right with reflection of the q pentagon alone. It is a determinant −1 automorphism of H, so the resulting matrix has positive determinant and the image body is unchanged. This reduces both the inequality and its equality classification to the positive determinant case.

## Reproduction and scope

Run `verify.py` with a Sage Python environment. On this workstation:

`/workspaces/msc-math/.local-environments/miniforge/envs/sage/bin/python verify.py`

The input is certificate.json; the output is verification-summary.json. The verifier reconstructs all row coefficients from exact feasible HK words, builds the explicit 50-vertex/62-halfspace outer polytope, verifies its vertex sphere bounds, and verifies four projection identities and the exact symmetry coverage of all 62 outer halfspaces. The normal-fan halfspace completeness argument, affine reduction, and equality implications remain mathematical proof obligations for an independent reader.

Primary mathematical inputs: Haim–Kislev, On the symplectic size of convex polytopes (HK formula); Haim–Kislev–Ostrover, A counterexample to Viterbo's conjecture, Annals 203 (2026), 603–622 (HKO benchmark). The bounded novelty search found no full-GL(4) HKO optimization theorem; this is not a comprehensive priority claim.
