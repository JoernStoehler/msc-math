# Factorial tangential matchability

This target-free packet asks whether the four existing factorial generator arms
can supply a complete paired geometry panel before any capacity or `sys`
evaluation. Each latent baseline draw produces `factorial-baseline`,
`factorial-q`, `factorial-p`, and `factorial-both` rows with one canonical
`pairing_id`. The reviewed producer family filter prevents unrelated laws from
consuming the panel budget. A mandatory identity scope prevents IDs from
aliasing the earlier alternative-generator artifacts.

## Disposable smoke

Build the current producer, write the smoke outside the repository, then audit
it with the matching expected size and identity scope:

```text
cargo build -p exp-sys-landscape --release --bin sys-datascience-alternative-generator-smoke
target/release/sys-datascience-alternative-generator-smoke \
  --out-dir /tmp/generator-tangential-matchability-smoke \
  --seed 20260714 --attempts 128 --runtime-cap-ms 2000 --rows-per-law 2 \
  --only-family factorial --identity-scope tangential-matchability-smoke-v1
uv run --script experiments/sys-datascience/methods/generator-tangential-matchability/analyze.py \
  --input /tmp/generator-tangential-matchability-smoke/smoke-rows.jsonl \
  --manifest /tmp/generator-tangential-matchability-smoke/batch-report.json \
  --expected-rows-per-bucket 2 \
  --identity-scope tangential-matchability-smoke-v1 \
  --out-dir /tmp/generator-tangential-matchability-smoke/analysis
```

`--only-family` is structurally incompatible with `--target` and requires an
identity scope. The analyzer independently rejects any non-null capacity,
`sys`, or iteration field, any positive target time, incomplete row/pair grids,
split latent attempts, or noncanonical identities.

## Parent-gated retained panel

The full packet requests 64 rows per arm and bucket: 768 rows arranged in 192
structurally complete four-arm grids. The number of accepted-complete geometry
pairs can be smaller when bounded generation is exhausted; such failure rows
remain generator evidence and are counted separately. The retained run is not
authorized by this implementation task. After parent approval, use:

```text
cargo build -p exp-sys-landscape --release --bin sys-datascience-alternative-generator-smoke
target/release/sys-datascience-alternative-generator-smoke \
  --out-dir experiments/sys-datascience/methods/generator-tangential-matchability/artifacts/full-64 \
  --seed 20260714 --attempts 128 --runtime-cap-ms 2000 --rows-per-law 64 \
  --only-family factorial --identity-scope tangential-matchability-v1
uv run --script experiments/sys-datascience/methods/generator-tangential-matchability/analyze.py \
  --input experiments/sys-datascience/methods/generator-tangential-matchability/artifacts/full-64/smoke-rows.jsonl \
  --manifest experiments/sys-datascience/methods/generator-tangential-matchability/artifacts/full-64/batch-report.json \
  --expected-rows-per-bucket 64 --identity-scope tangential-matchability-v1 \
  --out-dir experiments/sys-datascience/methods/generator-tangential-matchability/artifacts/full-64/analysis
```

The repository rule `experiments/**/*smoke*.jsonl` ignores the retained
`full-64/smoke-rows.jsonl`. After the run and review, retention therefore
requires a deliberate force-add followed by a tracked-file check:

```text
git add -f experiments/sys-datascience/methods/generator-tangential-matchability/artifacts/full-64/smoke-rows.jsonl
git ls-files --error-unmatch experiments/sys-datascience/methods/generator-tangential-matchability/artifacts/full-64/smoke-rows.jsonl
```

Do not treat the local file or a successful analyzer run as retained evidence
until the `git ls-files` check succeeds.

The analyzer emits `report.json` with schema
`generator-tangential-matchability-report-v2` and a generated `summary.md`. It
reports structurally complete four-arm grids, accepted-complete geometry pairs,
and structural pairs containing rejected rows as distinct counts. It also
audits bounded attempts and acceptance, unit factor areas,
factor-area/product-volume agreement, scale-free support CV, angular-gap CV,
and isoperimetric distributions by bucket and arm, their range overlap, and
paired equality witnesses for unchanged factor arms. Rejected/exhausted rows
remain visible but do not enter accepted-only geometry summaries.

Normalized ridge features are not included: the exact product cache exposes
vertices and incidence, but their computation currently requires the
prepare-private `features_face_symplectic` module and two-face assembly rather
than a shared cache-to-feature API. Vertex-covariance eligibility is likewise
private `vertex_covariance_diagnostics` in
`extreme-scalar-rejection-proposer`. Copying either implementation and
expanding the shared smoke schema is outside this narrow packet.

Allowed observations concern paired generator plumbing, acceptance cost,
normalization agreement, and coarse factor-geometry matchability within this
fixed generator panel. The packet prohibits capacity/`sys` effects, transfer,
population frequencies, stable arm rankings, causal mechanism, and
all-polytopes claims. Even the retained 64-row panel would remain exploratory
generator evidence, not target evidence.

## Retained panel result

The reviewed `full-64` run retained all 768 requested rows and all 192
four-arm pairings. Every row passed exact product reconstruction; no pair
contained a bounded failure. The largest attempt count was 117 of 128, in the
`3x3` bucket. Factor areas and product volume agree within `1.32e-14`.

Tangentialization sends the modified factor's support CV to numerical zero and
leaves its angular-gap CV unchanged because the normal fan is paired exactly.
The isoperimetric-ratio ranges remain strongly overlapping: the all-arm range
intersection over union is between `0.967` and `1.000` across factors and
buckets. Thus this panel is well matched in latent fan and coarse compactness
while deliberately separated in support variability.

This clears generator feasibility and coarse matchability only. Normalized
ridge and covariance fields are still absent from the row payload, and the
packet contains no capacity or `sys`. The next decision is whether adding the
exact feature payload to a small paired pilot is worth its implementation cost;
the retained geometry panel itself does not authorize target exposure.
