# f64 Capacity Verification

This packet is a small rerunnable f64 capacity verification/audit surface. It
checks representative f64 rows against handwritten expectations and reports
expectation status separately from f64 claim scope.

Run from the repository root:

```bash
experiments/dev-quadratic-program/verification/run.sh /tmp/f64-capacity-verification
```

Outputs:

- `/tmp/f64-capacity-verification/generated-scan.jsonl`
- `/tmp/f64-capacity-verification/artifact-scan.jsonl`
- `/tmp/f64-capacity-verification/edge-default-scan.jsonl`
- `/tmp/f64-capacity-verification/edge-product-facet-removal-scan.jsonl`
- `/tmp/f64-capacity-verification/comparison.jsonl`

`comparison.jsonl` is the packet output to inspect. The command prints only a
compact count of met and failed expectations.

## Interpretation

- `expectation_status = met`: the scan row matched the manifest expectation.
- `expectation_status = failed`: at least one expectation failed, or an
  unexpected scan row was present.
- `claim_scope = full`: the case expects the f64 scan row to make the checked
  local output decision without reporting a limited f64 output object. This is
  not an evidence-support label; generated rows have fresh exact audit here,
  while retained rows currently have stored-label support only.
- `claim_scope = limited`: the case expects f64 to report a limited claim, such
  as unresolved exact equality inside the near-minimum band or visible fallback.

The packet uses `f64-capacity-scan` rows as the primary observed data. Generated
rows run exact audit in this packet. Retained artifact rows are compared against
their stored labels and explicitly expect `exact_audit_status = not_requested`;
add retained exact recomputation only when the scan path supports it for the
selected row shape. Add `tracing` events only for variables that are useful for
investigation but should not become stable scan-row fields.

The first manifest covers:

- generated clean random baseline with exact audit;
- generated product tie with exact audit, where capacity is decided and the
  exact minimizing-sigma set is intentionally unresolved by f64;
- retained clean random baseline;
- retained product tie, with the same intentional f64 equality boundary;
- retained generic ascent endpoint;
- retained product-shaped ascent endpoint;
- HKO-like fallback-visible stress row;
- edge fixtures for duplicate vertices, missing origin interior, product
  rounding drift, and near-redundant product facet removal.

Edge fixtures are code-owned rows in `exp-dev-quadratic-program`; they are not a
separate status packet.
