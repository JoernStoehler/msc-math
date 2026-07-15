# Adaptive multilevel-splitting readiness smoke

## Status and allowed use

This is a target-free-tested pre-run packet. No production target evaluation
has been run and no real-target artifact is tracked. Its next consumer is an
independent review deciding whether exactly 64 real requests may be spent on a
narrow driver/policy-readiness smoke.

The smoke can test target accounting, valid chart mutation, retained genealogy,
post-rejuvenation particle diversity, failure/timeout handling, and measured
cost. It cannot support AMS superiority, a rare-event or tail probability, a
conditional-law claim, or a scientific negative. The kernel is explicitly
`non_invariant_threshold_only_gaussian`; `tail_probability_supported` is
`false` in the manifest and is enforced independently by the analyzer.

## Frozen policy

`resolved-config.json` is the tracked contract. The adaptive arm constructs 16
independent valid `5 x 5` products and runs two levels. Each level orders states
by descending `sys` and ascending candidate ID, retains eight survivors, and
uses a SHA-256 deterministic uniform-with-replacement assignment for eight
clones. Each clone receives two sequential charged Gaussian chart proposals.
A successful proposal becomes the state exactly when `sys` is at least the
frozen level threshold. The adaptive budget is therefore
`16 + 2 * 8 * 2 = 48`; the IID arm has 16 requests.

Gaussian standard deviations remain 0.08 for independent gap logits, 0.04 for
independent centered log radii, and 0.08 radians for relative phase. SHA-256
counter material and Box--Muller draws make clone assignments and mutation
draws independently replayable with Python's standard library. The raw
proposal chart is retained before construction; accepted state uses the
canonical re-encoding from exact-valid geometry. Factor exchange remains
unquotiented.

Construction failures are uncharged, reasoned, and retried at most 64 times.
Every target request is charged before arm-private cache lookup, including a
duplicate, cache hit, target failure, invalid result, or timeout. A failed
target stops the readiness run incomplete.

## Killable target boundary and stop rules

Every uncached request invokes the same reviewed executable through its private
`target-once` subcommand. The parent feeds the constructed f64 dual vertices.
For production, the child reconstructs with
`SysLandscapePolytopeCache::from_f64_dual_vertices`, computes the current
automatic target, and returns a structured result. The parent polls the child
and kills it at the remaining global 600-second deadline. Stdout and stderr are
drained concurrently while polling, and the structured response is bounded at
64 MiB. A timeout is a charged final failure row.

A returned `sys > 1` is written before elapsed-time disposition. Exactly one
such row must be the final charged request and agree with `stop-event.json` in
event, request index, arm, candidate, exact key, scalar, and fixed action.
Independent geometry/target validation is still required before calling it a
candidate.

## Target-free verification

Normal synthetic run:

```bash
cd experiments/sys-datascience/methods/adaptive-multilevel-splitting
rm -rf /tmp/ams-readiness-synthetic
cargo build --release --locked
./target/release/adaptive-multilevel-splitting synthetic \
  --config resolved-config.json \
  --artifacts /tmp/ams-readiness-synthetic
python3 analyze.py /tmp/ams-readiness-synthetic
```

The synthetic child returns deterministic scalars satisfying
`sys = c^2/(2V)` and never invokes the real target. It still exercises the
production driver, child process, charging, cache, mutation, genealogy,
artifact, and stop paths. `--force-synthetic-hit` exercises the stop path.
The following target-free command proves that a slow child is killed and that
the charged timeout row remains auditable; the producer intentionally exits
nonzero and analysis returns `readiness_passed: false`:

```bash
rm -rf /tmp/ams-readiness-timeout
./target/release/adaptive-multilevel-splitting synthetic \
  --config resolved-config.json \
  --artifacts /tmp/ams-readiness-timeout \
  --synthetic-child-delay-ms 100 \
  --synthetic-call-timeout-ms 10 || true
python3 analyze.py /tmp/ams-readiness-timeout
```

## Reserved production command

Do not run this until the exact committed source and executable receive `GO`.
The caller must supply the reviewed full 40-hex commit; the producer compares
it with clean `HEAD` before creating artifacts. Any source change requires a
new review and new reviewed commit value.

```bash
cd experiments/sys-datascience/methods/adaptive-multilevel-splitting
reviewed_commit=FULL_40_HEX_COMMIT_THAT_RECEIVED_GO
repo_root=$(git rev-parse --show-toplevel)
test ! -e /tmp/ams-readiness-production
cargo build --release --locked
./target/release/adaptive-multilevel-splitting production \
  --config resolved-config.json \
  --artifacts /tmp/ams-readiness-production \
  --reviewed-commit "$reviewed_commit"
python3 analyze.py /tmp/ams-readiness-production \
  --expected-reviewed-revision "$reviewed_commit" \
  --repo-root "$repo_root" \
  --cargo-lock "$PWD/Cargo.lock" \
  --executable "$PWD/target/release/adaptive-multilevel-splitting"
```

Production refuses a dirty tree, wrong/missing reviewed commit, reused output
directory, and every synthetic test flag. Production analysis recomputes clean
`HEAD`, the packet lock hash, and the executable hash against the manifest.

## Artifacts and readiness gates

- `manifest.json`: launch run ID/start timestamp, exact config, reviewed source
  and executable/lock identities, budgets, non-invariant kernel, and prohibited
  probability claim.
- `target-evaluations.jsonl`: every charged request, explicit success/failure
  reason, exact and f64 dual geometry, key/identity/facets, canonical and raw
  chart, genealogy/threshold, compact target diagnostics, and wall time.
- `cache.jsonl`: exactly one row per arm-private successful miss, including
  exact geometry, compact diagnostics, and the full production
  `OrbitSearchResult` (synthetic rows identify their formula fixture instead).
- `construction-rejections.jsonl`: every uncharged rejected attempt, identity,
  parent/root, reason, and raw mutation chart when applicable.
- `levels.jsonl`: survivor order/roots, replayable clone assignments, and the
  actual 16-particle post-level candidate/key population.
- `mutation-transitions.jsonl`: before/proposal/after state, frozen threshold,
  acceptance, and root for every sequential proposal.
- `arm-runs.jsonl`: reconciled per-arm attempts, rejection/cache/failure counts,
  completeness, distinct successful keys, and wall time.
- `stop-event.json`: present only after a flushed `sys > 1` stop.
- `run-status.json`: final disposition, matching run ID, charged counts, total
  monotonic wall time, and SHA-256 of every owning artifact except itself.

Only disposition `complete` can pass readiness. It requires exactly 48/16
charged rows, exact base/mutation grids and retry histories, two complete level
populations with at least eight distinct exact geometry keys each, at least two
survivor roots per level, at least one accepted valid mutation, no failed row,
full accounting, and total time at most 600 seconds. The analyzer independently
replays identities, clone assignments, Gaussian mutations, thresholds,
genealogy, the exact global request schedule, exact product geometry and
product volume, canonical chart encoding (absolute chart tolerance `2e-10`),
cache audit, stop evidence, file hashes, and time totals.

## Local gates

```bash
cargo fmt --check
cargo test --locked
cargo clippy --locked --all-targets -- -D warnings
python3 -m unittest -v test_analyze.py
python3 -m py_compile analyze.py test_analyze.py
git diff --check -- experiments/sys-datascience/methods/adaptive-multilevel-splitting
```
