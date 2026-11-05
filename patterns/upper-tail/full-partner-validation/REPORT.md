# Full-partner validation: six substantial tail gains, with an exact width obstruction

17 October 2026. On a frozen sample excluding all 20 design controls, the unchanged full-partner containment selector improves the strongest previous feasible capacity bound by at least 1% on **6 of 27 upper-tail products**. It also loses to that incumbent on seven tail products. On `random_5x6_s1_198`, it is **7.8957% above an existing exact feasible width bound**, which disproves a uniform 5% capacity-accuracy guarantee for this construction independently of historical numerical capacity labels. The mechanism adds useful competing supports; it does not replace the earlier bounds.

The new bound is within 1% of inherited numerical capacity on **22/27 tails**, within 5% on **26/27**, and numerically agrees on **18/27**. The strongest incumbent scores 20/27, 27/27 and 19/27 respectively. All nine hash-selected bulk controls give numerical agreement for the new rule. This is **design-excluded retrospective validation**: these geometries and labels were already scanned by earlier methods. None of these agreement counts establishes equality with true capacity.

## Frozen question, sample and method

The success picture was a compact, target-free geometric selector that stays materially tight on principal upper tails away from its design examples. Source binding, design exclusion, frozen membership and criteria, strongest-incumbent comparison and independent feasible-witness checks cannot be skipped: each rules out an apparent gain from label leakage, weaker baselines or invalid objectives.

[PLAN.md](PLAN.md) compares four sample designs before scoring. A purely hash-selected sample risks rewarding easy low-ratio bodies; all principal top-2% cases would exceed the assigned 48-body cap and lack a bulk contrast; new generation would change population and require a producer. The selected bounded design uses the first nine eligible members of each existing 21-body top-2% list, plus three smallest SHA256(name) controls per family outside its top 103. All 20 prior full-partner controls are excluded. Families are 5×5, 5×6 and 6×6, each originally containing 1,024 bodies. The selected tails have family ranks 4–12, 4–13 and 4–13, with excluded ranks skipped. No population-frequency inference follows from this deliberately tail-heavy sample.

[sample.json](sample.json) records all 36 names, ranks, roles, exclusions and pinned inputs. Its SHA256 is `09a361c3b24d76a052833372a4204b31cee7c8308c3ff31ef67d7e18a9770338`. The frozen PLAN SHA256 is `b2a567e9d1dd755842e1d225f2d0b0fe22c1998375cacc1f7a775935a06a1138`. Both were sent to the receiving owner before scoring. The existing 3,072-body triangle and 10,240-body core/moment scans had exposed these source labels; excluding design controls does not make them unseen data.

The [unchanged source method](../full-partner-containment/REPORT.md) starts from canonical cone-volume closures. It evaluates four variants: either shadow handedness, with either factor responding first. Each response chooses the first maximizing sparse closure vertex, supports of size two then three and lexicographic indices within a size. It responds once in the first factor and once in the other, with no further sweep or word revision. Comparisons and ties on these rational bodies are exact. The new capacity upper bound is

$$C_{\rm full}=\frac1{2\max_{\text{four variants}}Q}.$$

The first response solves the minimum translated homothet problem for the entire rotated partner. Only those prescribed responses select supports; numerical capacity, systolic ratio, prior witnesses and comparator minima never select a new variant or enter the standalone rule.

The primary bucket was all 27 tails within 5% of historical numerical capacity. A declared useful-positive standard also required at least 24/27 within 1% and at least three improvements of at least 1% against the strongest incumbent. The run fails the first two accuracy requirements, while passing the improvement requirement. These are finite empirical decision criteria, not confidence levels. No cutoff, support or rule changed after scoring.

## Strongest available comparison

For every selected body, the exact feasible incumbent is

$$C_{\rm inc}=\min(C_{\rm width},C_{\rm triangle},C_{\rm contact},C_{\rm prior\ response},C_{\rm canonical}).$$

The triangle witness comes from the already retained exact minimum-area supporting-triangle run. The unchanged contact and contact-seeded signed-prefix response methods are evaluated on all 36 bodies. Width is reconstructed exactly from the same factor geometry. The fixed canonical closure with its two shadow words is also retained; the new LP responses dominate it. Contact already dominates width, but width is named and stored to expose the decisive failure below. Retained numerical affine-pentagon core point predictions are available as ancillary error comparators; they are not feasible capacity upper bounds. Earlier targeted challenger witnesses concern excluded design controls. No new complete original-body capacity computation is made, so no exact optimum comparator is available here beyond these feasible bounds.

| Principal tails | New within 1% | Incumbent within 1% | New within 5% | Incumbent within 5% | New gains ≥1% | New losses >1% |
|---|---:|---:|---:|---:|---:|---:|
| 5×5 (9) | 8 | 4 | 9 | 9 | 4 | 0 |
| 5×6 (9) | 7 | 7 | 8 | 9 | 2 | 2 |
| 6×6 (9) | 7 | 9 | 9 | 9 | 0 | 2 |
| All tails (27) | **22** | **20** | **26** | **27** | **6** | **4** |

The accuracy columns compare to inherited numerical capacity; gain/loss columns compare exact feasible bounds. There are seven exact strict tail regressions in total, including three below 1%. Bulk controls add one exact improvement below 1%, which is not counted as a material result. No tail in the selected 6×6 group improves the strongest incumbent, despite gains over weaker width/contact baselines on some bodies.

| Body (prefix `random_`) | Capacity reduction against strongest incumbent |
|---|---:|
| `5x5_s3_6` | 3.4412% |
| `5x5_s3_14` | 4.4571% |
| `5x5_s3_234` | 1.6548% |
| `5x5_s3_54` | 2.6170% |
| `5x6_s2_163` | 1.2788% |
| `5x6_s2_227` | 4.2295% |

Each of these six new feasible bounds numerically agrees with the historical capacity label. That supports finite explanatory value for the relative-factor contact mechanism; the witness alone proves only a smaller capacity upper bound.

As an explicitly **post-score descriptive calculation**, taking the minimum of the unchanged new bound and the incumbent gives 26/27 tails within 1%, all 27 within 5%, 25/27 numerical agreements and a worst historical-label capacity gap of 1.5187%, versus the incumbent’s 4.6650%. This uses no new support evaluation or fitting; it shows complementarity of already retained feasible witnesses. This minimum was not the predeclared standalone selector or a prospectively validated new algorithm. [complementarity.json](complementarity.json) retains the arithmetic and its provenance.

## Exact obstruction: full-partner contact can miss the width competitor

The fourth-ranked 5×6 product `random_5x6_s1_198` is the decisive counterexample to uniform 5% accuracy. Let $C_w$ be its retained contact witness, which equals the independently reconstructed exact width bound. Direct rational comparison gives

$$\frac{C_{\rm full}}{C_w}>1.0789>1.05.$$

Feasibility gives $c_{\rm true}\leq C_w$, hence

$$\frac{C_{\rm full}}{c_{\rm true}}\geq\frac{C_{\rm full}}{C_w}>1.0789.$$

This conclusion needs neither true-capacity equality nor a numerical target. The historical label agrees with $C_w$ numerically, which explains the displayed 7.8957% label-relative miss but is not a premise of the obstruction.

The new selected variant has negative shadow, A first, support A=`[0,2,4]`, support B=`[2,3,5]`. The old contact witness has support A=`[1,2,4]`, B=`[2,4,5]`, restricted word `[0,1,3,2,4,5]`, where restricted labels refer to the concatenated selected supports. [results.json](results.json) stores both exact rational weight lists, capacities and objectives, plus all four new variants. Thus using the entire partner before collapse still need not retain the supports attaining a two-bounce bound. No alternative starting word, seed or extra response was added after this failure.

## Ratio envelope and disposition

For exact original factor areas $A_P,A_Q$, every stored feasible witness yields

$$\operatorname{sys}(P\times_LQ)\leq U_{\rm full}=\frac{C_{\rm full}^2}{2A_PA_Q}.$$

All 36 new bounds are below one and hence below the known HKO value $(3+\sqrt5)/5$. The largest is **0.9347242958**, on the same width-obstruction body. The strongest incumbent already excludes all 36 below one. Therefore the run adds **zero new below-one or HKO-envelope exclusions**. These finite bounds do not approach a global HKO-maximality proof. The previously analyzed regular-pentagon exactness mechanism is imported from the source report; no fresh HKO control or new regular-family result is claimed here.

The bounded validation establishes six substantial new tail bounds and an exact obstruction to using this construction as a universally 5%-accurate replacement. It does not establish a fitted held-out predictor, true-capacity equality, a global bound or human proof acceptance. The four-variant method remains unchanged; this packet stops without tuning or a new scientific programme. The standing owner receives the evidence and retains responsibility for any manuscript selection.

## Evidence, checks and reproduction

[analyze.py](analyze.py) invokes only the frozen selectors and reads retained exact triangle certificates. [verify_metrics.py](verify_metrics.py) imports none of the discovery methods; it independently reconstructs all sample memberships, joins the inherited width labels, audits every stored summary and exact ratio/comparison, and verifies $C_{\rm full}/C_w>1.0789$. [metrics-verification.json](metrics-verification.json) reports PASS. The receiving owner independently checked original source geometry, factor areas, sparse tie ordering, LP duals, direct four-dimensional HK objectives and comparator feasibility. [geometry-verification.json](geometry-verification.json) records PASS for 36 bodies, 360 closure checks, 288 direct four-dimensional objectives, 432 LP dual certificates, 792 containment coefficients, 288 mixed-area identities, 288 sparse-tie checks, 180 comparator words and 36 exact width reconstructions. Its checker is retained by the receiving owner with the separate thesis-side integration evidence; the receipt pins its SHA256.

Source SHA256 is `66bf82010e92e0f26b0df226f4e6c0eef05d21eb22a0967c7f669530f6545736`, from the existing registered local cache. Discovery scripts and retained inputs are pinned in sample/results metadata. All 36 bodies were scored in a single **3.33-second CPU / 3.33-second wall** pass. The new files began under `/tmp/msc-math/full-partner-validation/20261017T1458/`; no capacity/volume producer, fitting, download, dependency installation, cloud job or network action was used. Independent verification validates feasibility and calculations within its scope, not source-label optimality or population generalization.

From this patterns checkout, set `SOURCE` to the existing registered `random-product.jsonl` and `OUT` to a new scratch directory. Copy the frozen membership and plan there before running:

```sh
mkdir -p "$OUT"
cp patterns/upper-tail/full-partner-validation/PLAN.md "$OUT/PLAN.md"
cp patterns/upper-tail/full-partner-validation/sample.json "$OUT/sample.json"
cp patterns/upper-tail/full-partner-validation/analyze.py "$OUT/analyze.py"
python3 -B "$OUT/analyze.py" --repo "$PWD" --source "$SOURCE" \
  --sample "$OUT/sample.json" --out "$OUT/results.json"
python3 -B patterns/upper-tail/full-partner-validation/verify_metrics.py \
  --repo "$PWD" --directory "$OUT"
```

Access and search were focused on repository/research entry points, collaboration guidance, retained pattern/tail reports, the existing full-partner construction and its prior comparators. These reads do not establish exhaustive repository or literature novelty. Evidence is retained on the separate patterns branch; no thesis source is edited here.

[review: /root/validation/packet_review]
