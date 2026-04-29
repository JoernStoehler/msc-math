# Planned TOC

This is a quickly written plan for the thesis structure.
I call it a TOC even though it's not quite that. It's more of a "summary tree" that tries to split into child nodes until I believe every leaf is straightforwardly doable by me.
The goal is to ensure that way that my plan is feasible (since a sequence of leaf nodes then guarantees that everything is doable, even if it takes time) and optimize the plan conveniently (e.g. order, nesting are revealed, I can rearrange, merge+split until the whole surface is nice).

The main things-to-optimize for are

- time: there isn't a benefit to waste a week just to gain low marginal gain; or, the other way around, if we want to keep our (current) deadline then we need to not waste time inefficiently / need to triage what to do
- readability: Kai and Elizabeth need to be able to understand what work was done as part of the master thesis
- completeness: it'd be sad, and kinda weird, to loose results we obtained
- correctness: it'd be sad, and bad for the grade, if results are wrong, or if the arguments for the results are wrong even if the conclusions are correct
- proof completeness: stronger than completeness, bc it's about potentially putting in work even now to close proof gaps, instead of just mentioning them (correctness requires to not gloss, completeness to not drop partial proofs)

Rough guidance: there's basically standard structures, and for good reason, that we follow.

Since we have SO MUCH CONTENT we heavily rely on offering content to be skimmed. In particular, we want to spoiler sections early so that readers can just ignore the detailed arguments if they want to. This also helps readability bc it provides context that helps interpret the details.

We use handdrawn sketches (e.g. to illustrate definitions and theorems and edge cases) instead of trying to do them professional - it's not worth the time. 
We use figures (matplotlib mainly) to illustrate results using human visual reasoning (e.g. for statistics).

## TOC (living document)

Title: "Probing Viterbo's Conjecture"

Abstract: 
  - standard abstract
  - probably a long paragraph instead of just 5 sentences
  - we want to spoil our results here, sort of a 'paper' style
  - potentially relevant for publication on arxiv

1. Introduction
  - standard introduction style
  - motivation and context: brief recap of Viterbo's conjecture's origins, attention paid to it, and the surprising counterexample in HKO2024 ; recap of computational approaches (HK2017, CH2021) to look for counterexamples and how they're different from HKO2024 in where they searched; natural idea to develop computational methods to do a large search and apply standard computational methods; highlight recent interest in data-science for pure math to discover connections (albeit not proofs) (e.g. knot theory papers, discussed in the lecture at uni augsburg)
  - state Viterbo's conjecture
  - operationalization/narrowing of the topic to be tractable: we focus on polytopes (computable and dense), we focus on the 4D case (bc high dimensions explode in difficulty, see e.g. the paper about NP completeness, and in computational cost), we focus on both generic and non-generic cases (since HKO2024 counterexample is highly non-generic, e.g. lagrangian product with high symmetry) ; we build upon existing computational methods but improve them, and implement them in a high-performance language (rust); we deal properly with numerics (since we want to trust the results) e.g. with interval-arithmetics and exact-arithmetic fallbacks
  - results/contributions: we find that HKO2024 looks like a local maximum of the systolic ratio in the space of convex bodies (proven for polytopes with 10 facets, ofc up to symmetries of the systolic ratio). We find that hammering with a standard data science book and the LICCA cluster at the problem yields no new counterexamples and no conjectures; theoretical considerations merely yield insights already exploited automatically by local gradient ascent, and so non-local considerations would be needed; 
  - side result: we flesh out the proof from HK2017 that there always is a simple Reeb orbit with minimum action on a polytope
  - method result: we developed a high-performance implementation of the computational methods from HK2017, CH2021, in particular we optimized the algorithms by using more of the combinatorial structure of the problem, and we hardened them using numerics, interval arithmetic, exact arithmetic fallbacks
  - method result: we developed an algorithm for the subgradients, and a gradient ascent method for the non-smooth problem
  - theory result: we discuss the negative result that standard data science approaches yield nothing besides the local information, in particular we discuss volumes of local maxima and the dimensionality of the search problem as a way to get a prior for why interesting cases are rare to find, and how subsequently data science methods lack enough examples (in particular, have only 1 example) of interesting cases
  - main result: we prove that HKO2024 is a local maximum in M_10 mod sym
  - side result: we prove a formula for P_5 x_L R(theta) P_5
  - side result: we empirically exhaust high-symmetry families of polytopes and find no counterexamples
  - structure of the thesis is sketched
  - we refer to the notation appendix for the standard notation we picked from the literature

2. Background
  2.1. Polytopes and their euklidean geometry
    - VPolytope, HPolytope, correspondence between them
    - k-faces (for us: closed)
    - dual polytopes, support and gauge functions
    - the topological space of convex bodies, the topological space of polytopes
    - the topological space M_F of polytopes containing the origin in their interior with F irredundant facets, and embedding into an open subset of R^4F via dual vertex coordinates
    - note on "generic properties", and promise we will introduce finitely many of those throughout the thesis
  2.2. Smooth symplectic geometry setting
    - notation J_0, omega_0, lambda_0, Sp(4), action
    - definition: minimum action of a convex body with smooth boundary
      - Reeb vector field, Reeb orbits, action of a Reeb orbit, minimum action
      - cite only: existence of at least one Reeb orbit, existence of a minimum
    - definition: symplectic capacity axioms
      - monotonicity (A \embeds into B implies c(A) <= c(B))
      - conformality (c(lambda A) = lambda^2 c(A))
      - normalization (c(B^4(1)) = c(Z^4(1)) = pi)
    - cite only: minimum action is a symplectic capacity, called EHZ capacity
    - Viterbo's conjecture: for any convex body K, c(K)^2/2/vol(K) <= 1
    - Cite only: relation/implications, e.g. to whether on convex bodies all capacities coincide, etc
  2.3. Symplectic geometry on polytopes
    - main method: polytopes as limits of smooth convex bodies
    - definition: generalized Reeb orbits on polytopes
    - theorem (CH2021): K_n -> K wrt Hausdorff, action(gamma_n) bounded => gamma_n has a subsequence converging to a generalized Reeb orbit gamma on K wrt W^1,2 topology
      - TODO: LOOK UP IF THIS THEOREM WAS STATED IN CH2021
    - corollary: c_EHZ(K_n) -> c_EHZ(K)
    - definition: simple Reeb orbits on polytopes
    - theorem (HK2017): for any polytope, there is a simple Reeb orbit with minimum action
    - proof: fleshed out in its own, skippable chapter since it uses heavy machinery such as Clarke's dual action principle

3. Methods
  3.1. HK2017 algorithm
    - recall the minimum action optimization problem
    - definition: the HK2017 optimization problem in (sigma,beta)
    - theorem: it is equivalent to the original problem
      - UNSURE: whether we can give the proof or still need the Clarke's dual action principle; I think we can give the proof
      - lemma: shoelace formula for the symplectic action of a piecewise linear curve
    - theorem/formulas for how to recover gamma from (sigma,beta) and vice-versa
    - algorithm: the HK2017 solver we developed
      - input: F dual vertices a
      - output: a partial permutation sigma: 1..m -> 1..F, and beta \in R^m, s.t. Q(sigma,beta) is a global maximum, when one understands sigma,beta to extend to a full permutation by padding with zeros
      - loop over all sigma: 1..m -> 1..F partial permutations
        - solve linear constraints:
          - variables: beta \in R^m
          - constraints: sum_i beta_i a_{sigma(i)} = 0, sum_i beta_i = 1
        - solve, yields a m' dimensional solution space {beta'} (can be empty!)
        - project quadratic objective:
          - H(sigma)_ij = omega_0(a_{sigma(i)}, a_{sigma(j)}) * sign(j-i) (symmetric matrix)
          - Q(sigma,beta) = 1/2 beta^T H(sigma) beta = sum_{i<j} beta_i beta_j omega_0(a_{sigma(i)}, a_{sigma(j)})
          - project onto the m' constraint space, yields an objective: 
            Q'(sigma,beta') = 1/2 beta'^T H'(sigma) beta' + b'(sigma)^T beta' + c'(sigma)
        - if H' is not negative definite, then skip
        - find the critical point beta'* (which may not exist)
        - recover beta* by projecting back to the full space
        - if beta* > 0 and Q(sigma,beta*) > Q_best, then update the best solution
      - return the best solution found
    - theorem: the algorithm indeed yields a global maximum
      - main arguments:
        - the admissable beta \in C(sigma) sets are compact, and P(1..F) is finite, and Q is continuous, so the maximum is attained
        - decompose C(sigma) into open faces (including the polytope interior, and the vertices)
        - the faces are given by setting some of the beta_i to zero; so enumerating partial permutations is the same as enumerating faces
        - there's now a global maximum with minimum face dimension
        - since it lies inside the open face, it's a critical point of the projected objective
        - if H' had a positive eigenvalue, then there'd be a higher Q value nearby
        - if H' had a zero eigenvalue, then we could follow that direction until we hit the boundary of the face, which would contradict the minimum face dimension
        - the beta>0 check is needed since there may be critical points outside C(sigma)
    - improved versions (not explicitly restated):
      - we can track not just one best solution, but all best solutions
      - we can track also the semidefinite cases that define >= 1 dimensional families of maxima (sigma,beta)
      - we can expand the sigma to full permutations via padding
  3.2. Subgradient algorithm
    - we want to analyze the local neighborhood of a polytope K
    - for this we look at the HK2017 optimization problem with fixed sigma
      A_min(K) = min_sigma A_min(K;sigma) where sigma \in PartialPerm(F)
      A_min(K;sigma) = min { action(beta;sigma,a) :
        sum_i beta_i = 1
        sum_i a_i beta_i = 0
        action = 1/2Q
        Q = sum_{i<j} beta_sigma(i) beta_sigma(j) omega_0(a_sigma(i), a_sigma(j))
        beta is a critical point of the constrained problem
        the constrained problem is negative definite
        beta_i > 0
      } - which can be undefined
    - considering these equations/conditions with the frame of algebraic geometry tells us
      - generically, the constraints are maximum rank i.e.
        rank(1^T \\ a^T) = min(5, |sigma|)
      - the constrained quadratic problem is negative definite on an open set of polytopes
      - 
    
    in order to describe the behavior of K_n -> K we need closed conditions everywhere, so we need to modify our algorithm:
      - now use beta_i >= 0 instead of beta_i > 0
      - now use negative semi-definite as requirement instead of negative definite
      - record the potentially infinitely many critical points beta for eigenvalue-zero cases
    - this yields a larger set of minimizers, but importantly now every K_n->K has a converging subsequence
      sigma_n -> sigma [wlog we just pick a subsequence with sigma_n=sigma]
      beta_n -> beta [wrt R^|sigma|]
      sigma,beta is a minimizer for K
    - main insight: sys(K) is not smooth in the dual vertices a, but we can define a branch for each partial permutation sigma
      beta(a;sigma) convex subset of R^|sigma|, can be empty
      the beta have all the same action action(a;sigma) \in R \union {undefined}
      action_min(a) = min_{sigma} action(a;sigma) \in R
      Sigma_min(a) = argmin_{sigma} action(a;sigma) \subset PartialPerm(F) , nonempty
    - the limit behavior (sigma_n,beta_n) -> subsequence -> (sigma,beta) guarantees that Sigma_min(a), beta(a;sigma) are both hemi-continous
      and action_min(a) is continous
    - per hemi-continuity, the set of a where action(a;sigma) is defined, is closed; this can also be obtained by realizing the conditions of "beta(a;sigma) is not-empty" are closed
    - we can also see that "beta(a;sigma) is unique" is a dense open condition, so in particular it's a generic condition
    - this implies that "beta(a;sigma) has dimension >=2" is a closed condition with dense complement
    - [Venn-diagram of the conditions and their intersections, or a table, or sth concise for lookup / for verification]
    - 