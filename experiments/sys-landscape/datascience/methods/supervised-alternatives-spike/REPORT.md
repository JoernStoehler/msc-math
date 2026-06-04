# DS-I005 Supervised Alternatives Spike

## Command And Provenance

Historical note: this report records an older worker run that used a temporary
dataset path. Current reruns should use
`experiments/sys-landscape/datascience/dataset/` and should not recreate this
private `/tmp` dataset.

- historical command: `uv run --script experiments/sys-landscape/datascience/methods/supervised-alternatives-spike/analyze.py --dataset-dir /tmp/sys-ds-pilot1-tables-tH33Hr --permutations 20`
- current rerun command: `uv run --script experiments/sys-landscape/datascience/methods/supervised-alternatives-spike/analyze.py --dataset-dir experiments/sys-landscape/datascience/dataset --permutations 20`
- historical dataset path: `/tmp/sys-ds-pilot1-tables-tH33Hr`
- historical producer command: `cargo run -p exp-sys-landscape --bin sys-dataset -- --out-dir /tmp/sys-ds-pilot1-tables-tH33Hr`
- generated at UTC: `2026-04-30T15:47:56.754035+00:00`

## Dataset Snapshot And Guards

- polytope rows: `282`
- observation rows: `282`
- max `sys`: `0.906316153431123`
- `sys > 1` count: `0`
- guard status: `passed`
- random rows/groups: `170` / `170`
- endpoint rows/groups: `112` / `42`

## Method

Observation: the claim-bearing matrix uses numeric scalar columns from `polytope-table.jsonl`.
It excludes `sys`, capacity, raw vertex/sigma arrays, ids, and observation provenance.
The `metadata_caveat` block is reported only as a provenance comparison, not as
a geometry-based candidate-proposer.

- regression panel: `elastic_net, extra_trees, hist_gradient_boosting, knn, lasso`
- classification panel: `extra_trees, knn`
- claim-bearing feature counts: `intrinsic_no_orbit_search=113`, `intrinsic_numeric=123`
- split policy: grouped CV by root/lineage/source fields; random-to-endpoint
  prediction trains on random rows and scores endpoint rows.

## Observations

Best claim-bearing regression surfaces:

- random-to-endpoint: `hist_gradient_boosting` on `intrinsic_no_orbit_search` with `R^2=-2.8894`, `RMSE=0.2392`
- within-random: `extra_trees` on `intrinsic_no_orbit_search` with `R^2=0.8995`, `RMSE=0.0651`
- within-endpoint: `extra_trees` on `intrinsic_no_orbit_search` with `R^2=0.3268`, `RMSE=0.0995`

Top regression rows:

| surface | model | feature block | r2 | secondary |
| --- | --- | --- | ---: | ---: |
| `random_to_endpoint` | `hist_gradient_boosting` | `intrinsic_no_orbit_search` | -2.8894 | 0.2392 |
| `random_to_endpoint` | `extra_trees` | `intrinsic_no_orbit_search` | -3.3021 | 0.2516 |
| `random_to_endpoint` | `hist_gradient_boosting` | `intrinsic_numeric` | -3.4757 | 0.2566 |
| `random_to_endpoint` | `extra_trees` | `intrinsic_numeric` | -3.5980 | 0.2601 |
| `random_to_endpoint` | `elastic_net` | `intrinsic_no_orbit_search` | -10.3592 | 0.4089 |
| `random_to_endpoint` | `lasso` | `intrinsic_no_orbit_search` | -10.8303 | 0.4172 |
| `within_random` | `extra_trees` | `intrinsic_no_orbit_search` | 0.8995 | 0.0651 |
| `within_random` | `extra_trees` | `intrinsic_numeric` | 0.8991 | 0.0652 |
| `within_random` | `hist_gradient_boosting` | `intrinsic_no_orbit_search` | 0.8857 | 0.0694 |
| `within_random` | `hist_gradient_boosting` | `intrinsic_numeric` | 0.8851 | 0.0696 |
| `within_random` | `elastic_net` | `intrinsic_no_orbit_search` | 0.6344 | 0.1242 |
| `within_random` | `lasso` | `intrinsic_no_orbit_search` | 0.6053 | 0.1290 |
| `within_endpoint` | `extra_trees` | `intrinsic_no_orbit_search` | 0.3268 | 0.0995 |
| `within_endpoint` | `extra_trees` | `intrinsic_numeric` | 0.3128 | 0.1006 |
| `within_endpoint` | `hist_gradient_boosting` | `intrinsic_no_orbit_search` | 0.2619 | 0.1042 |
| `within_endpoint` | `hist_gradient_boosting` | `intrinsic_numeric` | 0.2619 | 0.1042 |
| `within_endpoint` | `knn` | `intrinsic_no_orbit_search` | 0.2195 | 0.1072 |
| `within_endpoint` | `knn` | `intrinsic_numeric` | 0.1870 | 0.1094 |

Random-to-endpoint permutation null for claim-bearing blocks:

| model | feature block | real R^2 | permuted p05 | permuted median | permuted p95 |
| --- | --- | ---: | ---: | ---: | ---: |
| `lasso` | `intrinsic_no_orbit_search` | -10.8303 | -30.0188 | -23.3379 | -15.0676 |
| `lasso` | `intrinsic_numeric` | -33.1484 | -78.2555 | -24.5474 | -11.2494 |
| `elastic_net` | `intrinsic_no_orbit_search` | -10.3592 | -35.2534 | -24.0411 | -14.8933 |
| `elastic_net` | `intrinsic_numeric` | -46.2632 | -149.6961 | -27.0925 | -4.9557 |
| `hist_gradient_boosting` | `intrinsic_no_orbit_search` | -2.8894 | -21.3139 | -16.3893 | -14.0346 |
| `hist_gradient_boosting` | `intrinsic_numeric` | -3.4757 | -20.6325 | -17.2550 | -14.0226 |
| `extra_trees` | `intrinsic_no_orbit_search` | -3.3021 | -20.0110 | -17.1348 | -14.4440 |
| `extra_trees` | `intrinsic_numeric` | -3.5980 | -20.2309 | -17.4335 | -14.5677 |
| `knn` | `intrinsic_no_orbit_search` | -11.2707 | -21.6808 | -18.9428 | -14.9786 |
| `knn` | `intrinsic_numeric` | -13.1432 | -21.7738 | -19.4518 | -14.9756 |

Endpoint-vs-random classification:

- best claim-bearing classifier: `extra_trees` on `intrinsic_numeric` with balanced accuracy `0.9451` and ROC AUC `0.9931`.

| surface | model | feature block | balanced_accuracy | secondary |
| --- | --- | --- | ---: | ---: |
| `endpoint_vs_random` | `extra_trees` | `metadata_caveat` | 1.0000 | 1.0000 |
| `endpoint_vs_random` | `knn` | `metadata_caveat` | 1.0000 | 1.0000 |
| `endpoint_vs_random` | `extra_trees` | `intrinsic_numeric` | 0.9451 | 0.9931 |
| `endpoint_vs_random` | `extra_trees` | `intrinsic_no_orbit_search` | 0.9364 | 0.9886 |
| `endpoint_vs_random` | `knn` | `intrinsic_numeric` | 0.9081 | 0.9736 |
| `endpoint_vs_random` | `knn` | `intrinsic_no_orbit_search` | 0.9010 | 0.9681 |

## Inference

The cheap supervised alternatives do not change the M011 no-candidate-proposer story under the load-bearing random-to-endpoint prediction surface. The best claim-bearing random-to-endpoint `R^2` remains negative, even when flexible tree and kNN alternatives are allowed. Within-random and within-endpoint fits can be positive, but they do not give positive random-to-endpoint prediction.

For the M012-style endpoint-vs-random question, non-provenance numeric polytope features can still separate endpoint rows and random rows. That is a table observation, not a rule for proposing new high-`sys` candidates; the metadata/provenance baseline is kept only as a caveat comparison.

## Verdict

- verdict: `no-search-output`
- evidence_strength: `medium`
- implementation_trust: `medium`
- thesis_use: `supporting/caveat only`
- caveat: Current 282-row dataset only; feature matrix excludes target/capacity, raw arrays, ids, and observation provenance for claim-bearing blocks; the `intrinsic_numeric` block still includes cached orbit-search scalar features, so `intrinsic_no_orbit_search` is the cleaner geometry-side sensitivity. The method panel is small and cheap.
- reopen trigger: Reopen if refreshed tables add materially more random/endpoint
  rows, if new non-provenance features make random-to-endpoint R^2 nonnegative
  under grouped prediction, or if a reviewer finds a leakage bug in the feature
  exclusion or group policy.
