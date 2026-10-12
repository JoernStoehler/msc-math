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
Python manifest gate and adversarial tests completed without invoking the real
backend. The evaluator's fake-target Rust tests (including clean-identity and
overwrite gates) and the producer's seed-translation semantic test also pass
without invoking the real backend.

Frozen artifact identity:

- source SHA-256:
  `161f6361fd9c99b1b86a863c3cdb7db438fd76329392992f6212e37c83e69963`;
- feature SHA-256:
  `8a87ef1a050cd9b3a717c85a43b0577f9e72c308e635fcc93defed58ec8883a5`;
- selection SHA-256:
  `2e4953cc61fa3eb02405c2fff9844c842c7813fd05edb7a741413574b794a168`.

The selected/control union has 91 unique rows (five rho/ridge overlaps), with
16 memberships per arm and bucket. At this target-free production stage no
capacity, `sys`, cache lookup, bounce label, or target output was produced.
The post-target analyzer was not run on fabricated values; its
partial-artifact rejection is covered by `test_packet.py`. The separately
reviewed target and analysis are recorded below.

The frozen descriptive analysis constants are committed before exposure:
bootstrap seed `2026071602`, permutation seed `2026071603`, and 10,000
repetitions for each deterministic bucket/arm-stratified bootstrap and
within-bucket label-permutation diagnostic. The target-free handoff was later
accepted by independent result review; the exact post-target account and
analysis are preserved alongside this report.

The reviewed evaluator identity is source digest
`a810e55a82bce73b8a728d7394a576a260f80da204b689b55f7dfff89a9a451a`, lock
digest `740441674806a1baaea966d5f8f12a66d8e2ef1229b66ca9dcf9225a02f6c45f`,
backend digest
`2fb95be2f16bbd730adfcc610fdb331bfdb692a283bdfe7092bc12cfde07721b`, and
repository HEAD `5a5736687dcd8ad10f4a682266fa24d1fe067efc`; the accepted
evaluation used a detached clean worktree at that commit.

## Accepted post-target result

The exact 91-row target artifact has SHA-256
`6016b66c5cad4af948b6d0188ccfa5f1d455b10093e5e2f61d303401fc0082f5`; the
frozen analysis has SHA-256
`872b932cf38811184104a6bf46afe34f079c291fa5b6e9bc90e05a80df1d407a`.
Both selectors received the predeclared `strong_transfer` label as finite
sub-threshold enrichers on the one `factorial-both` source across `4x6` and
`6x6`. The analysis JSON owns the detailed bucket table, exact effects,
descriptive bootstrap ranges, and permutation counts. These diagnostics are
not population confidence intervals, `p = 0`, or causal randomization
evidence.

The result has shared controls, five rho/ridge overlaps, one master seed, two
fixed buckets, and zero `sys > 1` rows. It does not support threshold,
mechanism, causal, population, superiority, theorem, or counterexample claims.
The packet is closed with no automatic follow-up queued; see
`POST-TARGET-ACCOUNT.md` and `artifacts/transfer-v1/result-manifest.json` for
the complete bounded account and provenance.
