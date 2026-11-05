# Partial independent replay record

29 October 2026. Reviewer `/root/hko_certificate/audit`.

**Incomplete; no capacity certificate or ratio claim is independently
confirmed.** At the coordinator's quota-zero wrap-up instruction, I stopped my
fresh replay at the exact-curvature stage. The replay directory is
`/tmp/msc-math/viterbo-continuation-20261029/two-parameter/independent-replay/`;
it and its progress logs are retained.

## Completed checks

The final `replay.sh` launched successfully in the fresh directory. The exact
eight-label attainer script passed: the candidate word
`(0,6,5,2,19,11,8,4)` has positive exact weights and

```text
Q = tau (5328025872481 - 3162037824 tau^2) / 26435588952481.
```

The exact geometry script passed before and after correction of its coarse
inclusion metadata. It reconstructed 75 simple vertices, 120 graph edges and
20 genuine facets; both Lawrence evaluations agreed on

```text
V = 12188906067413/2122975008000
    - (7700705900357/20264761440000) tau^2.
```

All original rows have support 1 on `H`; all moving rows have support
`571/500 = a+t`. This verifies `H/(571/500) subset K` and supplies the stated
singular-family coarse bound. The corrected geometry source and its second
receipt are in the fresh replay directory. The earlier geometry receipt there
has the same geometric values but stale explanatory text; the corrected
receipt supersedes it.

The exact curvature process rechecked 61,000 of the 99,085 proposed words
before it was stopped. None of those 61,000 failed the positive-curvature
predicate. This partial prefix is not a complete exclusion list. The producer
writes its accepted and rejected sets only at the end, so no complete
curvature receipt or accepted bank was produced.

I read the current `COVERAGE-PROOF.md`, `common.py`, curvature extension,
suffix-aware DFS, KKT checker and final verifier. The exact body definition,
support bound, inherited zero-padding argument, lower-dimensional singular
feasibility argument and scalar benchmark caveat are mathematically
consistent. The benchmark comparison is a statement about this one fixed
body; it does not prove a family-wide envelope. These code/proof observations
do not fill the unrun predicates below.

## Unfinished and unresolved

The independent replay did not finish the 99,085-word curvature pass. It did
not run descendant curvature checks, cycle generation or the separate
coverage traversal, exact leaf KKT checks, or `verify_result.py`. Therefore
there is no independent confirmation of the candidate's global capacity,
the ratio, or the exact comparison with `1.052432`. No leaf-count or runtime
estimate was established for this body.

The same two-parameter curvature producer was still active under its
production owner when I stopped my own replay; its progress log showed 69,000
of 99,085 checks. I did not stop or alter that separately owned process.
Production completion and its final assembled proof review remain pending.

## Resume route

The partial directory has no checkpoint of completed per-word curvature
predicates. Rerunning `exact_curvature.py` in place would start the entire pass
again. To obtain a clean, complete independent reproduction after work resumes,
use a new destination with the final packet script:

```bash
SAGE_PYTHON=/workspaces/msc-math/.local-environments/miniforge/envs/sage/bin/python \
  bash replay.sh /tmp/msc-math/viterbo-continuation-20261029/two-parameter/replay-after-quota
```

Do not treat the existing partial replay directory as a completed certificate.
