# Exact twenty-facet HKO improvement at t = 7/20

The complete production certificate passes. Independent staged fresh-copy
replay and assembled mathematical audit are recorded in `AUDIT.md` and
`REVIEW-independent-replay.md`; consult those records for their final status.
This is a new fixed-body result, outside the selected thesis.

For `tau=tan(pi/5)`, the root in `(0,1)` of `tau^4-10tau^2+5`, the body
`K=H_(7/20)` has 20 genuine facets, 75 simple vertices, and

```text
Qmax = tau (8456329321 - 4892944 tau^2) / 41964992141
c(K) = 41964992141 / [2 tau (8456329321 - 4892944 tau^2)]
V(K) = 6644112217/1156332480 - 2199353203 tau^2/5781662400
rho(K) = 1.052345598871062...
rho(K) - rho(H_(3/10)) = 0.00043146004157479465...
rho(K) - rho(H) = 0.0051320033711041175...
```

The exact attaining word is `(0,6,5,2,19,11,8,4)` with strictly positive
weights stored in `exact-attainer.json`. Exact real-algebraic comparisons
prove both improvements; the displayed decimals are descriptive. The exact
ratio is retained in `verification-summary.json`. No optimization of the
whole family or global record is asserted.

`COVERAGE-PROOF.md` gives the full implication from the published geometric
simple-minimizer theorem, exact geometry, curvature exclusions, exhaustive
cycle coverage and all stationary families to this scalar capacity.
`GEOMETRY.md` records the Lawrence volume formula and the independently
checked inclusion `H/(57/50) subset K`.

## Complete finite checks

All 77,927 old exclusion proposals were rechecked at the new parameter:
76,624 passed and 1,303 were discarded. Two exact descendant rounds added
22,461 exclusions, for 99,085 total. Fresh integer DFS checks the canonical
full prefix and each contiguous suffix against that bank. It visits 348,716
prefixes and 52,306 surviving cycles, represented by 2,839 canonical leaves
of maximum length 14. A separate traversal finds zero unknown cycles.

Every leaf is checked exactly. All 1,151 consistent singular KKT families
are resolved: 1,087 have constant objective below the attained target,
33 exceed the proved coarse inclusion bound and hence are infeasible, and
31 have empty parameter polyhedra by exact vertex enumeration. There is no
feasible above-target singular family. The complete per-leaf accounting is
`exact-leaves-detail.json`.

Measured producer times: old proposal recheck 491.7 s, descendant checks
204.0 s, leaf check 127.4 s, geometry 8.5 s; integer traversals take seconds.
These are observations under concurrent host load, not future runtime bounds.
The prefix-only diagnostic would have left 163,389 leaves; its receipt is
retained as `generation-two-round-prefix-only.json` and is not an acceptance
result. The suffix test uses the same zero-padding curvature lemma and is
what makes the completed certificate inexpensive.

## Fresh-directory replay

Prerequisites: SageMath Python and a C++17 compiler. From this packet:

```bash
SAGE_PYTHON=/workspaces/msc-math/.local-environments/miniforge/envs/sage/bin/python \
  bash replay.sh /tmp/msc-math/hko-t035-fresh-replay
```

The destination must not already exist. The script copies only source and
proposal inputs, then reconstructs every output in the new directory. It
runs exact attainers, geometry/inclusion, all old proposal curvature checks,
two exact descendant rounds, fresh complete cycle generation, all exact leaf
checks, a separate cycle coverage replay and the final receipt verifier.
Frozen outputs in this packet are never overwritten by this command.

`proposed-exclusions.txt` is only the old proposal bank. `base-exclusions.txt`
and `extension-exclusions.txt` contain newly verified positive-curvature
words; their union is `exclusions.txt`. The old leaf set is not an input.
The final verifier cross-checks receipts and exact values; running it alone
does not replace the underlying predicates. The first independent replay
was staged while the extension/suffix improvements were introduced; its
review states which source versions and complete predicate sets were run.
