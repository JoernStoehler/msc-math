# QP KKT Error Audit

This packet is the compact dev-QP continuation of the scratch work in
`.worktrees/f64-error-bound-audit/experiments/dev-f64-capacity/kkt_error_audit/`.
It keeps the useful numerical ideas and rejects the old packet shape where that
shape added maintenance cost.

The audit is empirical falsification machinery. It is not a production capacity
route and not a proof-bearing certificate.

Use `SEARCH-LEDGER.md` for the current idea/status ledger: which old ideas were
kept, which were rejected, which remain worth re-exploring, and what proof
patterns should guide the next route-contract search.

## What Was Kept

| Old content | Treatment here |
| --- | --- |
| ternary predicate contract | central output contract |
| verified inverse beta radius | kept as the main theorem-shaped predicate candidate |
| beta-radius Q bound | kept for action interval and capacity isolation checks |
| current f64 admissibility | kept as baseline comparison |
| capacity isolation with not-`False` candidates | kept and simplified |
| exact/f64 comparison rows | kept only for the live predicate and Q-bound questions |

## What Was Not Imported

| Old content | Reason |
| --- | --- |
| one-off HKO debug binary | chat/debug artifact, not a reusable experiment |
| broad epsilon/gate sweeps | useful history, not the current dev surface |
| projection and curvature policy variants | negative or unresolved work; re-add only for a targeted question |
| old `best_guess` hardcoding | contradicted the later theorem-shaped candidate |
| `/tmp/f64-policy-final` outputs | reproducible oracle data, not tracked source |
| monolithic old binary/summarizer shape | too costly to maintain and review |

## Predicate Contract

For one fixed sigma, the target proposition is:

```text
Positive(beta_exact) := every coordinate of beta_exact is > 0
```

A ternary predicate must mean:

```text
True  => Positive(beta_exact)
False => not Positive(beta_exact)
Indet => no claim
```

The verified-inverse candidate first tries to prove a radius
`||beta_f64 - beta_exact||_inf <= R`. Given such an `R`, it decides:

```text
if min(beta_f64) > R:
    True
else if min(beta_f64) < -R:
    False
else:
    Indet
```

Missing inverse data or a failed inverse certificate is not a predicate value.
The summarizer reports it as `None`, and capacity isolation treats it like a
not-`False` competitor.

## Verified Inverse Radius

For the exact binary64 KKT system `K_exact x = b_exact`, computed f64 candidate
`x_hat`, and computed f64 inverse `B`, the candidate proof obligation is:

```text
rho   = ||K_exact x_hat - b_exact||_inf
delta = ||I - K_exact B||_inf
delta < 1
R     = ||B||_inf * rho / (1 - delta)
```

Then `||x_hat - x_exact||_inf <= R`, hence the beta block has the same radius
bound. This packet measures the terms and checks the resulting predicate
against exact rational KKT solves for the same binary64 input.

The Q/action bound currently checked from the beta radius is:

```text
|Q(beta_f64) - Q(beta_exact)|
  <= R * ||H beta_f64||_1 + 1/2 * R^2 * sum_ij |H_ij|
```

The f64 rounding term is still a proof obligation before production use.

## Commands

HKO/artifact smoke:

```bash
cargo run -p exp-dev-quadratic-program --release --bin qp-kkt-error-audit -- \
  --input-source artifacts \
  --family-filter hko2024_f64 \
  --max-rows-per-family 1 \
  --max-candidates-per-case 64 \
  --output /tmp/qp-kkt-error-hko.jsonl
```

Generated random smoke:

```bash
cargo run -p exp-dev-quadratic-program --release --bin qp-kkt-error-audit -- \
  --input-source generated \
  --family-filter generated_random_f64 \
  --generated-samples-per-facet 1 \
  --max-candidates-per-case 32 \
  --output /tmp/qp-kkt-error-generated.jsonl
```

Summarize:

```bash
python3 experiments/dev-quadratic-program/tools/kkt_error_audit/summarize.py \
  /tmp/qp-kkt-error-hko.jsonl \
  /tmp/qp-kkt-error-generated.jsonl \
  --out-dir /tmp/qp-kkt-error-summary
```

Primary outputs:

- `predicate_summary.csv`
- `q_bound_summary.csv`
- `capacity_impact_summary.csv`
- `capacity_impact_by_family.csv`
- `family_summary.csv`
- `report.md`

## Interpretation

Do not use a single decided fraction as the route-usefulness metric. The useful
checks are:

- whether any `True/false` or `False/true` rows appear;
- whether the verified-inverse radius is unavailable on the cases that matter;
- whether not-`False` candidates change nominal capacity;
- whether the best `True` action upper bound beats every not-`False` action
  lower bound in the audited candidate set.

HKO-like degeneracy is expected to remain fallback-required unless the verified
inverse certificate and action isolation both apply.
