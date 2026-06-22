# Candidate Filter Audit

Dev-only audit for the HK2017/QP route.

This tool answers a different question from `tools/kkt_error_audit/`:

- `kkt_error_audit` studies f64/exact behavior on candidates that survived the
  f64 candidate solve.
- `candidate_filter_audit` enumerates the sigma stream and exact-solves each
  visited sigma, then checks whether the f64 single-sigma solve retained or
  discarded it.

The output is JSONL:

- one `qp_candidate_filter_summary` row per input case;
- optional `qp_candidate_filter_false_discard_example` rows for exact-positive
  sigmas that f64 discarded.

The ground-truth predicate in this audit is:

```text
exact KKT solve exists, all beta_i > 0, and exact Q > 0
```

The f64 retained predicate is:

```text
single-sigma f64 solve returns True or Indet
```

Use targeted source IDs for edit-loop work. Exact-all-sigma runs are
case-dependent and can be too expensive for broad generated banks or HKO-like
inputs.

Example:

```bash
cargo run -p exp-dev-quadratic-program --release --bin qp-candidate-filter-audit -- \
  --input-source generated \
  --generated-seed 99540836 \
  --source-id-filter \
    seed99540836:F5:sample0:attempt5000000008,seed99540836:q4:p5:attempt405000000000 \
  --output /tmp/qp-filter-generated-targeted.jsonl
```
