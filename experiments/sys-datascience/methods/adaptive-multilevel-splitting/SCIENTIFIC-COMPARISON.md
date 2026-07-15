# Frozen scientific comparison v1

Status: **frozen, costed, and not authorized for target exposure**. The exact
machine-readable contract is in `scientific-comparison-v1.json`. No request in
this comparison has been charged.

## Question and design

The comparison asks whether this fixed adaptive multilevel-splitting policy
generates more hostile `5 x 5` Lagrangian products than IID sampling under the
same charged target budget. It does not estimate a tail probability or claim
invariant/conditional sampling.

There are three independent replicates. Each has 256 adaptive and 256 IID
charged requests, hence 1,536 total:

- adaptive: 64 IID initial particles, then three levels with 32 survivors, 32
  clones sampled uniformly with replacement, and two sequential charged
  Gaussian mutations per clone (`64 + 3 * 32 * 2 = 256`);
- IID: 256 requests from the same fixed valid `5 x 5` product law;
- matched prefix: within a replicate, adaptive initial indices `0..63` and IID
  indices `0..63` use the same geometries. Candidate identities, evaluations,
  requests, and caches remain arm-specific, so both copies are charged. IID
  indices `64..255` are fresh;
- replicate seeds: `9823939103103691338`, `12729872386946910155`, and
  `15242111109740286705`;
- arm order: adaptive/IID, IID/adaptive, adaptive/IID;
- mutation scales: 0.08 gap logits, 0.04 centered log radii, and 0.08 relative
  phase radians. These are carried forward from readiness without tail-value
  tuning;
- factor exchange is not quotiented during search. Analysis deduplicates by
  the lexicographically smaller canonical exact geometry key of `X` and `JX`,
  where `J(q1,q2,p1,p2) = (p1,p2,-q1,-q2)`.

The base stream is exactly `iid_base_candidate_attempt` from
`equal-budget-product-search/src/chart.rs` at readiness source revision
`8c63f94f72d4af7007a04ea49f74e252c4959017`: BLAKE3 derive-key context
`s0-iid-product-base-stream-v1` over little-endian u64 `(master_seed,
replicate, base_index, construction_attempt)`, followed by `ChaCha8Rng` and the
retained five-facet `[0.8,1.2]` polygon constructor. Arm is absent from this
material. Both arms call the same stream for indices `0..63`; candidate IDs
remain arm-specific. IID continues the stream at indices `64..255`.
Chart encoding, continuous-coordinate conversion, reconstruction, and
canonical re-encoding use `ProductChart` at the same reference revision.

The comparison-specific random schedule is
`sha256_counter_box_muller_v1`, and the kernel is
`non_invariant_threshold_only_gaussian`. Clone index `c` at level `l` is the
big-endian integer in the first eight bytes of
`SHA256("ams-clone-assignment-v1\nams-vs-iid-scientific-v1\n{master_seed}\n{replicate}\n{level}\n{clone_index}\n")`
reduced modulo 32. Base candidate IDs are the first 24 hex digits of
`SHA256("ams-vs-iid-scientific-base-v1\n{master_seed}\n{replicate}\n{arm}\n{base_index}\n{construction_attempt}\n")`.
Each mutation proposal ID is the first 24 hex digits of
`SHA256("ams-vs-iid-scientific-mutation-v1\n{master_seed}\n{replicate}\n{level}\n{clone_index}\n{mutation_step}\n{construction_attempt}\n{parent_candidate_id}\n")`.
For coordinate pair `k`, its two standard normals use the first two u64s of
`SHA256("ams-mutation-gaussian-v1\n{proposal_candidate_id}\n{coordinate_pair}\n")`, interpreted
big-endian and converted by `u = ((u64 >> 11) + 0.5) / 2^53`, then the ordinary
Box--Muller cosine/sine pair. Coordinates 0--3 and 8--11 are gap logits, 4--7
and 12--15 are the independent centered-log-radius coordinates, and 16 is
relative phase. A rejected construction increments `construction_attempt` and
therefore receives a fresh proposal ID/draw.

Selection is descending `sys`, then ascending candidate ID. The threshold is
the 32nd survivor's `sys`. Each successful proposal is accepted exactly when
`sys >= threshold`; survivors remain in the population. Construction
rejections are uncharged, fully retained, and retried at most 64 times for the
same logical slot. Every target request is charged before an arm-private cache
lookup; duplicates and cache hits remain in the budget. Each `(replicate,
arm)` cache begins empty and is keyed by exact geometry. There is no cache or
retained-result reuse across arms or replicates.

## Frozen decision

For each arm and replicate, define best-so-far AUC as the mean of the 256
running best values, starting from zero; a failed request occupies its index
and does not improve the running best. Define `T8` by sorting successful
factor-exchange-orbit-distinct results by descending `sys`, then ascending
factor-exchange orbit key, and taking the mean of ranks four and five
(one-indexed) among the leading eight. Within an orbit, the representative is
the row with largest `sys`, then smallest candidate ID.

The policy becomes a credible adaptive generator only when at least two of the
three replicates simultaneously satisfy:

- `adaptive AUC - IID AUC >= 0.02`; and
- `adaptive T8 - IID T8 >= 0.01`.

Maxima are always reported. `adaptive max - IID max >= 0.02` in at least two
replicates is a maximum-only diagnostic, not promotion by itself.

Every scientifically interpretable replicate must have exactly 256 requests
per arm, zero target failures, at least 32 factor-exchange-orbit-distinct
adaptive states after every level, at least four initial roots in every
survivor set, at least two roots in the final adaptive top eight, at least one
accepted mutation per level, and at least eight orbit-distinct successful
results per arm. A failed health gate is policy degeneracy/incomplete evidence,
not a negative search result. Mixed healthy replicate outcomes are reported as
heterogeneous rather than forced into a positive/negative label.

Any successful observed `sys > 1` synchronously flushes its exact row, stops
unrelated work and the comparison, and triggers independent classification as
a new trusted example, a known-equivalent control, or evaluator failure. The
comparison remains incomplete and cannot resume without a new authorization.

## Measured cost

The readiness smoke gives target means `b = 0.422865 s`, `m0 = 0.372572 s`,
`m1 = 0.371567 s`, and `i = 0.386028 s`; the largest target time was
`u = 0.610637 s`. Because readiness has only two mutation levels, the third
level is costed as

`m2 in [min(m0,m1), max(u, m1 * max(1,m1/m0))] = [0.371567, 0.610637] s`.

Thus the target-only estimate is

`3 * (64b + 64m0 + 64m1 + 64m2 + 256i) = 591.9..637.8 s`,

or 9.87..10.63 minutes serial. Extrapolating measured construction/artifact
gaps by stage, with third-level overhead between the observed level-1 and
adaptive-initial means, adds 280.6..307.6 seconds. The all-in measured
extrapolation is therefore **872.5..945.4 seconds (14.54..15.76 minutes)**.

Each arm has a separate 600-second cost-only wall limit. This caps all six arms
at about 60 minutes plus finalization even if the measured extrapolation is
wrong. The exact target budget remains 1,536 regardless of cache hits. A
reasonable incremental labor estimate for generalizing/reviewing the driver,
executing, independently reviewing, and reporting the comparison is 6–12
Codex hours ($180–$360), plus 0.5–1 Jörn hour after results exist ($150–$300):
about **$330–$660** beyond this checkpoint.

The previously considered 768-request design (three 128-per-arm replicates) is
rejected. In retained IID calibration, reducing blocks from 256 to 128 roughly
doubled top-eight variability (SD 0.0097 to 0.0196) and moved ranks four/five
from about the 98.2nd to the 96.5th percentile. It cannot resolve the frozen
0.01-scale upper-tail quality difference. Three replicates are already the
minimum for the two-of-three rule.

## Return branches

The report applies this exclusive precedence:

1. any observed `sys > 1`: independent classification;
2. otherwise any evaluator or artifact failure: evaluator/artifact failure;
3. otherwise any failed health gate: policy degeneracy, with diversity
   collapse recorded as a subtype when applicable;
4. otherwise at least two replicates meet both primary thresholds: credible
   adaptive generator;
5. otherwise at least two replicates meet the maximum-only threshold: healthy
   maximum-only signal;
6. otherwise at least one replicate meets either individual primary threshold:
   heterogeneous healthy result;
7. otherwise: healthy AMS-specific negative.

A healthy negative therefore means that all three replicates passed every
health gate and none reached either material primary threshold. It closes this
AMS conditional-resampling policy on this chart, not adaptive search in
general.

Implementation and a fresh target-free exact-commit review remain mandatory
before exposure. Freezing this protocol does not authorize those 1,536 target
requests.
