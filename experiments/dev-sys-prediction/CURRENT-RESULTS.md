# Dev Sys Prediction Current Results

Status: first producer and smoke evidence. This is not yet a closure decision
for the layer-2 goal.

## Producer

Command:

```bash
cargo run -p exp-dev-sys-prediction --release --bin dev-sys-prediction-cloud -- \
  --diagnostic-dir /tmp/dev-gradient-ascent-branch-diagnostic-allsafe-check \
  --polytope-table /tmp/dev-sys-prediction-fixture-panel.jsonl \
  --out-dir /tmp/dev-sys-prediction-cloud-largegap-smoke \
  --selection-threshold-relative 0.001 \
  --degeneracy-labels large_gap \
  --max-fixtures-per-label 1 \
  --steps 1e-4 \
  --trace-iterations 1
```

Analogous high-degeneracy run:

```bash
cargo run -p exp-dev-sys-prediction --release --bin dev-sys-prediction-cloud -- \
  --diagnostic-dir /tmp/dev-gradient-ascent-branch-diagnostic-allsafe-check \
  --polytope-table /tmp/dev-sys-prediction-fixture-panel.jsonl \
  --out-dir /tmp/dev-sys-prediction-cloud-highdeg-smoke \
  --selection-threshold-relative 0.01 \
  --degeneracy-labels high_degeneracy \
  --max-fixtures-per-label 1 \
  --steps 1e-4 \
  --trace-iterations 1
```

The compact fixture panel used for these smoke runs was:

```text
/tmp/dev-sys-prediction-fixture-panel.jsonl
```

It was extracted from the retained datascience polytope table. The producer can
read a compact panel through the existing `--polytope-table` argument, so smoke
runs do not need to parse the full retained table.

## Observations

Both smoke runs used the same base polytope:

```text
07455e997d624c62193180fd92026e2aba426e9b5bd1c3be4e8fe303ca4ffe5b
```

At threshold `0.001`, it is a `large_gap` fixture with one near-active branch.
At threshold `0.01`, it is a `high_degeneracy` fixture with ten near-active
branches.

Large-gap smoke:

- rows: 4;
- max absolute active/near prediction error: `5.334050949198921e-08`;
- max absolute candidate-window prediction error: `5.334050949198921e-08`;
- near-active ranking matched observed ranking for all rows;
- candidate-window ranking matched observed ranking for all rows;
- elapsed time: `33.6s` for one fixture and four directions.

High-degeneracy smoke:

- rows: 5;
- max absolute near-active prediction error: `3.555148172480823e-04`;
- max absolute candidate-window prediction error: `5.334050949198921e-08`;
- near-active ranking did not match observed ranking;
- candidate-window ranking matched observed ranking for all rows;
- elapsed time: `33.9s` for one fixture and five directions.

The high-degeneracy result is the important signal. The broad near-active set
is not a good finite-step predictor at this basepoint and radius, while the
low-action candidate-window lower-envelope prediction is excellent. This
supports separating:

- behavior of the returned low-action sigma set
  `action <= min_action(a0) * (1 + threshold)`;
- behavior of individual raw `sysext_sigma(a)` branches and beta-domain
  boundaries.

## Cost Interpretation

The smoke producer is too slow for direct use inside the optimizer loop in its
current form. It recomputes actual `sys(a0 + t u)` for every cloud row with the
AllSafe branch route and exact volume path. That is useful offline evidence,
but it is not yet a cheap step-selection primitive.

This does not falsify prediction as an optimizer aid. It says the optimizer
should not pay full prediction-cloud recomputation at every step. The plausible
split is:

- use branch-window lower-envelope information at `a0` for cheap model
  predictions and direction ranking;
- use sparse recomputed `sys` checks for validation and line search;
- use a separate `sysext_sigma` microprobe for beta-domain geometry, because a
  fixed sigma call should be microsecond-scale and avoids enumerating all
  branches.

## Sysext Sigma Line Probe

Command:

```bash
cargo run -p exp-dev-sys-prediction --release --bin dev-sysext-sigma-line-probe -- \
  --diagnostic-dir /tmp/dev-gradient-ascent-branch-diagnostic-allsafe-check \
  --polytope-table /tmp/dev-sys-prediction-fixture-panel.jsonl \
  --out-dir /tmp/dev-sysext-sigma-line-highdeg-smoke \
  --selection-threshold-relative 0.01 \
  --degeneracy-label high_degeneracy \
  --steps -1e-3,-3e-4,-1e-4,0,1e-4,3e-4,1e-3
```

Output:

```text
/tmp/dev-sysext-sigma-line-highdeg-smoke/sysext-sigma-line-probe.jsonl
```

Result:

- rows: 70;
- sigmas: base best plus nine additional near-active sigmas;
- statuses: all `ok`;
- elapsed time: `298ms` excluding release compile;
- every sampled fixed sigma stayed beta-positive on the tested line;
- the smallest beta margins over the line were about `0.00206` for the two
  closest-to-boundary sigmas.

This supports treating fixed-sigma `sysext_sigma(a0 + t u)` as a cheap separate
object. It should not be merged conceptually with the low-action sigma-set
question:

- low-action set behavior asks how many returned branches are near the minimum
  and which one wins at target points;
- fixed-sigma sysext behavior asks whether an individual KKT branch has a
  stable raw critical point, how its action changes, and whether beta margins
  approach the domain boundary.

For this fixture and line, the low-action candidate-window model was highly
predictive and the fixed-sigma sysext branches behaved smoothly. That does not
yet test beta-invalid raw sysext branches.

## Next Required Work

The current producer answers the low-action candidate-window question for one
basepoint and one radius. It does not yet answer the raw sysext question.

The next pass should extend the line microprobe to beta-invalid or barely valid
raw sysext branches:

Candidate sigma sources:

- candidate-window witness sigma for a prediction row;
- target best sigma from recomputed `sys(a0 + t u)`;
- raw sysext branches from the scratch sprint once packet-local raw KKT
  enumeration is moved into this packet.

This would directly test whether beta-domain failures are smooth/algebraic
line behavior of individual branches, rather than a property of the whole
low-action branch set.
