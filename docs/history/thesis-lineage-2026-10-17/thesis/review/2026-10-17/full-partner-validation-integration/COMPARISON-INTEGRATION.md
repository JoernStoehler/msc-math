# Containment comparison and later joint-closure result

17 October 2026. The optional [explanation candidate](EXPLANATION-CANDIDATE.md) now scopes its **1.52%** figure to the original containment-plus-incumbent method and original 27-tail panel. It also identifies the later joint-closure result without conflating the two methods or samples. The selected manuscript and Kai PDF are unchanged. Human inclusion question `f57e68b5` remains pending; insertion task `9ef3cbae` remains blocked on that verdict. This integration supplies no human acceptance.

## End state and editorial choice

The intended end state is a reviewable explanation whose historical figures remain reproducible and whose newer context cannot suggest an unresolved old miss or universal success. A membership join is necessary because the panels differ; an exact row join is necessary because the later result addresses the formerly worst body; method names are necessary because containment alone did not achieve the later performance. Source hashes preserve which retained results support each statement. No new experiment or numerical polishing was selected.

Three prose options were compared: keep the historical paragraph with only a footnote (shortest, but easy to read as current best); replace it entirely with the joint-closure result (current, but discards the contact mechanism Jörn is being asked to assess); retain the contact explanation and add a short, clearly distinct later-result paragraph (more reader burden, but preserves both meaning and current evidence). A shortest-text rubric can conceal consequential scope; a best-score rubric can erase mechanism and sample changes; a most-results rubric can overload the chapter. The chosen draft uses the third option, judged by geometric insight, accurate comparison and reading cost. This is a draft choice, not a manuscript-inclusion decision.

## Exactly which panels overlap

| Scope | Original full-partner containment validation | Later conditional-closure validation |
|---|---:|---:|
| Principal upper tails | 27: 9 each from 5×5, 5×6, 6×6 | 26: 9, 8, 9 respectively |
| Hash-selected bulk controls | 9 | The same 9 |
| Total | 36 | 35 |
| Original design exclusions | 20 | The same 20, plus the conditional design exclusions |

The intersection is **all 35 later bodies**, with identical names, geometries, inherited capacity/ratio labels, family ranks and sample roles. There are no later-only bodies and no replacements. The only original-panel body removed is **`random_5x6_s1_198`**, the old standalone containment rule's exact width obstruction. The other two declared conditional design controls, `random_5x5_s1_64` and `random_6x6_s3_124`, were already absent from the original validation panel. All 35 common names are retained in [comparison-verification.json](comparison-verification.json).

The omission is declared design exclusion, not a new score or a deletion after this integration. Nevertheless, agreement on the later panel cannot be presented as curing every old failure: it does not test the old obstruction as an excluded validation body. That body has separate design-stage evidence in the scientific packets. Neither panel is a prospective holdout; the corpus and labels had already been exposed.

## Which methods the numbers describe

| Method and sample | Meaning of retained result |
|---|---|
| Original standalone containment, 27 tails | Four prescribed variants from canonical cone-volume closures: both shadow signs and both response orders, one response in each factor. Six ≥1% cuts of previous feasible bounds; seven strict regressions; one exact >7.8956% loss to width. |
| Original post-score minimum, the same 27 tails | Minimum of containment and the previous incumbent (width, triangle, contact, earlier signed-prefix response and canonical witnesses). 26/27 within 1% of inherited numerical capacity; 25 numerical agreements; maximum historical-label gap **1.5186788152%**. |
| Later joint-closure LP and row/column rounding, 26 common tails + 9 common bulk | Nonnegative row/column-closing couplings for each of two angular shadow words, followed by prescribed conditional responses. All 35 numerical agreements; **26 exact capacity certificates = 17 tails + 9 bulk**. Nine tails remain uncertified. The standalone rounded bound equals its minimum with the stronger previous incumbent on this panel; it has no regressions. |

The later incumbent is exactly the old post-score minimum on every shared body; the retained rational capacities agree row by row. Its only two strict improvements are `random_5x5_s3_227` (material) and `random_6x6_s2_63` (0.0541%, polishing alone). This integration does not count the latter as a separate material outcome. The later method optimizes a joint coupling; it is not the earlier containment selector or a simple scalar invariant. No global-maximality claim or new ratio exclusion follows.

## The old maximum is now certified on the shared body

The unique largest old combined historical-label gap belongs to **`random_5x5_s3_227`**, a 5×5 tail with family rank 6 retained in both panels. The old combined upper bound was **1.5186788152%** above its numerical capacity label `5.691937093646149`.

The new rounded upper equals the new LP lower **exactly**, giving

$$C_{\rm round}=c_{\mathrm{EHZ}}=C_{\rm LP,lower}\approx5.691937093646149.$$

The exact rational value is retained in the receipt's `material_row.new_upper` and `material_row.new_lower`, and independently checked identical. Its attaining positive-shadow word is `[9,0,5,1,6,2,7,3,4,8]`; active factor supports are `[0,1,4]` and `[0,2,4]`. The exact cut of the previous feasible bound is **1.4959599878%**. This differs from the old 1.5186788152% label gap because the denominators differ. The new upper/lower identity establishes capacity equality through the scientific certificates; numerical agreement alone would not.

The new exact certificate is material because it closes the original panel's largest combined miss without exhaustive support-pair enumeration. The other 24 old attaining witnesses certified by this later LP were not already certified capacities before that calculation: their old upper bounds now match newly computed lower bounds. Nine remaining tails have numerical agreement but unresolved lower/upper intervals. No further tuning or experiment is part of this integration.

## Verification and reproduction

The standard-library [comparison-check.py](comparison-check.py) checks both scientific manifests, declared source-file hashes, sample/plan bindings, exact membership differences, unchanged shared geometry and label rows, every old/new incumbent identity, rational upper/lower identities and cuts, exact-capacity counts, and the original post-score tail metrics from retained rows. It runs no optimizer, selector or capacity producer. [comparison-verification.json](comparison-verification.json) records **PASS**, input SHA256 values, all memberships, every common-row comparison and the exact material row.

From the thesis checkout:

```sh
python3 -B thesis/review/2026-10-17/full-partner-validation-integration/comparison-check.py \
  --patterns ../patterns-2026-10-17 \
  --out /tmp/msc-math/containment-comparison/recheck/comparison-verification.json
```

This arithmetic checker does not replay the geometric or LP certificate proofs. Those claims retain the scope of the hash-bound original [geometry verification](../../../../../patterns-2026-10-17/patterns/upper-tail/full-partner-validation/geometry-verification.json) and later [scientific verification](../../../../../patterns-2026-10-17/patterns/upper-tail/conditional-validation/verification.json) plus its [independent owner verification](../../../../../patterns-2026-10-17/patterns/upper-tail/conditional-validation/owner-verification.json). The new checker joins their retained results and binds their bytes; it does not reconstruct the full original corpus or extend their theorem scope. The shared declared original-geometry SHA256 is recorded; no new original-dataset verification is implied.

Sources inspected: both checkouts' entry guidance, thesis/research artifact owners, original full-partner validation report/sample/results/manifest and complementarity rows, and conditional-validation report/sample/results/manifest/certificate receipts. This is focused repository inspection, without a new literature search or exhaustive repository audit. New files began under `/tmp/msc-math/containment-comparison/20261017-1656/` and were deliberately retained with this optional thesis-integration evidence. The [independent root comparison](comparison-root-verification.json) separately checks the shared geometries and labels, source-result bindings and exact old/new material-row identity. Root owns exact-text review, PM state, final integration and commit; the integration worker owns these files and reports their checks. No manuscript/PDF build is needed because no manuscript or PDF bytes changed.

[review: /root/question_triage]
