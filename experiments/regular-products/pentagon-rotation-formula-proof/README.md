# Pentagon Rotation Formula Proof

This folder owns the exact executable proof and code-audit guide for the
formula for

```text
sys(P_5 x_L R(theta)P_5).
```

The sibling folder `../pentagon-rotation-empirics/` owns sampled data, figures,
and the interactive viewer. Those empirical artifacts are useful for intuition
and thesis exposition, but they are not proof inputs.

Local filenames below are relative to this folder. Paths outside this folder
are repo-root relative unless they begin with `../`.

## Source Files

```text
executable_proof.sage.py
```

Exact SageMath executable proof. The default invocation runs the full
certificate. `--limit N` runs the same assertions on a prefix.

```text
executable_proof.full.stdout.txt
```

Raw stdout from the full no-limit proof run. This file is the durable source
for exact run output and status counts. Do not hand-edit it; regenerate it by
rerunning the full proof command.

If `executable_proof.sage.py` changes, rerun the full proof and replace this
stdout file before using it as evidence.

## Read Path

1. To check the proof result, read this README and
   `executable_proof.full.stdout.txt`.
2. To audit the code, read `executable_proof_audit_guide.md`, then open only
   the Sage functions named there.
3. To write thesis prose, use
   `thesis/rotated-regular-polygons-content.md`.
4. Do not open empirical JSONL/PNG/HTML artifacts for proof verification.

## Audit Notes

```text
executable_proof_audit_guide.md
```

Code-audit guide. It is not source truth and not thesis prose.

## Proof Surface Routing

Use these files for different questions:

| Question | Source |
| --- | --- |
| What is the executable proof? | `executable_proof.sage.py` |
| What did the full proof run print? | `executable_proof.full.stdout.txt` |
| How should I audit the Sage source? | `executable_proof_audit_guide.md` |
| How should the thesis explain the proof architecture? | `thesis/rotated-regular-polygons-content.md` |
| Where is older formal source material? | `formal/pentagon-rotation-capacity.tex`, treated as stale source material |
| Where are empirical figures and viewer artifacts? | `../pentagon-rotation-empirics/` |

The formal file is useful for earlier notation and active-branch material, but
the current lower-bound proof source is the executable Sage certificate.

## Commands

Exact proof prefix:

```bash
sage -python experiments/regular-products/pentagon-rotation-formula-proof/executable_proof.sage.py --limit 50
```

Exact full proof:

```bash
sage -python experiments/regular-products/pentagon-rotation-formula-proof/executable_proof.sage.py --progress-every 500
```

## Full-Run Result

The current raw stdout artifact records the cached excerpt:

```text
open_domain_raw_sigma_count = 3340
classified_raw_sigma_count = 3340
classification_statuses = {'no_kkt_solution': 25, 'zero_q_identity': 1680, 'singular_kkt_forced_zero_beta': 470, 'not_feasible_on_open_domain': 735, 'zero_gap_identity': 20, 'strict_gap_positive_on_feasible_open_domain': 410}
CERTIFICATE PASSED in 2126.34s
```

After the CLI cleanup, prefix checks passed with `--limit 5` and `--limit 50`.
