# Target-free run report

Run from clean producer commit `fcd5546af014942b74a1e9313ee898329a507d3d`
with the release binary built by
`cargo build --release --manifest-path experiments/sys-landscape/Cargo.toml
--bin sys-datascience-alternative-source-transfer`.

Generation command:

```text
/usr/bin/time -v target/release/sys-datascience-alternative-source-transfer \
  produce /tmp/alternative-source-transfer-full
```

The deterministic producer scanned 4,000 row indices per bucket, retained
3,200 rows in each bucket, and completed in 752.79 wall seconds and 8,173.12
user CPU seconds (26.02 system seconds; 1,088% aggregate CPU; peak RSS
200,620 KiB). It wrote 6,400 rows and no target fields. `features`, `select`,
and `validate` then completed in under one second plus process startup; the
Python manifest gate and seven self-tests completed in under one second.

Frozen artifact identity:

- source SHA-256:
  `161f6361fd9c99b1b86a863c3cdb7db438fd76329392992f6212e37c83e69963`;
- feature SHA-256:
  `8a87ef1a050cd9b3a717c85a43b0577f9e72c308e635fcc93defed58ec8883a5`;
- selection SHA-256:
  `2e4953cc61fa3eb02405c2fff9844c842c7813fd05edb7a741413574b794a168`.

The selected/control union has 91 unique rows (five rho/ridge overlaps), with
16 memberships per arm and bucket. No capacity, `sys`, cache lookup, bounce
label, or target output was produced. The post-target analyzer was not run on
fabricated values; its partial-artifact rejection is covered by
`test_packet.py`.

