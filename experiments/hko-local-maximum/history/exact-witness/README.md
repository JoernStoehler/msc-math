# Exact Witness Execution Notes

This directory contains the runnable tooling and generated artifacts for the
older exact representative route. It is history and fallback material, not the
current theorem certificate.

Research interpretation, status, blockers, and theorem-facing notes live in
`research/hko-local-maximum-exact-witness.md`.

The scripts in this directory write tracked JSON artifacts next to the scripts.
Do not run them as casual smoke checks. Run them only when intentionally
refreshing HKO exact witness evidence or checking a specific witness artifact.

Typical local commands:

```bash
cd experiments/hko-local-maximum/history/exact-witness
python3 build_widened_representative_witness.py
sage verify_widened_representative_witness.sage
```

These commands overwrite `widened-representative-witness.json` and
`widened-representative-witness-verification.json`.
