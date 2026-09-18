# Mathematical orientation

## Conventions

Work in R⁴ = R²_q ⊕ R²_p with
ω₀((q,p),(q′,p′)) = q·p′ − p·q′. A planar polygon factor is full dimensional
and contains 0 in its interior. Its dual vertices are the covectors u_i such
that its facets have inequalities u_i·q ≤ 1; they are unit outward normals
divided by positive support heights, not necessarily unit vectors.

For Kθ = A × RθB the product covectors are (u_i,0), (0,Rθv_j).
A word σ consists of distinct facet indices in order. With position-indexed
weights β_k ≥ 0, sum β_k = 1 and sum β_k a_{σ_k} = 0, define

    Qσ,β = sum{j<k} β_j β_k ω₀(a_{σ_j},a_{σ_k}),
    Qmax = max{σ,β} Qσ,β,       c_EHZ(K) = 1/(2 Qmax).

All facet permutations with zero weights allowed and all distinct-index
subset words give the same maximum: deleting zero entries preserves Q and
closure, and any subset word can be extended by zero-weight entries.
Earlier entry is the first argument of ω₀. The original Haim–Kislev formula
uses the opposite named-word order; reversal of the whole word and weights
identifies the maxima. Do not silently reverse a fixed witness.

Our four-dimensional systolic ratio is c_EHZ(K)²/(2 vol₄(K)). Thus the proposed
Viterbo bound is sys ≤ 1 and the known HKO example has sys = (3+√5)/5 > 1.
For a Cartesian product, vol₄(A × B) = area(A) area(B).

HKO's symplectic convention is sum dp_i ∧ dq_i = −ω₀. Changing coordinates
(q,p) ↦ (q,−p) converts its pentagon position at relative angle −π/2 to ours
at +π/2. Pentagon symmetry and simultaneous reflection identify both endpoint
capacities used in the supplied proof. This convention detail is explicit in
the project's chapter-v2 exposition; the supplied compact proof states the
already-translated endpoint value.

## What the accepted proof uses

For arbitrary fixed polygon factors, closure still separates, removing Rθ
still leaves a common feasible set, and a fixed candidate still has angular
form A cos θ + B sin θ. Positive sine interpolation on an interval of length
strictly between 0 and π therefore supplies an upper bound on Qmax from its
two endpoint values. Taking reciprocals reverses the capacity inequality.
This observation alone does not give equality or determine the endpoints.

For the regular pentagon the extra ingredients are:

- Equal known endpoint capacities at ±π/10, imported from HKO.
- Rotational, reflection and factor-exchange symmetries reducing all angles
  to that interval.
- The exact identity n₀+n₁ = −2 cos(π/5)n₃, providing a single feasible witness
  that attains the interpolation bound throughout the interval.

Those are the inputs to revisit rather than assuming every polygon enjoys
the same formula. An optimizer can change at an angle even though the
feasible set is fixed.

## Optional product reduction and a possible lens

The optional source proves that some global maximizer has total weight 1/2
on each factor and uses a vertex of each factor's normalized closure polytope
C = {α ≥ 0: sum α_i = 1, sum α_i u_i = 0}. Such a vertex has support ≤ 3.
The proof uses bilinearity after separating the factor masses, not a claim
that every minimizing orbit has six facets. It permits finite enumeration of
closure-vertex pairs and cyclic orders; cyclic invariance follows from closure.

A possible mathematical lens, offered as a deduction to check and develop:
for a fixed pair of polygons the finite enumeration is independent of θ.
Its coefficient pairs (A,B) suggest viewing Qmax(θ) as the support function
of their convex hull on (cos θ,sin θ). Merely packaging the finite maximum this
way is not asserted to be novel. The potential value would be characterizing
that hull geometrically, identifying its relevant vertices without brute
force, obtaining sharp profiles/bounds, or explaining support transitions.
This is optional and should not exclude a better approach.

The raw reduction source contains references to the original thesis convention
subsection and project review history. The conventions above supply what is
needed here; neither the repository nor the historical numerical evidence is
a logical input to the reduction's proof.
