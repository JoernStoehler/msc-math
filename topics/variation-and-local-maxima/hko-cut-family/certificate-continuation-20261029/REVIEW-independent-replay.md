# Independent replay and mathematical audit

29 October 2026. Reviewer `/root/hko_certificate/audit`.

**PASS for the assembled proof and every finite acceptance predicate in a
staged fresh-directory replay.** The replay was deliberately staged while
the production route was being reduced from an impractical prefix-only leaf
set. It ran in
`/tmp/msc-math/viterbo-continuation-20261029/certificate/independent-replay/`;
it did not overwrite production outputs. All 2,839 final leaf representatives
were independently checked, and the final receipt verifier passed.

## Replay route and independent comparisons

I started with the documented `replay.sh` in a destination that did not
exist. Its initial syntax check found an apostrophe in the `${...:?word}`
error text; the owner removed it, after which `bash -n replay.sh` and the
script launch succeeded. I ran the exact attainer, exact geometry, all 77,927
proposed curvature predicates, and retained that fresh base-exclusion output.
The replay shell was paused before its then-current prefix-only cycle
generation so that it would not perform the known 163,389-leaf KKT run. After
the owner added the checked descendant bank and suffix-aware pruning, I
continued inside that same fresh directory: exact-checked both descendant
rounds, compiled the updated C++ source, regenerated cycle coverage and leaves,
checked all leaves, and ran the final verifier. The only difference in the
curvature checker source used for the first stage is its output filename
(`exclusions.txt` in the initial copy versus `base-exclusions.txt` in the
final source); the exact predicate and proposal input are identical. The
fresh output was preserved under its final `base-exclusions.txt` name before
the extension stage.

The independent base pass classified 76,624 proposals as positive-curvature
exclusions and discarded 1,303. The descendant helper proposed and checked
18,204 words in round one and 5,567 in round two; respectively 17,225 and
5,236 passed. The final set contains 99,085 exact exclusions. The updated
integer search generated 2,839 leaves from 348,716 prefixes and 52,306
surviving cycles, with maximum word length 14. A separate coverage traversal
returned `verified=true`, `unknown_cycles=0`.

The exact leaf checker passed on all 2,839 words. Its 1,151 consistent
singular KKT systems partition as 1,087 constants at or below the attained
target, 33 constants above the exact inclusion bound, and 31 infeasible by
complete parameter-polytope vertex enumeration. No feasible above-target
singular family remains. The receipt verifier returned
`verified_finite_predicates=true` and reproduced

```text
Qmax = tau (8456329321 - 4892944 tau^2) / 41964992141
V    = 6644112217/1156332480 - 2199353203 tau^2/5781662400
rho = 1.052345598871062...
rho - rho(H_(3/10)) = 0.00043146004157479465...
```

The ratio improvements are exact real-algebraic sign checks; the decimals are
descriptive. This establishes the claimed fixed-body capacity and volume
result, subject to the separately cited Haim–Kislev simple-minimizer theorem
and Lawrence volume formula.

I compared the fresh replay with the production packet. These files were
byte-identical: `common.py`, attainer and geometry source, descendant helper,
leaf checker, final verifier, suffix-aware C++ source, original proposal bank,
graph, base/final exclusions, descendant proposal/acceptance lists, leaves,
attainer receipt, per-leaf detail, coverage and generation summaries, and the
final verification summary. The exact curvature source differs only in the
output filename described above. Geometry, curvature, descendant, and leaf
summary JSON differ in elapsed-time fields; the leaf summary consequently
also hashes a geometry summary containing a different elapsed time. Their
mathematical predicates and counts agree exactly. All other shared summary
claims agree.

## Proof audit

The geometric reduction is correctly scoped: the published simple-minimizer
result supplies a capacity-attaining word with distinct facet labels, and
interior transitions imply the directed graph's nonnegative-pairing test.
Exact vertex enumeration proves graph completeness, simplicity and the
genuineness of all twenty inequalities. The exact symmetry action is checked
on all twenty rows and on the complete pairing matrix.

Both pruning mechanisms preserve every possible maximizing positive support.
The exact curvature test uses algebraic signs and congruence, so it has no
floating threshold. New descendant words are only added after the same exact
predicate passes. The C++ search now tests both each full prefix and every
contiguous suffix of sufficient length. If a positive-curvature block occurs
internally, it was a suffix when its final label was appended; zero-padding
then gives the same obstruction on any larger positive support. Canonical
rotation and reflection preserve the obstruction. The minimum-length bound
omits only lengths absent from the certified bank. Thus the added suffix
pruning does not remove an attaining word, and the no-unknown-cycles replay
checks the resulting leaf set.

The KKT argument covers boundary maxima through their positive active
supports. Independent equality-row reduction preserves the feasible affine
space. For singular systems, the checked kernel identities imply constant
objective; the inclusion bound is converted through capacity monotonicity to
an exact upper bound on `Q`. The remaining nonnegative-weight feasibility
test is complete because normalization and injective kernel projection make
the parameter polytope bounded, including in lower-dimensional cases. Exact
enumeration of all independent active-constraint bases therefore decides its
emptiness.

The volume computation checks all simple vertices and nonzero Lawrence
denominators for two generic functionals; equality of the resulting exact
field values is a useful arithmetic check. Lawrence's formula remains a
separate cited premise. Likewise, the capacity reduction relies on the
published geometric theorem; this finite replay does not re-prove that
theorem.

## Scope limits

The run was staged rather than one uninterrupted invocation of the final
`replay.sh`: the unchanged base curvature stage came from its initial fresh
copy, then the exact descendant and suffix-aware stages were applied to that
preserved directory. I inspected and compared the final replay source, and
all subsequent checkers and receipts were independently executed there. This
is evidence for the predicates and the assembled result, not an independent
uninterrupted execution of the final shell wrapper. The certificate concerns
only `H_(7/20)`; it does not optimize the full one-parameter family or prove a
global extremality statement for other bodies.

Current source SHA-256 values:

```text
common.py                 8c0cefde139cb72132d68132fc0b18425872e25546fcaf0add4b84943747c60c
exact_attainer.py         fc3b96fa414245f9605e46c5c5f16a53ca564f014969bf7024edcd740192d96e
exact_geometry.py         921a13905861c5b708a8e1cc425ea7b777c14ed3e3435498e545cf7870a41d9a
exact_curvature.py        0b9fe7e042adfc760b28b87dcbaa90c6438e5b37a448a298d008696a2b0ec8b5
exact_extend_curvature.py fc05249a1bb12692823d43230408b48ab03cf0f0b99a5800aac8f6abf1ca8926
exact_leaves.py           79cb43073f53d498789ca29c7067e2a9d3cb1f183b6b380a8cad43f9bdb663e2
coverage_replay.cpp       c926ee0facdb2f967e079d844dda6ae45bda6965382a15a4907dd6e40f28c4a2
verify_result.py          a9d1e487af99c813f79b772301324ba4177d33883088826af95f26b7a88cf2c5
replay.sh                 f965fcd330e5bb0d172d71e9c80c7768990f53301d71f6b6f2f16350358e162c
proposed-exclusions.txt    685d59a61c2a1491475f507369a3247297a90e0a275ff006b81037b395158e90
```
