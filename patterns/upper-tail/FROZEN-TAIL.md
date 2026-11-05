# Frozen triangle selection improves width, but leaves material upper-tail gaps

The frozen exact minimum-area triangle rule was evaluated on all **3,072 retained 5×5, 5×6 and 6×6 rows**. It improves the numerical width ceiling on **484/3,072** rows. The worst capacity gap in each bucket's top 2% is **9.90%, 12.74%, and 6.61%**, respectively, after taking the better of triangle and width. The selected triangle alone can miss capacity by **56.14%** even in the 5×5 top 2%. **Stop this fixed rule as a uniformly tight upper-tail explanation; no supports or fitted choices were added.**

The earlier exact rejection of 25/26 width false positives remains valid. This expanded diagnostic answers a different question: those rejection counts do not establish that the small selected support explains the actual highest historical ratios. No theorem or predictor is inferred from the retrospective gaps.

## Frozen choices, alternatives and rubric

[TAIL-PLAN.md](TAIL-PLAN.md) was written before expanded execution. The useful end state was a tight small-support upper-tail explanation, or explicit failures sufficient to stop. Source and selector identity, full population coverage, tail-specific comparison and independent feasible-word checks cannot be skipped: they prevent label changes, survivor-selection bias, bulk-score masking and unsupported exactness claims.

Width alone is the baseline; full sparse-support enumeration would merely rerun the existing capacity algorithm; fitting the support choice risks hiding target dependence. The selected option kept the existing rule: enumerate bounded circumscribed triangles from original halfplanes, keep **all exact area minimizers**, and compare all 120 cyclic orders for each selected pair. Every retained row has exactly one minimizing triangle per factor. The same separate numerical HKO control is preserved. No alternative supports were tested after inspecting failures.

A rejection-count rubric rewards already easy low-width rows; a bulk mean hides the hardest tail. The predeclared rubric therefore reports distributions and counts within 1%, 5% and 10% capacity gap in every bucket's top 10/5/2%. A top-2% gap of at least 10% was designated a clear failure of a uniformly tight (within 10%) reading before execution. The 5×6 example triggers that stop. These scales describe this bounded diagnostic, not a universal scientific tolerance.

## Capacity-gap distributions

Here $U=\min(C_T,A_2)$, with $C_T$ the exact selected-triangle bound and $A_2$ the retained numerical width ceiling. Gaps below are $100(U/c_{\rm historical}-1)$, in percent. Tails use descending **historical numerical systolic ratio**, ties by ascending name, and $\lceil1024f\rceil$ rows. Quantiles use linear interpolation. The all-row [summary](tail-summary.json) includes minimum, 10th percentile, median, 90th/95th percentile, maximum and named worst rows for all three bound variants.

| Bucket | Tail (n) | U gap median | U gap 90th | U gap max | Width gap median | Width gap max | Within 1% |
|---|---:|---:|---:|---:|---:|---:|---:|
| 5x5 | top10% (103) | 1.92% | 7.23% | 14.79% | 4.24% | 22.55% | 47/103 |
| 5x5 | top5% (52) | 2.18% | 7.44% | 14.79% | 4.26% | 16.32% | 21/52 |
| 5x5 | top2% (21) | 3.56% | 7.83% | 9.90% | 7.55% | 12.79% | 8/21 |
| 5x6 | top10% (103) | 1.56% | 8.70% | 14.10% | 3.78% | 19.35% | 38/103 |
| 5x6 | top5% (52) | 2.08% | 8.52% | 12.74% | 5.68% | 19.35% | 18/52 |
| 5x6 | top2% (21) | 1.89% | 8.63% | 12.74% | 4.62% | 17.82% | 9/21 |
| 6x6 | top10% (103) | 0.69% | 6.05% | 10.60% | 1.82% | 17.10% | 59/103 |
| 6x6 | top5% (52) | 0.93% | 6.26% | 7.56% | 2.09% | 14.19% | 28/52 |
| 6x6 | top2% (21) | 2.50% | 6.27% | 6.61% | 3.23% | 8.14% | 8/21 |

The top-2% triangle rule improves width on 11/21, 8/21 and 6/21 rows. It cuts median gaps materially in the first two buckets, but does not deliver uniformly tight values. Among all rows, median U/c is essentially one; that bulk fact would conceal the tail failures.

## Concrete support failures

“Failure” here means a selected-support capacity upper bound remains above the inherited numerical capacity. It does not mean the selected word is infeasible. No unselected support was optimized to explain the discrepancy, and the historical source has a bounce count but no minimizing support word.

| Worst U/c in top 2% | Historical ratio | U/c | Width/c | Triangle/c | Selected factor indices | Feasible triangle counts |
|---|---:|---:|---:|---:|---|---|
| `random_5x5_s1_64` | 0.76426850 | 1.099046 | 1.099046 | 1.106452 | [[0, 2, 4], [0, 2, 3]] | [4, 5] |
| `random_5x6_s2_150` | 0.79466147 | 1.127423 | 1.129840 | 1.127423 | [[1, 2, 4], [0, 1, 3]] | [4, 6] |
| `random_6x6_s3_124` | 0.79404373 | 1.066064 | 1.073965 | 1.066064 | [[0, 2, 4], [0, 2, 4]] | [8, 6] |

All three named rows have inherited bounce count 3. For `random_5x5_s1_64`, width is still better than the chosen triangle pair; both miss capacity by about 10%. For `random_5x6_s2_150`, the chosen pair improves width only slightly and retains the previous packet's lone exact above-one survivor. Its inherited ratio is 0.79466147, while the exact selected-triangle ratio bound is 1.01008014. The gap is therefore consequential for screening, not numerical polishing.

The sharpest triangle-only top-2% failure is `random_5x5_s1_221`: historical ratio 0.80494495, triangle/c=1.561381, width/c=1.080890. Its unique selected supports are [0,2,3] in both factors, from 4 and 3 feasible triangles. Minimizing the two outer areas separately therefore need not select a capacity-tight pair. Exhausting the remaining pairs would return toward the full algorithm and is outside this frozen test.

Across each complete bucket, the worst U/c values are 1.154728 (`random_5x5_s3_114`), 1.231222 (`random_5x6_s3_29`) and 1.185842 (`random_6x6_s2_3`). The [row archive](tail-rows.jsonl.gz) retains every exact feasible-word witness, selected support, historical comparison and original rational area product, including these failures.

## Verification, source and reproduction

The original cache was verified offline by `python3 scripts/artifacts.py verify-cache polytope-datasets`: three files, 252,398,018 bytes, registered snapshot `f7bc6be841e9d30741d5bf7ec8d4f0c0c74ec22745d3325b792e64cec333ca96`. The original JSONL SHA-256 is `66bf82010e92e0f26b0df226f4e6c0eef05d21eb22a0967c7f669530f6545736`, width gzip `dcffead16ab5ad0fcdf6ae55d78375bea5b6ee7274877a33846cf4c3c2a7d57f`, and frozen `triangle_envelope.py` `03e0a9459cf40696b57d0f2119779d06274c9232eef02c04b28ce5a0e2323704`. All 10,240 source names, side counts, capacity and ratio labels join exactly to the width feature table. The eligible buckets each have 1,024 rows.

The expanded selected-support pass took **21.24 seconds**, below the 300-second initial wall cap, evaluating **368,640** exact rational cyclic-word objectives. The separate HKO numerical control returns selected-triangle ratio 1.047213595499958, matching the imported known expression to rounding. It is not included in the rational-body count.

[verify_frozen_tail.py](verify_frozen_tail.py) independently reconstructs every original area using the earlier verifier's all-pairs feasible intersections and rational convex hull, checks positive normalized closure, evaluates each retained word without optimization, and checks its exact ratio bound. It reports **PASS for all 3,072 rational witnesses** in 7.35 seconds; all 26 earlier exact ratios, selected supports and certificates match exactly as JSON values. It also independently checks tail membership, extrema and 1/5/10% gap counts. It does not independently prove area-selector optimality; the frozen selector identity and reuse establish what was executed. [tail-verification.json](tail-verification.json) records the scope.

```sh
python3 scripts/artifacts.py verify-cache polytope-datasets
python3 patterns/upper-tail/frozen_tail.py \
  --source /path/to/verified-cache/files/random-product.jsonl \
  --width patterns/product-shape/features.csv.gz \
  --out /tmp/msc-math/triangle-tail/replay --max-seconds 300
python3 patterns/upper-tail/verify_frozen_tail.py \
  --source /path/to/verified-cache/files/random-product.jsonl \
  --rows /tmp/msc-math/triangle-tail/replay/tail-rows.jsonl.gz \
  --summary /tmp/msc-math/triangle-tail/replay/tail-summary.json \
  --prior patterns/upper-tail/triangle-envelope.json \
  --out /tmp/msc-math/triangle-tail/replay/tail-verification.json
```

The earlier `verification.json` binds the original 26-row packet before the new completion paragraph in `REPORT.md`; its old report hash is historical. Exact witnesses remain unchanged. The current [tail manifest](tail-manifest.json) binds this expanded packet.

Standard library only. New scripts and outputs began under `/tmp/msc-math/thesis-owner/2026-10-17-batch2/triangle-tail/` and were deliberately retained here. The producer writes only its output directory, with a streamed partial row archive if the initial cap fails. Compressed bytes and elapsed times are observational; no cross-run byte reproducibility of gzip metadata is promised.

Exact arithmetic certifies the feasible selected-word upper bound on the rational input body. Historical capacity/ratio labels, width values and comparisons to them remain numerical; the minimum U is not an independently certified combined bound in this packet. No original-body full capacity or volume producer, additional-body search, fitting, cloud job or network operation was run. This is a focused finite diagnostic, not prospective prediction, a theorem of global maximality or an exhaustive literature survey.

[review: /root/batch_review]
