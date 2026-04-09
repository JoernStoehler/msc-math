# Results

<!-- How this file works:
Answers "what should the thesis say?" — not a repo inventory.
Sections group related takeaways by theme.
An item belongs if: (1) Kai needs it for "is thesis done?" (2) worth bragging about,
(3) impacts thesis structure, or (4) natural dependency cluster.
Stubs and aspirational items belong. Working-notes style. -->

[main result]
- the standard data scientist's toolkit fails to find new sys>1 examples.
- the shared reasons for this are that a) the sys function on polytope space has many local maxima, and each has a tiny attractor space, and all except the 1 known counterexample HKO2024 have sys<=1; and b) there is very little euclidean and symplectic data we have about polytopes that we can use to search for patterns among the local maxima / among the non-local-maxima polytopes ; the entire data scientist's toolkit relies on either having globally relevant local behavior or rich enough data to exploit, and evidently we just don't have either

[main result]
- for the one sys>1 example HKO2024 we know, we conjecture it's a local maximum in polytope space
- we proved this for the subspace of 10-facet polytopes and empirically validated the conjecture for up to 13-facet perturbations, so we have some reason at least to believe this
- relating to the first point: the attractor of HKO2024 is tiny as well

[methods]
- in order to do the above investigations we
  - developed algorithms for capacity, minimum action simple reeb orbits, 1st and 2nd order perturbations, a 1st order "gradient ascent"-like method with standard tricks to escape saddle points
  - proved correctness of the algorithms in an idealized version, proved or at least empirically measured error bounds for their floating point implementations, and used slower but exact rational arithmetic to get exact values where needed / as a fallback
  - optimized performance until we could get large enough datasets
  - developed random polytope distributions we believe to be representative for our purposes of the full polytope space
  - fleshed out mathematical literature to be more accessible to master students, and added own results to close minor gaps

[minor, standalone]
- crosspolytope capacity: c_EHZ = 4.0, sys = 3/4 — new computation, satisfies Viterbo. Same capacity as hypercube despite different volumes
- a closed formula for sys(pentagon x rotated pentagon) with proof [aspirational]
- a visualization of the 4d geometry on a computer screen

[non-math deliverables]
- the project is available for future research
  - with a library of our algorithms, development tests, documentation
  - the full data analysis pipelines and research notes
  - the complete development environment used
  - notes about workflows, conventions and how we involved AI
  - the majority of the history of the project, e.g. if earlier results / if the methodology we used is interesting to a future researcher
- we analyzed what AI contributed to this project, and speculate what counterfactual impact it had and what lessons to draw from that
