# Taxonomy: Derivative-Free / Black-Box Optimization

Source intent:

- Frozen taxonomy snapshot for optimization/search methods adapted from derivative-free and black-box optimization references.
- Main anchor sources: Audet--Hare, *Derivative-Free and Blackbox Optimization*; Conn--Scheinberg--Vicente, *Introduction to Derivative-Free Optimization*; Rios--Sahinidis (2013) as survey bridge.
- This file is the main external home for search baselines, direct-search families, multistart wrappers, and non-Bayesian surrogate-guided search.

## Baseline And Sampling Search

- `DFO-BASE-RANDOM` Blind random search / random sampling
- `DFO-BASE-LHS` Space-filling random designs such as Latin hypercube sampling
- `DFO-BASE-MULTISTART` Multistart or restart wrappers around local search

## Direct Search And Pattern Search

- `DFO-DIRECT-PATTERN` Pattern search
- `DFO-DIRECT-GPS` Generalized pattern search
- `DFO-DIRECT-MADS` Mesh adaptive direct search
- `DFO-DIRECT-NELDERMEAD` Nelder--Mead simplex search

## Model-Based Derivative-Free Search

- `DFO-MODEL-QUAD` Local quadratic / interpolation model methods
- `DFO-MODEL-TRUST` Derivative-free trust-region methods
- `DFO-MODEL-SURROGATE` Surrogate-assisted black-box optimization

## Global And Hybrid Black-Box Search

- `DFO-GLOBAL-BRANCH` Deterministic branch-and-bound style global black-box search
- `DFO-GLOBAL-STOCHASTIC` Stochastic global-search wrappers
- `DFO-GLOBAL-HYBRID` Hybrid local/global black-box search

## Constraint Handling

- `DFO-CONSTR-BOX` Box-constrained derivative-free optimization
- `DFO-CONSTR-GENERAL` General constrained derivative-free optimization

## Likely Use In This Project

- Strongest overlap: random search baselines, multistart wrappers, derivative-free local refinement, surrogate-guided search.
- Moderate overlap: constrained black-box search where explicit gradients are not available or not trusted.
- Weak overlap: heavy deterministic global-optimization families unless the thesis later emphasizes certified global search.
