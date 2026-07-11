# f64 Capacity Verification

This packet is a small rerunnable f64 capacity verification surface. It
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
  not an evidence-support label; generated rows have a fresh reference-route
  comparison label, while retained rows have stored artifact labels only.
- `claim_scope = limited`: the case expects f64 to report a limited claim, such
  as unresolved exact equality inside the near-minimum band or visible fallback.

The packet uses `f64-capacity-scan` rows as the primary observed data. For
requested generated or preprocessed rows, the producer first interprets the
stored binary64 coordinates as rationals and validates origin interior and
facet extremality exactly. It then runs `capacity_auto`, whose route selection
and candidate generation include binary64 computation before exact action
aggregation. Consequently `reference_route_capacity_success` supplies a fresh
same-row comparison label, not an exact-capacity oracle or certificate.

`exact_geometry_validation_status` in `comparison.jsonl` records the exact
geometry-validation fact separately. Retained artifact rows are compared with
stored labels and explicitly expect `exact_audit_status = not_requested`.
`comparison_label_kind` distinguishes fresh reference-route labels, stored
artifact labels, and rows without a label. The scan schema's legacy
`abs_action_error` and `rel_action_error` fields compare with whichever label is
present; `comparison.jsonl` therefore exposes the latter as
`comparison_label_rel_difference`, not as exact-capacity error.

The first manifest covers:

- generated clean random baseline with a fresh reference-route label;
- generated product tie with a fresh reference-route label, where capacity is
  decided and the true minimizing-sigma set remains unresolved by f64;
- retained clean random baseline;
- retained product tie, with the same intentional f64 equality boundary;
- HKO-like fallback-visible stress row;
- edge fixtures for duplicate vertices, missing origin interior, product
  rounding drift, and near-redundant product facet removal.

Edge fixtures are code-owned rows in `exp-dev-quadratic-program`; they are not a
separate status packet.

The former fixed-F generic and product ascent endpoint cases are deliberately
absent. The sys-datascience owner removed those artifacts from its active
producer contract and marks them as archaeology-only; this packet does not
depend on deleted, untracked inputs.
