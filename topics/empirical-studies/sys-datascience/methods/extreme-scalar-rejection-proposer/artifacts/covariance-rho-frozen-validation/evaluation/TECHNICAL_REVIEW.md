# Frozen covariance-rho validation: technical/provenance review

Status: **GO as a same-generator prospective selector result; no escalation.**

## Frozen inputs and execution

- Combined pre-target manifest:
  `frozen-packet/frozen-selected-candidates-before-sys.jsonl`, SHA-256
  `0f3dcb1a518f864d9e4ba76e7471a87e2ce4c070bcff103247c7732da688438a`.
- It contains exactly 1,436 unique candidate IDs.  Each arm has 500
  memberships, rho/ridge overlap is 64, and control is disjoint.
- Evaluation used only symlinked, read-only frozen per-seed geometry caches,
  selection manifests, and plans.  It did not generate geometry, features, or
  a selection.  Fresh target caches are `seed-2026071201/` and
  `seed-2026071202/` in this directory.
- The target caches contain 719 and 717 rows respectively, 1,436 unique rows
  overall.  The manifest-driven reader verified every candidate ID, `poly_id`,
  bucket, arm membership, selection feature/direction/value, and rule values;
  no extra, duplicate, missing, or nonfinite evaluated row was accepted.
- The target-free key-path audit remains the pre-target evidence: it covered
  both direct feature tables and the combined manifest, forbidding `sys`,
  `capacity`, `bounce`, `target`, and `min_action` key paths.  Its SHA-256 is
  `cd507fbceba59b1afa968e12a80ce80743e9b2e98d0d1fb36ecef29267cc87fb`.

## Results and frozen criteria

The frozen verdict is
`verdict/covariance-rho-validation-verdict.json`, SHA-256
`aa4337006552a0628a5096e5ae0b11a1b4de8c385610bb0e68609c9f37026acb`.

- Rho minus control: `0.33312314750128585`, two-sided 95% interval
  `[0.30974801675570574, 0.356498278246866]`.
- Both seed aggregates are positive (`0.3265899253321042`,
  `0.33965636967046753`) and all ten seed-pooled bucket effects are positive.
  Thus the primary frozen success rule passes.
- Rho minus ridge: `0.014487995580657193`, two-sided 95% interval
  `[-0.014925556344788319, 0.0439015475061027]`.  Rho is competitive under
  the predeclared `-0.05` lower-bound rule, but is not better than ridge.
- Ridge minus control is `0.31863515192062863`, two-sided 95% interval
  `[0.27923967060072663, 0.35803063324053064]`; the positive-control
  replication is present.
- No row has `sys > 1`; the global maximum is `0.9002763913390446`.
  Therefore independent candidate escalation is not triggered.

## Cost and boundary

The two runner wall times were 56.930 and 58.711 seconds at two worker threads.
The sum of recorded per-row capacity times is 117.904438626 seconds; all 1,436
capacities and sys values are finite, with bounces between 2 and 3.

This is prospective evidence that the frozen low-rho rule enriches `sys` on the
specified fresh random-product height law.  It is not a capacity theorem, a
mechanism result, a direction-flip license, or evidence for transfer outside
that generator.  It does not support saying rho beats ridge.
