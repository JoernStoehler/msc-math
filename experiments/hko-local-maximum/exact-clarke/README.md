# Exact Clarke Execution Notes

This directory contains the runnable exact-Clarke tooling and generated
artifacts for the HKO local-maximality route.

Research interpretation, status, blockers, and theorem-facing notes live in
`research/hko-local-maximum-exact-clarke.md`.

Typical local commands:

```bash
cd experiments/hko-local-maximum/exact-clarke
python3 build_widened_seed_witness.py
sage verify_widened_seed_witness.sage
```
