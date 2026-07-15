# Generator-map distortion and boundary diagnostics

This target-free packet asks what the accepted local Jacobian spectra and
rejection-boundary margins say beyond rank. It keeps the accepted
`generator-map-jacobian-rank` packet immutable and binds its report and analyzer
hashes.

Raw singular values depend on both the body-chart metric and latent
coordinates. Analytic coordinate-scaling and redundant-fiber controls show
that raw pseudo-determinants and condition numbers can change without changing
the underlying body law. They are therefore retained only as within-native-
parameterization sensitivity diagnostics, never as cross-law density scores.

The one honest change-of-variables comparison is the equal-support Dirichlet
family. For fixed `n`, all alpha values use the same gap simplex coordinates
`dg_1...dg_(n-1)` and the same body-chart Euclidean Hausdorff measure. The
packet computes the Dirichlet density and body-volume Jacobian at all 36
alpha=1/4/16 reference bodies, evaluating all three laws at each same body.
It reports both a single linked-label branch and, for generic bodies, the
unlabeled-body density after adding the `n` cyclic starting-facet preimages.
Reversing traversal order labels the same vertex set; reversing the positive
gap sequence spatially reflects a generic body and is not another preimage,
because the chart does not quotient reflection. A retained asymmetric control
checks this distinction.

The generator rejects any draw with a gap at least `pi`. Densities therefore
divide by the exact acceptance probability
`Z=1-n*2^(-(n*alpha-1))*sum_{k=0}^{alpha-1} binom(n*alpha-1,k)`.
This follows from the Beta marginal/binomial-tail identity and the fact that
at most one simplex coordinate can exceed `1/2`. The report retains exact
rational and floating values for all nine `(n,alpha)` settings; no SciPy
dependency is used.
Regular is a Dirac law. Baseline, zonogon, mutation, and primal-hull density
claims are explicitly abandoned because quotient fibers, clipping, inactive
points, or conditioning require a fuller coarea/fiber-mass contract.

Dimensionless or scale-normalized margins record proximity to named normal-fan,
support, clipping, gap, convexity, hull-active-set, radial, and origin-interior
boundaries. They are diagnostic distances, not rejection probabilities.

Reproduce after the separate source commit:

```bash
uv run --script analyze.py --out-dir artifacts
uv run --with numpy==2.5.1 --script test_packet.py
uv run --with numpy==2.5.1 --script test_reproducibility.py
```

The output supports within-law/native-parameter sensitivity and declared-
measure Dirichlet comparisons only. It does not support cross-law density,
coverage, quality, topology, rare-mode, target, or `sys` claims. Expected
change-of-variables statements are agent-reviewed mathematics, not Jörn-
reviewed theorems.
