# Ridge Endpoint Path

Status: the repaired, frozen eight-row endpoint packet passed the recorded
technical review. The target artifact set was replaced only after the
mechanical provenance and certificate repairs; `reviews/` preserves both the
initial rejection and the final acceptance.

Question: after a ridge-area feature becomes exceptionally small inside a
fixed product bucket, does `sys` continue to improve along two explicit,
nondegenerate equality-family paths, or does the proxy reverse before its
analytic endpoint?

This packet serves later research by preserving a pre-target construction and
the resulting bounded observation. It is not thesis prose, a population study,
or a general candidate proposer.

## Frozen design and provenance

The exact candidates are in `artifacts/candidates.jsonl`. There are exactly
eight: `q01`, `q001`, `q0001`, and `endpoint` for each of the `3x6` and `4x4`
product buckets. `design_candidates.py` constructs only these fixed candidates
from the target-free feature thresholds embedded in the script. It never calls
the capacity or `sys` API.

`artifacts/cdf-placement.{json,tsv}` places their ridge values in the frozen
one-million-row, per-bucket feature table. The detailed CDF artifact records
the input SHA-256 and counts, not a machine-local cache path. The endpoint
count of zero is right-censored at the retained 100,000 rows per bucket; it is
not an estimated population rarity or a finite number of rarity bits.

`artifacts/api-verification.jsonl` is the retained pre-target geometry/feature
acceptance artifact. Every row has the expected product combinatorics,
successful face ordering, support heights in the frozen interval, and
agreement between the edge formula and current four-dimensional feature.

## Artifacts and source separation

Source is intentionally small:

- `design_candidates.py`: deterministic, target-free candidate producer.
- `place_in_frozen_cdf.py`: feature-CDF placement analyzer; requires an
  explicit frozen cache path.
- `src/main.rs`: eight-row target evaluator and capacity-provenance manifest
  producer.
- `refresh_summary_links.py`: provenance-only summary-link refresher; makes no
  target calls.
- `check_packet.py`: no-target-call identity/linkage check for the retained
  packet. It checks the packet-local retained `Cargo.lock` and reads the
  implementation closure from the manifest's recorded Git commit, rather than
  treating later checkout changes as corruption of the retained run.

Generated evidence is retained under `artifacts/`. In particular,
`target-evaluation.jsonl`, `target-summary.json`,
`capacity-implementation-manifest.json`, and
`q01-certified-minimizers.json` are generated outputs, not hand-edited tables.
`evaluator-source-v2-before-promotion.rs` is an archival source copy whose
SHA-256 identifies the retained target run; the runnable current source is
`src/main.rs`.

The four reviewed mathematical records are in `notes/`; their status and
boundaries are summarized in `DERIVATION-STATUS.md`. Technical review records
live in `reviews/`.

## Allowed use

The accepted repaired artifacts may support all of the following narrow
statements:

- the two endpoint formulas and endpoint `sys` controls described in the
  reviewed derivation notes;
- that these eight pre-target-frozen, hand-designed geometries passed the
  retained geometry/feature contract;
- the numerical `sys` trajectory recorded for these two rotation paths;
- a planning decision about whether a general optimizer with only this ridge
  objective is presently justified.

The q01 exact-minimizer record is stronger evidence for that one submitted
f64-derived rational geometry and its enumerated stream, but not for a
symbolic ideal family or an unrestricted exact search.

## Prohibited claims

Do not use this packet as evidence of:

- a random-generator tail probability, hit rate, or a number of rarity bits
  beyond the frozen empirical resolution;
- a held-out validation, repeatable proposer, causal effect, or general
  relation from low ridge sum to `sys`;
- monotonicity outside the two frozen one-dimensional paths, an active-branch
  mechanism, or a bucket-only mechanism;
- a new `sys=3/4` or `sys=1/2` mathematical value, a `sys>1` result, or a
  Viterbo counterexample.

## Reproduction and checks

All commands run from this directory. `cargo` resolves repository code through
the current checkout. The retained-packet check reports drift in the
packet-local `Cargo.lock`, artifact bytes, and the capacity implementation
closure as staleness warnings. It uses
`capacity-implementation-manifest.json`'s `repo_commit` when available. It may
print a non-failing current-checkout drift diagnostic when implementation paths
have changed since the retained run.

```bash
cargo check --locked
python3 design_candidates.py
python3 check_packet.py
```

The first Python command regenerates the deterministic candidate/design files;
compare them with the committed artifact identity before treating a changed
result as evidence. To regenerate CDF placement from an available frozen
feature cache, pass it explicitly:

```bash
python3 place_in_frozen_cdf.py --cache /path/to/candidate-feature-table.jsonl
```

This overwrites only the CDF placement artifacts. Confirm the cache SHA-256
against the packet before use. If only provenance presentation changed while
the retained counts and input digest stayed fixed, refresh and check the links
without target calls:

```bash
python3 refresh_summary_links.py
python3 check_packet.py
```

If geometry, target values, the capacity implementation, or the certificate
changed, regenerate the linked target artifacts together:

```bash
cargo run --release
```

That final command deliberately reevaluates exactly the frozen eight
candidates and overwrites target/manifest/certificate/summary artifacts. It
is not a smoke check and must not be used to add candidates or change their
geometry.

## Reopen gate

Do not expand the candidate set or run a general optimizer merely to make this
packet look more complete. Reopen only for a named decision not answered by
the current paths, with a target-free frozen geometry/rule, expected result or
stopping rule, and an independent review plan. A `sys >= 1` result requires
immediate independent capacity and geometry verification before unrelated
exploration.
