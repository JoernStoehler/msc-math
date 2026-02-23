# Tube Algorithm — Working Notes

**Status:** Scoping session — Jörn writes, then we refine together.
**Branch:** tube-algorithm
**Related:** CH2021 (in papers/ch2021/)

---

## Jörn's Notes

<!--
   Write your algorithm description here.
   Anything is fine: pseudocode, geometric intuition, key steps, edge cases,
   definitions you need, open questions, references to CH2021 sections.
   Don't worry about precision — we'll sharpen it together.
-->

### Setting (which polytopes does the tube algorithm apply to?)

Polytopes K \subset R^4 that have no 2-face that is lagrangian.
Definition nuances:
- facets for us are non-degenerate, i.e. we have an irredundant H-Rep, and each facet is fully 3-dimensional
- facets are closed sets, we say facet interior for the open interior.
- 2-faces are non-degenerate, i.e. they are 2-dimensional. Note that intersections of 2 facets can be empty,0,1,2-dimensional.
- again 2-faces are closed, with open interior
- Lemma: every 2-face is the intersection of exactly 2 unique facets, i.e. not 0-1 or >=3. (Follows from a dimension argument)
- 1-faces are defined similarly: 1-dim, closed, open interior
- 0-faces are defined similarl: 0-dim, clopen
- Note: 1-faces CAN be intersections of >=3 facets, there's no dimension argument that saves us here. 0-faces CAN be intersections of >=4 facets, same reason.
- A 2-face is lagrangian iff omega_0 |_ T F_ij = 0, where F_ij is the 2-face defined by facets i and j.
- We only handle polytopes with 0 lagrangian 2-faces ; i.e. for all i,j where F_i \cap F_j is 2-dimensional, we have omega_0(n_i, n_j) != 0. Note: you cannot check that property by looking at any pairs of facets, you must look at the ones that have a 2-face intersection!
- For non-lagrangian 2-faces, the sign of omega_0(n_i, n_j) gives us the direction of the Reeb flow through the 2-face, i.e. {i,j} now is ordered, s.t. trajectories through an interior point x \in int(F_ij) flow from F_i to F_j. 

### Key idea / geometric picture

Instead of solving the optimization problem in one go as in HK2017 for a given simple Reeb orbit combinatorics, we construct solutions iteratively.

We define the "tube" of a combinatorial structure as follows:
- Consider a squence of facets F_sigma(1), ..., F_sigma(k) 
- A piecewise affine, pure (i.e. no nontrivial convex mixtures of Reeb vectors at points on <=2-faces) Reeb trajectory gamma is of type sigma if it has some interval that has a partition t_0 < t_1 < t_2 < ... < t_k, such that \dot\gamma(t) = R_sigma(i) for t in (t_{i-1}, t_{i}) ; note that this implies \gamma(t) \in F_sigma(i) for t in [t_{i-1}, t_{i}]. We demand that t_0 and t_k are breakpoints, i.e. there's some F_sigma(0) and F_sigma(k+1) in principle, though the combinatorics does not fixate them. We won't care about the time parametrization here, so we also demand for k>=1 that t_1 = 0
- The tube of sigma is now the set of all trajectories of type sigma ; we call it a tube, bc on [t_0, t_k] it looks like a tubular region. The dynamics on the polytope are piecewise affine AND we fixed the combinatorics, s.t. there are no jumps i.e. we are globally affine, and thus in particular the tube is "convex" in the sense that if gamma_1 and gamma_2 are in the tube, then any affine convex combination of them is also in the tube. Note that the spatial shape need't be convex!
- We can also map tubes to "cuts" where we take one point gamma(t_gamma) ; the t1 cut is {gamma(t_1=0): gamma in tube} \subset F_1 \cap F_2 (assuming k>=2 F_2 is gamma-independent), but we can also consider the t_{k-1} cut where t_{k-1} is gamma-dependent. As long as gamma <-> t is a bijection, we can reconstruct the tube.

Now the iteration is:
- start with the full tube of combinatorics [] i.e. all Reeb trajectories ;
- refine the combinatorics by appending a facet F_k
- the new tube is now a subset of the old tube
- computationally we can represent tubes by tracking a bunch of data of interest that we then iteratively compute the new data for i.e. there's an algorithm (F_0...k, data, F_{k+1}) -> (F_0...k+1, data')
- since we search for closed simple Reeb orbits, we can dismiss tubes that have duplicate facets
- we have a special extension step called "closing the tube":
  - we have a tube F_0...k and extend it with F_k+1=F_0 and F_k+2=F_1
  - all trajectories now fulfill gamma(t1=0) \in F_0 \cap F_1 and gamma(t_{k+1}) \in F_k+1 \cap F_k+2 = F_0 \cap F_1
  - any simple periodic Reeb orbit must fulfill gamma(t1)=gamma(t_k+1) now!
  - so we check the affine map psi' that is part of the data:
    psi' : H_0 \cap H_1 \to H_k+1 \cap H_k+2 = H_0 \cap H_1
    for fixed points
    and then check whether it lies in the set
    Start' = {gamma(t1=0): gamma in tube'} \subset F_0 \cap F_1
    If multiple points (1-dim, 2-dim subset) we have to optimize over that set to find the ones with minimum action, since we care about not just ~all simple Reeb orbits, but the ones with minimum action. If no fixed points, we can dismiss the tube as containing no simple closed Reeb orbits.
- The search above is exponential, but we can using the data exit early!
- Lemma: a tube contains no minimum action simple Reeb orbit if the lower bound over all trajectories in the tube is already above the (best known cancidate) minimum action !
- Lemma: a tube contains no minimum action simple Reeb orbit if it contain no trajectories at all i.e. Start=empty
- Lemma (CH2021): a tube contains no minimum action simple Reeb orbit if the rotation number of the trajectories on the time interval (t0 + eps, t_k+1 - eps) (which equals the rotation number on (t1 - eps, t_k + eps) ) is above the known upper bound "2" (iirc counted as full turns?) -- this uses that at least one minimum action Reeb orbit has to exist with 1 < rotation < 2, and that refining the tube increases (non-decreases?) the rotation number, i.e. the rotation number increases as we increase the time interval we look at.
- also ofc we can throw away tubes with duplicate facets -- instead we just have to whenever possible consider the closing of a tube, i.e. try first whether F_0,F_1 can be appended legally or if that makes the tube be empty due to facet-adjacency considerations
- There's some nuance to figure out still: should we really demand t_i < t_i+1 or should we allow t_i = t_i+1 ? This has to do with how we want to sort trajectories that go through the interior of 1-faces and 0-faces instead of sticking to the interior of 2-faces and 3-facets.
- Also instead of Start we may want to track End since that may be easier to compute the refinement of
- The algorithm has a lot of precomputable data: e.g. which tubes
[F_1, F_2] are empty already? since then we can skip any tubes [..., F_1, F_2, ...] as well. Or what are the affine maps psi for [F_1, F_2, F_3] ? etc. What are the standard 2-dim bases for F_ij ?
- The data probably is sth like
  - Start and End point set in the 2-dim basis, i.e. a convex 2-dim polytope in R^2
  - Affine map: Start -> End of the trajectories on the interval [t1, t_k] ; can be written in the 2-dim bases of H_0,1 and H_k-1,k
  - Affine function : End -> Action of the trajectories on the interval [t1, t_k] 
  - Rotation: number
- Extending the data when we extend the tube by F_k+1 then is
  - compose the affine map with the precomputed [F_k-1, F_k, F_k+1] map
  - compose the affine function, and add the precomputed affine increment for [F_k-1, F_k, F_k+1]
  - add the rotation precomputed for [F_k, F_k+1]
  - update the End point set by intersecting with [F_k, F_k+1] polygon, similarly update the Start point set by intersecting with the [F_k, F_k+1] polygon pulled back along the affine flow map to H_0\cap H_1.
- As a hack ofc to not deal with weird tubes with k<=2, we can just start with tubes [F_1, F_2, F_3] of which we know they aren't empty.


### Algorithm steps (high level)

### Edge cases / corner cases I'm aware of

Biggest issue: trajectories that go through the 1-faces, 0-faces instead of sticking to the interior of 3-facets and 2-faces.

### Correctness argument (sketch)

### Performance vs HK2017 (why is it faster / what's the cost model?)

### Open questions I already see

---

## Questions from Claude (to be answered during discussion)

See conversation — Claude will add questions after reading.

---

## Settled definitions and steps (post-discussion)

<!-- We'll fill this in as we converge. -->
