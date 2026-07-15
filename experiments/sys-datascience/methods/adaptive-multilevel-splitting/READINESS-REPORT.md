# Production readiness smoke

This is an operational driver/policy result, not an adaptive-versus-IID
comparison. It supports no superiority, scientific-negative, or rare-event
probability claim.

## Reviewed launch identity

- source commit: `8c63f94f72d4af7007a04ea49f74e252c4959017`;
- release executable SHA-256:
  `cf4719a269ad7bdfb970825ac3dd09a49287e2c1d2760bba666668d8b659a55a`;
- packet `Cargo.lock` SHA-256:
  `04e9dbe2a35070ae5472db3c958714d9a4dea309df1718f712afe87052388c4d`;
- artifact:
  `/tmp/sys-ds-research-lines/adaptive-hostile-search/readiness-production-8c63f94f`.

Fresh producer/process and analyzer/evidence reviews independently returned
`GO` for that exact source/executable pair before exposure. Both reviews ran
zero production targets. The producer then completed, and `analyze.py`
independently returned `verified: true` and `readiness_passed: true`.

## Result

| Check | Observed |
|---|---:|
| Charged adaptive / IID requests | 48 / 16 |
| Durable ledger / target rows | 64 / 64 |
| Mutation transitions / completed levels | 32 / 2 |
| Construction rejections | 138 |
| Failed or outcome-unknown requests | 0 |
| Cache hits | 0 |
| Post-level distinct states | 14, 15 |
| Accepted mutations | 12/16, 13/16 |
| Total monotonic wall time | 36.681 s |
| Sum of target wall times | 24.849 s |
| Non-target overhead | 11.833 s |
| Largest observed `sys` | 0.7920699999868354 |
| `sys > 1` | none |

The adaptive maximum was `0.7920699999868354`; the IID maximum was
`0.7274406339454365`. These small, adaptively related arms were sized only for
readiness, so their difference is calibration and cannot be interpreted as
scientific evidence.

Target timings in seconds were:

| Stage | n | Mean | Median | Min | Max | Sum |
|---|---:|---:|---:|---:|---:|---:|
| Adaptive initial | 16 | 0.422865 | 0.409424 | 0.357692 | 0.610637 | 6.765844 |
| Adaptive level 0 mutation | 16 | 0.372572 | 0.363835 | 0.349355 | 0.414491 | 5.961159 |
| Adaptive level 1 mutation | 16 | 0.371567 | 0.366716 | 0.347009 | 0.420579 | 5.945075 |
| IID | 16 | 0.386028 | 0.374079 | 0.332829 | 0.464431 | 6.176446 |

Construction/artifact gaps assigned to the next request averaged 0.264659 s,
0.149410 s, 0.124150 s, and 0.199836 s in the same stage order. Construction
rejections were respectively 71, 8, 1, and 58.

## Accounting history

Before this valid smoke, a defective clean-tree refusal test accidentally
launched eight production children. Seven results were retained in the
preserved partial artifact, the eighth was orphaned and later exited, and none
of the retained rows had `sys > 1`. Those eight launches are charged as
readiness work but are not a valid smoke and provide no adaptive-versus-IID
evidence. Total readiness exposure is therefore 72 target launches: eight
accidental plus 64 valid. Scientific-comparison exposure remains zero.

The accidental artifact is preserved at
`/tmp/sys-ds-research-lines/adaptive-hostile-search/accidental-clean-test-production/`
with its adjacent SHA-256 manifest.

## Limits

The analyzer does not reimplement the capacity/KKT target or the full IID
constructor. A future `sys > 1` requires independent target validation. The
reviewed executable identity is build-specific, and any rebuild or source
change invalidates its launch review.
