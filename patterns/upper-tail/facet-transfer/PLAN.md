# Bounded combinatorial transfer challenge, 17 October 2026

Reporting owner: `/root/facet_transfer`; receiving/integration owner: `/root`.
End state: an exact strict failure of the unchanged LP-seeded completed-response extractor on a genuine near-HKO 6x6 product with ratio above one, with a mechanism and, if strict certificates permit, an open-set obstruction; or a bounded negative result explicitly adding no insight when all cases agree. This tests crossing a facet-combinatorics boundary rather than polishing five-facet agreement. It does not target a scalar invariant or prove global maximality.

## Choice, alternatives and rubric criticism

- Fresh small vertex cuts of perturbed HKO factors: strong upper-tail prior from proximity, fresh label-free geometry, crosses five-to-six facets, cheap exact full-pair truth; selected. Novelty alone could reward an irrelevant extra facet, so verify the new facets are genuine and report actual ratio independently.
- Full HKO chamber theorem: potentially strongest insight, but ten tied maximizing pairs and LP chambers make a complete proof implausible inside this cap. A proof-only rubric could reward an unfinished conjecture; do not select it here.
- More fresh 5x5 scans: cheap and tail relevant but 44 previous fresh agreements plus one established local theorem make agreement low value. An agreement-count rubric rewards easy redundant cases; reject.
- Existing exposed 6x6 labels: cheap and combinatorially relevant, but retrospective selection cannot test the intended fresh challenge. A largest-error rubric risks label selection; reject.

Primary rating: a structural strict extractor obstruction above one is meaningful; all agreement without a mechanism is not. Failure below one is a useful limitation but does not answer the upper-tail objective. No tiny numerical improvement counts as a result.

## Geometry frozen before any oracle truth

Read exact HKO normal coordinates from the existing conditional-coupling fixture and substitute sqrt(5)=2236068/1000000, exactly as a rational construction, not an exact algebraic HKO claim. For seeds 0,...,5, perturb all twenty coordinates by eps=1/100 times the first twenty SHA256 bytes transformed to (byte mod 9 - 4)/4, using the fresh namespace `msc-math/thesis/facet-transfer/v1:`. These six base products are not selected by ratio/capacity.

Sort each factor's five normals by the existing exact angular rule. For A choose adjacent angular positions seed mod 5 and successor; for B choose (2*seed+1) mod 5 and successor. For each delta in [1/1000,1/100], append the new normal (u+v)/(2*(1-delta)) to its factor. At their old primal intersection this inequality is violated strictly, cutting that primal vertex. Ordering is seed-major, cut-depth-minor, 12 inputs total. Keep original input normal order and append index 5; only the two selected normals are angularly adjacent. Record provenance and verify by exact halfspace vertex enumeration that all six inequalities define genuine facets, factors are bounded convex polygons with origin strictly inside, and the new cut does not remove another facet. Geometry failure stops rather than replaces an input.

## Stages and why each cannot be skipped

1. Freeze all 12 rational geometries, construction/code/source hashes, and exact geometry checks. Prevents selection by labels and excludes a fictitious sixth facet.
2. Predict all 12 with the unchanged mathematical rule: two frozen angular shadow words; exact-certified conditional LP; every nonzero row and column seed; two initial opposite best responses followed by strict coordinate ascent; first-max closure enumeration ties retained. Existing hardcoded 5x5 slicing is generalized mechanically to n,m only. No support or optimizer tuning, baseline fallback, or target-dependent choice. Freeze prediction hashes and send root before oracle. Prevents truth leakage.
3. Exact full closure-pair enumeration in both words supplies oracle truth only after all predictions. Independently verify direct 4D HK sum, LP primal/dual, geometry, responses, capacity/volume ratio. Stop scoring at the first strict miss; do not expand or tune. Actual ratio>1 is scored, never assumed from intended near-HKO placement.
4. On a strict miss, inspect finite response basin/LP stability and winning-support mechanism, retaining only a materially useful certificate. Otherwise report finite agreement with no new mechanism and stop. No theorem inferred from agreement.

Budget: at most 12 new inputs, at most 25 wall minutes and 180 cumulative scientific CPU seconds, no network, installation, cloud, expansion or manuscript changes. Prediction/oracle/check CPU receipts share this cap. Root checkpoint GO is required before constructing or scoring. Files remain in `/tmp/msc-math/facet-transfer/20261017T2012Z/` until root's deliberate retention; root owns commits/cleanup. This is a constructed local combinatorial challenge, not a broad-population holdout or a simple scalar invariant.
