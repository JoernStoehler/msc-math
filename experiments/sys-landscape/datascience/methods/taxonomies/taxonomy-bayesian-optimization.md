# Taxonomy: Bayesian Optimization

Source intent:

- Frozen taxonomy snapshot for Bayesian optimization adapted mainly from Garnett, *Bayesian Optimization*.
- This file exists because BO is a common external search family even if the current repo never uses it.

## Surrogate Models

- `BO-SURR-GP` Gaussian-process surrogate models
- `BO-SURR-BAYESREG` Simpler Bayesian surrogate models

## Acquisition Strategies

- `BO-ACQ-EI` Expected improvement
- `BO-ACQ-PI` Probability of improvement
- `BO-ACQ-UCB` Upper confidence bound
- `BO-ACQ-ENTROPY` Entropy / information-based acquisition

## Practical Variants

- `BO-BATCH` Batch / parallel Bayesian optimization
- `BO-CONSTR` Constrained Bayesian optimization
- `BO-MULTIFID` Multi-fidelity Bayesian optimization
- `BO-HIGHDIM` High-dimensional Bayesian optimization

## Likely Use In This Project

- Strongest overlap: surrogate-guided black-box search over expensive objective evaluations.
- Weak overlap: current repo state, unless the thesis later wants an explicit "we did not try Bayesian optimization" row under an external source.
