# Product-bounce mechanism packet review

Review date: 2026-07-14.

Verdict: ready for the first mechanism-design discussion, not for a causal or
resampling claim.

A fresh Sol-low structured review inspected the analyzer, generated summary,
reviewed source contracts, and selected raw rows. It checked:

- the four fixed input identities and raw/class/prepared join route;
- the common fixed-effects design used for `log(sys)`, `2 log(capacity)`, and
  `-log(2 volume)` and the resulting additive coefficient identity;
- the complete-class overlap definition;
- recovery of support heights and angular gaps from q-first/p-second stored
  dual rows;
- the distinction between volume-free ridge distribution ratios and
  volume-normalized ridge-magnitude sensitivities;
- all winning minimizer representatives, A3-null exclusions, and the retained
  eight-facet A3 stratum;
- whether the README claims stayed within the generated evidence.

The review found two repairable defects.

1. The first interpretation inferred equal effective conditional-fibre
   dimension from equal active-facet counts. The packet now stops at the
   supported count statement and keeps constraint rank/effective dimension as
   an open resampling-design check.
2. The README claimed duplicate raw/class names would abort, but the analyzer
   silently built maps. The analyzer now checks raw and class name uniqueness,
   equality of their name sets, and joined `(k,m)`, label, capacity, volume, and
   `sys` fields before analysis.

Independent parsing confirmed that all 22,175 winning minimizer word
representatives have six distinct facets split `3q+3p`; within every row the
winning representatives also share one unordered support. The 321 rows with an
eight-facet A3 minimizer are all two-bounce global winners. The current fixed-
hash artifact was not affected by the join-guard defect. Regeneration after the
repairs is byte-identical to the retained summary, and the coefficient additivity
residuals remain at floating-point roundoff (`4e-14` to `6e-14`).

No other material finding remains for the named discussion use. A later active-
facet resampling packet still needs its own conditional-law contract, smoke
evidence, and technical/mathematical transition review.
