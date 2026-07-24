# Two-Face Control LICCA Smoke Plan

Status: dormant reusable LICCA smoke design. It is not selected for execution.
A new research decision must first select the associated producer axes
and justify a current compute/review packet. If reactivated, this plan tests
producer plumbing, not research-scale evidence.

Plan file:

```text
experiments/polytope-datasets/plans/two-face-control-licca-smoke.json
```

## Objective

Conditional objective if new research reactivates the proposal: test the smallest useful
smoke plan for:

- Slurm submission and environment export;
- `--plan-file` parsing;
- both producer families;
- every bucket in the dormant high-complexity replication proposal;
- multiple sample indices per bucket;
- nonuniform bucket counts so bucket/source mixups are detectable;
- producer output, validation, prepare, and fingerprint.

The objective is not to estimate high-tail behavior or produce thesis evidence.
Passing this smoke would not authorize production; a separate current
production decision and review gate would still be required.

## Chosen Counts

```text
random:
  F10: 3
  F11: 5
  F12: 6

random_product:
  4x6: 7
  5x5: 9
  5x6: 10
  6x6: 11

total: 51 rows
```

## Selection Rule

Under these smoke-design constraints:

1. every bucket in the dormant replication proposal appears;
2. every bucket count is distinct;
3. every bucket count is at least `3`;
4. no bucket count is a power of two;
5. total row count is minimized;

the smallest count multiset is:

```text
3, 5, 6, 7, 9, 10, 11
```

The assignment puts the three smallest counts on the generic random buckets and
the four larger counts on product buckets. Product capacity rows are not cheaper
in general, but this assignment keeps the product family totals distinctive
while preserving the exact high-complexity bucket set.

## Why It Beats The Alternatives

| Alternative | Why not |
| --- | --- |
| `1` per bucket | Does not exercise multiple sample indices within each bucket. |
| uniform small counts such as `8` or `16` | Bucket collapse or bucket-label swaps can preserve family totals and remain harder to spot. |
| powers of two such as `128` | Larger than needed and has no diagnostic advantage for this objective. |
| `127`, `128`, or `129` per bucket | Much more expensive while testing the same parser/schema paths. |
| distinct primes such as `2,3,5,7,11,13,17` | Includes a one-row or two-row bucket if minimized, or costs more if all counts are at least three. |
| `197`-row odd-prime smoke | Useful as mild runtime stress, but not better for the plumbing objective per row. |
| production | Evidence-producing run; should not be the first test of wrapper/cache/prepare plumbing. |

## Required Validation

If reactivated, this plan works as a diagnostic smoke only when validation
checks the exact bucket vector. Family totals alone are insufficient.

Use:

```bash
python3 experiments/polytope-datasets/validate-datascience-produced.py \
  --produce-dir "$SMOKE_DIR" \
  --mode smoke \
  --producers random,random-product \
  --expected-plan-file experiments/polytope-datasets/plans/two-face-control-licca-smoke.json
```

Expected totals derived from the plan:

- random rows: `14`;
- random-product rows: `37`;
- computed payload rows: `51`.
