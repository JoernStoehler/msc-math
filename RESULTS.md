# Results

What this project found and built.
Written in a terse bullet-list style for easy maintainability.
Not intended to be complete/detailed - consult the repo folders for that.
No particular order or grouping.

Developed Methods:
- Capacity: We can compute the systolic ratio of polytopes in R^4, two algorithms based on HK2017, CH2021
- Reeb orbits: We get the set of simple minimum action Reeb orbits
- Perturbations: We can compute 1st and 2nd order perturbations
- Gradient Ascent: We can do gradient ascent on the systolic ratio, with some tricks
- Random Polytopes: We can sample polytopes with a distribution that looks representative to us

Results:
- HKO2024 is a local maximum ...
  - Proven: ... for 10-facet polytopes
    - Uses 1st and 2nd order perturbation methods
  - Conjecture: ... for 11,12,13-facet polytopes
    - Empirics: random perturbations that add facets reduce $sys$
    - Empirics: gradients point back to HKO2024
  - Speculation: ... for convex bodies
- No novel $sys>1$ examples found
  - Empirics: Random sampling and gradient ascent
    - Find local maxima with $sys < 1$ only
    - Attractors are exponentially small wrt the dimension $4F$
    - The one known $sys>1$ attractor of HKO2024 is also small
    - Most ideas we have for guided search are local, and covered by gradient ascent
    - Some polytope spaces (e.g. products, symmetries) are lower dimensional, but still too big
    - Empirics: a histogram fit of $sys$ predicts exponentially few $sys>1$ cases
  - Regressions against euclidean polytope data yielded no insights
  - Among products of regular polygons, HKO2024 is the only $sys > 1$ case
    - Proven: A closed formula for the systolic ratio of pentagon x rotated pentagon
    - Empirics: no other m-gon x rotated n-gon we visited is $sys > 1$
    - Theory: some arguments for why a large lcm(m,n) rules out $sys > 1$
- Visualizing the \partial K setting yielded no new insights

Rejected ideas:
- Machine Learning to produce candidates
  - The gap between HKO2024 and all other $sys<1$ examples suggests qualitatively different behavior
  - Bet: ML would just learn minor improvements in the $sys < 1$ region, e.g. ascent in fewer steps, memorizing an especially good $sys \approx 1$ polytope or the region around it, a non-expential reduction in what minima to look at
- Richer data for regressions: what data is there to add?
- Throw more compute at it:
  - We can cover maybe densities like $10^{-6}$, but the hypothesis is now that we're in the $10^{-F}$ to $10^{-4F}$ regime (depending on how one models sys as a random distribution)

Open Ideas:
- TODO

Datasets:
- Size: we used optimized Rust code to run 10^5 polytopes with up to 12 facets
- Families (Seed):
  - Generic: random dual vertices in R^4
  - Product: lagrangian products of random polygons in R^2
  - Regular Product: lagrangian products of regular polygons with free rotation between them
  - Literature: polytopes with known/conjectured capacity from the literature
- Transformations:
  - Random finite perturbations, including random extra facets
  - Gradient ascent, intermediate steps
  - Gradient ascent, final local maxima
- Various tiny custom datasets e.g. for capacity axioms validation, regression tests, edge cases

Quality:
- Performance: E2E profiled, benchmarked; bottlenecks are now in the theory and eigendecompositions of small matrices
- Proofs: idealized algorithms are rigorously proven correct, we cite standard literature in a few places
- Validation: we validated our methods using as many propositions and test cases as possible
- Numerics: we have error bounds with proofs, we have empirical errors compared to exact rational arithmetic

Project:
- Reproducibility: The complete project is reproducible, and the majority of the project's history is stored in the git repo
- Documentation: the project is self-documenting, including research notes and workflows
- Library: our methods are available as a polished Rust library for further use
- AI Agents: Usage patterns of AI agents and experiences are documented for future research
