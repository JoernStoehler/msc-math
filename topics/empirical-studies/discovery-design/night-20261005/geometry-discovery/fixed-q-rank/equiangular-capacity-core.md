# Capacity core and sharp global bound for equiangular pentagon partners

Derived 2026-10-04 23:20 UTC by geometry-discovery lead. This is a new
agent-derived positive relation, independently checked in
[equiangular-global-review.md](equiangular-global-review.md) at23:28 UTC;
no thesis/formal promotion or literature novelty claim. It supersedes a weaker
proposed bound by max(1, regular ratio) and removes any need for vertex
enumeration in this class. The prescribed ten-direction fan from the retained
September archive already settles one special orientation; this argument
covers every orientation by a simple structural mechanism.

## Objects and exact statement

Fix the regular pentagon Q of circumradius one whose outward unit normals are
n_i=(cos(2πi/5),sin(2πi/5)). Let Pθ=Rθ Q, and write

    b=cos(π/5), w=1+b,
    m_i=Rθ n_i,
    K_h={x: m_i·x<=h_i, i=0,…,4}.

Assume K_h is a bounded convex body with interior. Every displayed facet need
not remain genuine: triangles and quadrilaterals obtained by losing edges are
allowed. No normals outside this prescribed five-direction set are allowed.

Define the inner regular-pentagon scale

    ιθ(K)=max{t>=0: some z satisfies z+tPθ⊆K}.

**Capacity-core theorem.**

    c_EHZ(K×_L Q)=c_EHZ(Pθ×_L Q) ιθ(K).                         (1)

Consequently

    sys(K×_L Q)<=sys(Pθ×_L Q),                                  (2)

with equality precisely for translates and positive dilates of Pθ. Thus the
regular partner is the unique global maximizer in the entire fixed regular
five-normal fan, including edge-loss boundaries. This is a theorem about
arbitrary supports in that fan, rather than only independently affine regular
pentagon factors. It is not an arbitrary-pentagon or arbitrary-partner theorem.

The existing independently affine regular-pentagon capacity formula gives

    d=dist(θ,(π/5)Z), 0<=d<=π/10,
    c_EHZ(Pθ×_L Q)=w² sec d,
    sys(Pθ×_L Q)=(5+2sqrt5)/10 · sec²d.                          (3)

The largest value over orientations is the HKO benchmark (3+sqrt5)/5,
attained when d=π/10. Combining (2)–(3) therefore supplies that benchmark
and its equality classification for **all equiangular pentagon partners**, at
all relative orientations. Within this class equality at the HKO benchmark
requires K to be regular and its orientation to be an HKO angle. Formula (3)
is imported, not rediscovered here; its owner is
topics/products/pentagon-affine-products/research.tex and the selected rotation chapter.

## Why all ten fitting constraints collapse

Use the complete ten normal unit-Q-perimeter triangles T_j from
[Balitskiy–Mitrofanov–Polyanskii, Definition3.2/Theorem3.3/§6.1](https://arxiv.org/html/2603.12495v1).
The desk evaluator's [README](../../../../../products/pentagon-arbitrary-partner/README.md)
owns conventions. In particular,

    c(K×_L Q)=min_j r_j(K),
    r_j(K)=min{h·a: a>=0, Σa_i m_i=0, Σa_i h_Tj(m_i)=1}.        (4)

The ten translation classes are invariant under rotation by2π/5. Translate
any T_j to contain zero in its interior if desired; the dual normalization is
unchanged because Σa_i m_i=0, and all support coefficients are then positive.
Thus these dual polytopes are compact and their vertex optima are complete.

Consider the normalized normal-closure polytope

    C={λ>=0: Σλ_i=1, Σλ_i m_i=0}.

Its vertices have at most three positive coordinates. No two pentagon normals
are opposite, and all triples are affine independent, so every vertex has
exactly three positive coordinates. A triple closes positively iff its three
cyclic gaps are (1,2,2) in some order; these are exactly the five cyclic
rotations of {0,1,3}. They form one orbit under the cyclic group of order5.

For that base triple, the exact closure identity is

    n_0+n_1=-2b n_3,
    λ^(0)=(1,1,0,2b,0)/(2w).                                  (5)

All five λ^(l) are its cyclic shifts. Rotating every normal byθ does not change
the closure weights. This proves both enumeration completeness and the
single-orbit property, without numerical hull enumeration.

Put s_ij=h_Tj(m_i). Every vertex of the jth dual in (4) is

    a^(j,l)=λ^(l)/(Σ_i λ_i^(l) s_ij).                          (6)

Indeed that dual is a positive transverse section of the same nonnegative
normal-closure cone, so its vertices are its five extreme rays. Each
denominator is positive because T_j has interior and λ closes the normals.
Therefore (4) becomes

    c(K×_L Q)=min_l [λ^(l)·h / D_l(θ)],
    D_l(θ)=max_j Σ_i λ_i^(l) h_Tj(m_i).                        (7)

This min/max order is essential: the numerator is fixed at a given l and is
positive for a full-dimensional K, since translation can put zero inside K.
Taking the smallest ratio across templates uses the largest denominator.

Cyclically shifting λ and the normal labels rotates the support evaluation by
2π/5. The complete ten template classes are permuted by that rotation;
translations do not change the weighted sum because λ closes the normals.
Hence all five D_l(θ) coincide. Call their common positive value Dθ. We obtain

    c(K×_L Q)=min_l λ^(l)·h / Dθ.                              (8)

This is the positive geometric compression: the ten templates have a single
common normalization on the five circuit rays. It is not an all-feasible HK
relaxation or a guessed scalar descriptor.

## The inner regular pentagon reads the same five circuits

The containment z+tPθ⊆K_h is precisely

    m_i·z+b t<=h_i, every i.

Its translation-and-scale LP dual is

    ιθ(K)=min_{λ∈C} λ·h/b = min_l λ^(l)·h/b.                  (9)

The maximum is attained because K is bounded and has interior. For Pθ itself
all supports equal b, so (8) gives c(Pθ×Q)=b/Dθ. Combining this with
(8)–(9) proves (1).

Choose a maximizing placement of ιθ(K) Pθ inside K. Area monotonicity gives

    area(K)>=ιθ(K)² area(Pθ).

Substitution into the definition of sys proves (2). Both bodies have interior;
a proper containment of convex bodies has a positive area difference. Equality
therefore forces K to equal that translated inner regular pentagon. Conversely
translations and dilations give equality. This proof includes every collapsed
facet case automatically and uses no support-chamber Hessian premise.

## Exact aligned-angle packet and costs if independently replayed

At θ=0,

    b=(1+sqrt5)/4, w=(5+sqrt5)/4,
    D0=b/w²=(sqrt5−1)/5,
    c(K_h×Q)=5/(sqrt5−1) min_l λ^(l)·h,
    max sys=(5+2sqrt5)/10=0.9472135955….

All geometry can be represented in Q(sqrt5) using coordinates (X,Y)=(x,y/τ),
τ=tan(π/5), τ²=5−2sqrt5. The transformed covectors are

    (1,0),
    ((sqrt5−1)/4,(5−sqrt5)/4),
    (−(1+sqrt5)/4,(3sqrt5−5)/4),
    (−(1+sqrt5)/4,−(3sqrt5−5)/4),
    ((sqrt5−1)/4,−(5−sqrt5)/4).

The ten canonical triangles are rotations by multiples ofπ/5 of
conv{(-R,0),(R,0),(0,R)} in these transformed coordinates, where
R=(5−sqrt5)/10. The physical rotationπ/5 is represented by

    [[b, -(3sqrt5−5)/4],
     [b, b]].

This coordinate transformation is used consistently for normals and templates;
it is not asserted to preserve capacity under changing just one factor. Actual
area equals τ times computational area.

The independent algebraic check of all5×10 support denominators passed:
50 weighted support evaluations and exact maximum comparisons. It checks the
normalization imported from (3); it is not needed to enumerate candidate bodies.
The superseded global polyhedron approach would have five fitting inequalities
plus five nonnegative-edge inequalities, in translation quotient dimension3,
so at most C(10,3)=120 intersection systems. No such enumeration was run.
The exact geometric theorem (1) makes those candidate-system costs unnecessary.
The reviewer checked closure completeness, single-orbit symmetry, positive
denominators, the min/max direction, attained core containment and all edge-loss
cases. The lead retained and reran the exact coefficient verifier successfully:

    uv run topics/empirical-studies/discovery-design/night-20261005/geometry-discovery/fixed-q-rank/verify_equiangular_denominators.py

[The verifier](verify_equiangular_denominators.py) uses exact Q(sqrt5) comparisons
and writes no files. Its five maximum denominators are all (sqrt5−1)/5.

## Novelty, existing coverage and falsifier

The September retained ZIP proves the special prescribed ten-direction fan,
whose normals are π/10+(π/5)k in the desk convention. It contains the HKO
orientation θ=π/10 moduloπ/5. That source is:

docs/history/pro-handoffs/msc-math-consolidation-20260924/msc_math_consolidation_codex.zip
→ incoming/hko_fixed_pentagon_results.zip
→ mathematics/TEN_DIRECTION_PROOF.md.

Its full fan certificate and Brunn–Minkowski vertex method should not be
recomputed or presented as new. The capacity-core proof above covers every
orientation and gives a different exact determining relation. The independent
affine theorem only treats affine images of regular pentagons; arbitrary
supports with these prescribed regular normals need not be affine regular.
A broader literature novelty assessment remains unperformed.

A counterexample would have to expose a missing closure circuit, a broken
rotation permutation of the complete template family, an invalid positive
normalization, or a mismatch between (9) and containment. A single fitting
scale is insufficient: (7) explicitly checks the minimum over the complete ten.
A body outside this five-normal fan cannot falsify the theorem and may have
capacity not determined by an inner regular pentagon. General pentagons,
nonregular normal fans, and arbitrary convex partners remain open domains.
