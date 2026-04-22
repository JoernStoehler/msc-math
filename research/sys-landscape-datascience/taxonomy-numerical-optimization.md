# Taxonomy: Numerical Optimization

Source intent:

- Frozen taxonomy snapshot for classical nonlinear optimization adapted mainly from Nocedal--Wright, *Numerical Optimization*.
- This file is the main external home for gradient-based local optimization language and standard local constrained/unconstrained optimization families.

## Smooth Unconstrained Optimization

- `NUMOPT-GRADIENT` Gradient descent / ascent
- `NUMOPT-LINESEARCH` Line-search methods
- `NUMOPT-TRUST` Trust-region methods
- `NUMOPT-NEWTON` Newton methods
- `NUMOPT-QUASINEWTON` Quasi-Newton methods
- `NUMOPT-CG` Nonlinear conjugate-gradient methods

## Smooth Constrained Optimization

- `NUMOPT-PENALTY` Penalty methods
- `NUMOPT-BARRIER` Barrier / interior-point methods
- `NUMOPT-SQP` Sequential quadratic programming
- `NUMOPT-ACTIVESET` Active-set methods
- `NUMOPT-PROJECTED` Projected-gradient methods

## Nonsmooth And Subgradient Methods

- `NUMOPT-SUBGRAD` Subgradient methods
- `NUMOPT-BUNDLE` Bundle methods
- `NUMOPT-PROX` Proximal methods

## Globalization And Initialization Strategies

- `NUMOPT-INIT-MULTISTART` Multi-start initialization
- `NUMOPT-INIT-CONTINUATION` Continuation / warm-start style initialization hooks

## Likely Use In This Project

- Strongest overlap: gradient-based local optimization, step-size control, local convergence language, constrained local refinement.
- Moderate overlap: nonsmooth/subgradient language when the search surface is only piecewise smooth.
- Weak overlap: polished second-order local methods unless a thesis packet explicitly compares them.
