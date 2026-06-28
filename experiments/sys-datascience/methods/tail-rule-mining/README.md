# tail-rule-mining

## Research Question

Can a simple interpretable rule learned from geometry-only features isolate the
upper `sys` tail of the trusted random/product sample better than source and
generator/stratum labels alone?

This is a standard high-tail rule-mining diagnostic. It is not a validated
candidate-proposer because every row in the train and test table already has
`sys` computed.

## Method

Train shallow decision-tree classifiers for two in-table labels:

- top decile: `sys` at or above the full-table 90th percentile;
- top five percent: `sys` at or above the full-table 95th percentile.

The geometry-only tree uses the shared random-only geometry feature selector.
It also runs the same tree diagnostic on disjoint engineered feature families:

- `symplectic_omega_only`: ridge symplectic-area, ridge/all-pair omega, and
  omega-matrix summaries;
- `euclidean_size_spread_only`: volume-normalized Euclidean norm, distance,
  singular-value, edge-length, facet-volume, and cosine summaries;
- `combinatorial_counts_only`: counts, incidence summaries, ridge sizes, and
  edge density;
- `transition_graph_only`: transition-graph summaries.

Two categorical baselines separate different concerns:

- `strata_only`: `capacity_source`, `facet_count`, and product bucket. These
  are sampling strata. `facet_count` is also an interpretable
  geometric/combinatorial feature, not mere provenance.
- `generator_provenance_only`: `capacity_source`, product bounce count, and
  height range. These are controls for source/generator provenance available in
  the retained table.

Rows are split by `capacity_source:facet_count` with the same grouped-holdout
convention as `prediction-ranking/`.

For each tree, record grouped-holdout precision, recall, enrichment over the
base rate, and the highest-probability leaves. Leaf rules are reported as
interpretable diagnostics only. A stability sweep reruns the comparison across
several grouped splits, tree depths, and minimum leaf sizes.

## Inputs

- trusted random-only rows from `../_shared/random_only.py`
- `../../prepare/polytope-table.jsonl`
- `../../prepare/polytope-provenance-table.jsonl`

## Command

```bash
uv run --script experiments/sys-datascience/methods/tail-rule-mining/analyze.py
```

For scratch prepared tables:

```bash
uv run --script experiments/sys-datascience/methods/tail-rule-mining/analyze.py \
  --tables-dir /tmp/sys-ds-random-only-full-current \
  --out-dir /tmp/sys-ds-full-current/tail-rule-mining
```

## Generated Artifacts After Rerun

- `summary.json`
- `leaf-rules.tsv`
- `stability-runs.tsv`
- `stability-split-features.tsv`
- `bucket-interpretation-diagnostics.tsv`

`summary.json` also records fixed coarse baselines and a label-permutation
null check for the single grouped split. It includes the first rows of
`bucket-interpretation-diagnostics.tsv` for navigation, but the TSV is the
recomputed source for bucket-level interpretation.

## Observation

Current full scoped random/product run after adding Euclidean two-face area
controls, using `/tmp/sys-ds-random-only-full.EbpaS8` as input and writing
`/tmp/sys-ds-two-face-euclidean-control-tail-rule-full`:

- rows: `14336`;
- geometry-only features: `146`;
- symplectic/omega features: `65`;
- Euclidean size/spread features: `48`;
- combinatorial/count features: `17`;
- transition-graph features: `6`;
- strata-only one-hot features: `21`;
- generator-provenance-only one-hot features: `6`;
- grouped split: `capacity_source:facet_count`;
- train rows: `9216`;
- test rows: `5120`;
- top-decile threshold: `0.6020490648950583`;
- top-5% threshold: `0.6624200460861029`.

Single grouped-holdout tree results:

| label | feature source | precision | recall | enrichment over holdout base rate | selected rows |
| --- | --- | ---: | ---: | ---: | ---: |
| top decile | geometry only | `0.3878116343490305` | `0.835820895522388` | `5.927150948856824` | `722` |
| top decile | symplectic/omega only | `0.3878116343490305` | `0.835820895522388` | `5.927150948856824` | `722` |
| top decile | Euclidean size/spread only | `0.36006825938566556` | `0.6298507462686567` | `5.503132800163008` | `586` |
| top decile | transition graph only | `0.08422214049282825` | `0.6835820895522388` | `1.2872159979799422` | `2719` |
| top decile | combinatorial/count only | `0.0` | `0.0` | `0.0` | `0` |
| top decile | strata only | `0.0` | `0.0` | `0.0` | `0` |
| top decile | generator provenance only | `0.11760154738878142` | `0.9074626865671642` | `1.7973729033748087` | `2585` |
| top 5% | geometry only | `0.23123123123123124` | `0.9447852760736196` | `7.263214134379778` | `666` |
| top 5% | symplectic/omega only | `0.22627737226277372` | `0.950920245398773` | `7.10760825757915` | `685` |
| top 5% | Euclidean size/spread only | `0.2570093457943925` | `0.6748466257668712` | `8.072931597958831` | `428` |
| top 5% | transition graph only | `0.040266106442577033` | `0.7055214723926381` | `1.2648003986870822` | `2856` |
| top 5% | combinatorial/count only | `0.0` | `0.0` | `0.0` | `0` |
| top 5% | strata only | `0.037353515625` | `0.9386503067484663` | `1.1733128834355828` | `4096` |
| top 5% | generator provenance only | `0.057640232108317216` | `0.9141104294478528` | `1.8105398061017433` | `2585` |

Fixed coarse baselines:

| label | rule | scope | precision | recall | enrichment | selected rows |
| --- | --- | --- | ---: | ---: | ---: | ---: |
| top decile | product rows | full table | `0.11748046875` | `0.8389121338912134` | `1.1744769874476988` | `10240` |
| top decile | product rows | grouped holdout | `0.076171875` | `0.9313432835820895` | `1.164179104477612` | `4096` |
| top decile | generic rows | full table | `0.056396484375` | `0.16108786610878661` | `0.5638075313807531` | `4096` |
| top decile | facet count `>= 10` | full table | `0.18501420454545456` | `0.7266387726638772` | `1.8496259667807786` | `5632` |
| top decile | facet count `>= 10` | grouped holdout | `0.0` | `0.0` | `0.0` | `0` |
| top 5% | product rows | full table | `0.06064453125` | `0.8661087866108786` | `1.2125523012552302` | `10240` |
| top 5% | product rows | grouped holdout | `0.037353515625` | `0.9386503067484663` | `1.1733128834355828` | `4096` |
| top 5% | generic rows | full table | `0.0234375` | `0.13389121338912133` | `0.4686192468619247` | `4096` |
| top 5% | facet count `>= 10` | full table | `0.09481534090909091` | `0.7447698744769874` | `1.895777862305059` | `5632` |
| top 5% | facet count `>= 10` | grouped holdout | `0.0` | `0.0` | `0.0` | `0` |

The geometry-rule enrichment is much larger than direct product-vs-generic
selection. Facet count `>= 10` is enriched on the full table, but the grouped
holdout split used here contains no `facet_count >= 10` rows, so it is not a
valid holdout comparator for this particular split.

### Bucket Interpretation Diagnostics

`bucket-interpretation-diagnostics.tsv` is the durable surface for
source/facet-bucket interpretation. It is regenerated from the current
prepared table and should be preferred over copying one-off interpretation
numbers into this README.

Each row fixes `capacity_source` and `facet_count`, chooses a within-bucket
top-decile or top-5% `sys` label, and reports the row-level association
`K -> (sys(K), f(K))` for a small set of interpretable scalar quantities. The
artifact includes:

- `mathematical_quantity`: prose definition of `f(K)`;
- `spearman_with_sys`: rank association inside the bucket;
- `feature_tail_rule`: whether the lowest or highest 15% of `f(K)` was used;
- `precision`, `recall`, `base_rate`, and `enrichment` for that one-scalar
  within-bucket rule.

The currently included quantities are:

- total and mean volume-normalized symplectic areas of primal two-faces,
  computed as `0.5 * |sum_i omega0(v_i, v_{i+1})| / sqrt(volume)` over
  cyclically ordered two-face vertices;
- total and mean volume-normalized Euclidean polygon areas of the same primal
  two-faces in `R^4`;
- summary quantiles of the per-two-face ratio
  `symplectic polygon area / Euclidean polygon area`, computed only when the
  Euclidean area is nonzero;
- volume-normalized symplectic pairings of facet normals, including the
  spectral norm of the matrix `sqrt(volume) * omega0(a_i, a_j)`;
- selected Euclidean size/spread controls.

Use this artifact to choose a simple bucket for discussion, then verify the
feature definition in `prepare/features_face_symplectic.rs`,
`prepare/features_omega.rs`, or the relevant feature module before presenting
the result. Do not treat a bucket row as a mechanism or theorem; it is an
empirical association computed from the current retained table.

The highest-precision geometry leaves use normalized ridge symplectic-area and
omega-matrix stable-rank/spectral-norm features. The best top-decile geometry
leaf has `55` test rows, positive rate `0.7272727272727273`, mean `sys`
`0.6511412387550054`, and max `sys` `0.7988226871046541`. The best top-5%
geometry leaf has only `22` test rows, positive rate `0.7727272727272727`,
mean `sys` `0.6978122767310648`, and max `sys` `0.7988226871046541`.

### Euclidean Two-Face Control

The added Euclidean two-face area controls make the old interpretation sharper:
small Euclidean two-face area is a real high-tail correlate, but it does not
match the symplectic-area block on this full retained table.

On the single grouped holdout, `symplectic_omega_only` still selects the same
top-decile rows as `geometry_only`: `280/722` selected rows are top-decile
hits, for precision `0.3878116343490305`, recall `0.835820895522388`, and
enrichment `5.927150948856824`. The Euclidean size/spread block, now including
Euclidean two-face area, selects `211/586` hits, for precision
`0.36006825938566556`, recall `0.6298507462686567`, and enrichment
`5.503132800163008`. For the top-5% label, Euclidean size/spread has higher
precision/enrichment (`110/428`, enrichment `8.072931597958831`) but lower
recall than `symplectic_omega_only` (`155/685`, enrichment
`7.10760825757915`).

In the stability sweep, `symplectic_omega_only` beats
`euclidean_size_spread_only` in enrichment in `61/72` paired top-decile
configurations, with median enrichment difference `0.4602026808551354`, and in
`66/72` paired top-5% configurations, with median difference
`0.86743321125401`. Within the Euclidean block, the new
`ridge_euclidean_area_volnorm_sum` is repeatedly selected
(`11.5` mean splits per run for top decile and `14.875` for top 5%), so
ordinary two-face size is not a negligible control.

The clean generic bucket `capacity_source=random_sample, facet_count=10`
contains `512` rows. For its within-bucket top-decile label, low symplectic
two-face area sum selects `36/77` hits (`52` positives in the bucket; base rate
`0.1015625`; enrichment `4.603396603396603`). Low Euclidean two-face area sum
selects `28/77` hits (enrichment `3.5804195804195804`). The symplectic-over-
Euclidean ratio is much weaker: the mean ratio selects `11/77` hits
(enrichment `1.4065934065934065`), and the median ratio selects `13/77` hits
(enrichment `1.6623376623376624`). For the top-5% label in the same bucket,
low symplectic area mean selects `21/77` hits (`26` positives; enrichment
`5.37062937062937`), low Euclidean area mean selects `17/77` hits
(enrichment `4.347652347652348`), and the ratio summaries are at or near
baseline.

Interpretation: the row-level map
`K -> (sys(K), ridge_symp_area_volnorm_sum(K))` still carries information not
explained by simply exposing Euclidean two-face size. However, the ratio
`A_symp/A_euclidean` is not the main signal in these diagnostics. The pattern
therefore looks like a mixture: high `sys` rows tend to have smaller ordinary
two-face size, which is ball/roundness-like, and an even stronger small
symplectic-area association visible after the Euclidean-size control is added.
This is still retained-table association evidence, not evidence that a
Lagrangian-product/HKO proposer works.

Stability sweep:

- `8` grouped resplits;
- depths `3`, `4`, and `5`;
- minimum leaf fractions `0.01`, `0.015`, and `0.025`;
- `1008` fitted tree/configuration rows across two labels and seven feature
  sources.

Across paired stability configurations:

| label | comparison | left win fraction | median enrichment difference |
| --- | --- | ---: | ---: |
| top decile | symplectic/omega over Euclidean size/spread | `0.8472222222222222` | `0.4602026808551354` |
| top decile | symplectic/omega over strata | `1.0` | `3.221063011457373` |
| top decile | symplectic/omega over generator provenance | `1.0` | `1.593895555559562` |
| top 5% | symplectic/omega over Euclidean size/spread | `0.9166666666666666` | `0.86743321125401` |
| top 5% | symplectic/omega over strata | `1.0` | `4.39227211208452` |
| top 5% | symplectic/omega over generator provenance | `1.0` | `2.6261442869912535` |

The symplectic/omega block is the most stable feature family in this sweep.
Its most stable split features are ridge symplectic-area summaries and
omega-matrix stable-rank/spectral-norm summaries. The Euclidean size/spread
block also carries substantial high-tail signal; on the single grouped split it
has higher precision/enrichment but lower recall than the symplectic/omega
block. Its most stable split features include edge-length spread, facet-volume
sum, and volume-normalized singular-value summaries.

Permutation-null check on the single grouped split, with `32` train-label
permutations per label/source:

| label | feature source | observed enrichment | null median | null max | permutation p-value |
| --- | --- | ---: | ---: | ---: | ---: |
| top decile | geometry only | `5.927150948856824` | `1.042921148816081` | `3.754527610088515` | `0.030303030303030304` |
| top decile | symplectic/omega only | `5.927150948856824` | `1.0414954231775087` | `2.5055052605823342` | `0.030303030303030304` |
| top decile | Euclidean size/spread only | `5.503132800163008` | `0.9710810782954318` | `2.056551704208869` | `0.030303030303030304` |
| top 5% | geometry only | `7.263214134379778` | `0.8590409299299467` | `13.801821899981409` | `0.06060606060606061` |
| top 5% | symplectic/omega only | `7.10760825757915` | `1.0388972992804795` | `3.61630465872409` | `0.030303030303030304` |
| top 5% | Euclidean size/spread only | `8.072931597958831` | `1.0211520314251694` | `3.766119944910479` | `0.030303030303030304` |

With only `32` permutations, the smallest possible reported p-value is
`1 / 33 = 0.030303030303030304`. The symplectic/omega top-decile null has one
high outlier, so this check is a guard against obvious label-leakage/overfit
rather than a precise significance estimate.

### LICCA Targeted Production Rerun

Jörn ran the targeted replication plan on LICCA from commit `f4525347`.
The prepared table has `32768` rows, max `sys`
`0.8978751217405233`, and no `sys > 1` rows. Bucket sizes:

| source | facet count | rows |
| --- | ---: | ---: |
| random product | `10` | `8192` |
| random product | `11` | `4096` |
| random product | `12` | `4096` |
| random generic | `10` | `8192` |
| random generic | `11` | `4096` |
| random generic | `12` | `4096` |

Local trimmed analysis command:

```bash
uv run --script experiments/sys-datascience/methods/tail-rule-mining/analyze.py \
  --tables-dir /tmp/sys-ds-two-face-control-prepare-9846319 \
  --out-dir /tmp/sys-ds-two-face-control-tail-rule-9846319-trimmed-top1 \
  --stability-runs 0 \
  --permutations 0
```

This rerun adds a top-1% label. On the single grouped holdout:

| label | feature source | hits / selected / positives | precision | recall | enrichment |
| --- | --- | ---: | ---: | ---: | ---: |
| top decile | symplectic/omega only | `1013 / 3134 / 1228` | `0.3232291001914486` | `0.8249185667752443` | `4.312528971935419` |
| top decile | Euclidean size/spread only | `613 / 1495 / 1228` | `0.4100334448160535` | `0.499185667752443` | `5.4706742344187465` |
| top 5% | symplectic/omega only | `434 / 1942 / 572` | `0.223480947476828` | `0.7587412587412588` | `6.401244481574039` |
| top 5% | Euclidean size/spread only | `380 / 1862 / 572` | `0.20408163265306123` | `0.6643356643356644` | `5.845582988440132` |
| top 1% | geometry only | `71 / 1352 / 102` | `0.05251479289940828` | `0.696078431372549` | `8.43531732219515` |
| top 1% | symplectic/omega only | `72 / 1448 / 102` | `0.049723756906077346` | `0.7058823529411765` | `7.987000324991875` |
| top 1% | Euclidean size/spread only | `34 / 1101 / 102` | `0.030881017257039057` | `0.3333333333333333` | `4.960339085679685` |

For the clean generic bucket `capacity_source=random_sample, facet_count=10`,
within-bucket top-1% has `82` positives among `8192` rows. The lowest 15% of
symplectic two-face mean selects `65 / 1229 / 82` hits
(enrichment `5.284`); symplectic two-face sum selects `63 / 1229 / 82`
(enrichment `5.121`). Euclidean two-face sum selects `55 / 1229 / 82`
(enrichment `4.471`), and Euclidean two-face mean selects `53 / 1229 / 82`
(enrichment `4.308`). Ratio summaries are much weaker: the mean
`A_symp / A_euclidean` selects `22 / 1229 / 82` hits
(enrichment `1.788`), and the median ratio selects `19 / 1229 / 82`
(enrichment `1.544`).

Interpretation for the row-level maps `K -> (sys(K), f(K))`: Euclidean
two-face area is a genuine high-tail correlate, especially for broad top-decile
selection where it gives the highest precision/enrichment by selecting fewer
rows. It does not beat the symplectic/omega block for top 5% or top 1%, and in
generic `F=10` the original small symplectic-area association survives after
Euclidean two-face size is visible. The ratio `A_symp / A_euclidean` is not the
main signal here. This points to a mixed pattern: there is ordinary
small-face/roundness evidence, but the stronger top-tail signal still comes
from symplectic-area and omega quantities, not from Euclidean size alone.

The full default stability/permutation analysis on the `32768`-row prepared
table was started locally but stopped during the stability sweep because it was
too slow for an interactive pass. The trimmed rerun above is the durable local
artifact for this production table; a full production stability rerun should be
done as a batch job if needed.

Interpretation: shallow geometry-only rules robustly isolate high-tail regions
better than the sampled strata and available generator-provenance controls in
this retained table. The signal is not localized to one column: the
symplectic/omega feature family is strongest under the stability sweep, while
Euclidean size/spread features are also informative. This is an in-table
interpretability diagnostic, not a validated candidate-proposer.

## Validity Guards

- This is in-table rule mining, not a generated-candidate proposer.
- Categorical baseline trees are diagnostics for sampled strata and available
  generator provenance. They cannot be used as geometry candidate rules.
- `facet_count` is not mere nuisance metadata; it is grouped with
  `strata_only` because it is both a sampling input and an interpretable
  geometric/combinatorial feature.
- Tree rules are unstable under correlated features. Use them as compact
  diagnostics of visible high-tail structure, not as mathematical explanations.
- A validated proposer would need to apply a frozen geometry-only rule to newly
  generated unevaluated rows before their `sys` values are computed.

## Current Disposition

Run-pending-review trial method. The current scratch run is successful and
needs method/statistics review before thesis use.

## Interpretation Boundaries

- This packet supports only retained-table high-tail enrichment, not discovery
  of a new `sys > 1` row.
- This packet does not show that a rule will enrich newly generated rows before
  `sys` is computed.
- The stable split features overlap with ridge and omega features from the
  scalar-association packet, so this packet should be treated as a local
  rule-shaped view of that high-tail signal.
- Euclidean size/spread features also carry high-tail signal in this packet.
  The current artifacts do not decide whether the Euclidean signal is
  independent of the symplectic/omega signal or a correlated proxy for it.
- The result is scoped to the current retained random/product producer
  contract. New random distributions, broader height/facet/product ranges, or
  independent producer reruns reopen the packet.

## Predicted Stability Under Rerun

Moderate on unchanged retained tables. Shallow trees are deliberately
interpretable but can select different correlated features after feature-schema
changes.

## Thesis Use

Potentially supports a statement that interpretable high-tail rule mining was
tried and did not by itself validate a candidate-proposer.

## Reopen Triggers

- retained tables are rebuilt;
- geometry feature schema changes;
- a leaf rule is promoted into a generated-candidate experiment;
- thesis wording asks for claims beyond retained-table high-tail enrichment.
