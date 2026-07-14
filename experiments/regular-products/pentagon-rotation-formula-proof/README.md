# Pentagon Rotation Formula Proof

This folder owns the exact executable proof for the formula for

```text
sys(P_5 x_L R(theta)P_5).
```

The sibling folder `../rotated-regular-products/` owns the sampled sweep and
profile figure used by the chapter.  `../pentagon-rotation-empirics/` owns
orbit and branch diagnostics.  These empirical artifacts are useful for
intuition and thesis exposition, but they are not proof inputs.

Local filenames below are relative to this folder. Paths outside this folder
are repo-root relative unless they begin with `../`.

## Source Files

```text
executable_proof.sage.py
```

Exact SageMath executable proof. The default invocation runs the full
certificate. `--limit N` runs the same assertions on a prefix.

The executable refuses to run when Python assertions are disabled. Its output
header records the SageMath version, assertion state, source SHA-256 digest,
and command arguments used for the run.

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
2. To inspect the proof code, read `executable_proof.sage.py`.
3. To write thesis prose, use
   `thesis/rotated-regular-polygons-content.md`.
4. Do not open empirical JSONL/PNG/HTML artifacts for proof verification.

## Proof Surface Routing

Use these files for different questions:

| Question | Source |
| --- | --- |
| What is the executable proof? | `executable_proof.sage.py` |
| What did the full proof run print? | `executable_proof.full.stdout.txt` |
| How should the thesis explain the proof architecture? | `thesis/rotated-regular-polygons-content.md` |
| Where is older formal source material? | `formal/pentagon-rotation-capacity.tex`, treated as stale source material |
| Where are empirical figures and viewer artifacts? | `../rotated-regular-products/` for the profile sweep; `../pentagon-rotation-empirics/` for orbit and branch diagnostics |

The formal file is useful for earlier notation and active-branch material, but
the current lower-bound proof source is the executable Sage certificate.

## Commands

Exact proof prefix:

```bash
PYTHONOPTIMIZE=0 sage -python experiments/regular-products/pentagon-rotation-formula-proof/executable_proof.sage.py --limit 50
```

Exact full proof:

```bash
set -o pipefail
PYTHONOPTIMIZE=0 sage -python experiments/regular-products/pentagon-rotation-formula-proof/executable_proof.sage.py --progress-every 500 \
  | tee experiments/regular-products/pentagon-rotation-formula-proof/executable_proof.full.stdout.txt
```

## Full-Run Result

The raw stdout artifact begins with the run-identity header described above
and ends with a summary of this form:

```text
open_domain_raw_sigma_count = 3340
classified_raw_sigma_count = 3340
classification_statuses = {'no_kkt_solution': 25, 'zero_q_identity': 1680, 'singular_kkt_forced_zero_beta': 470, 'not_feasible_on_open_domain': 735, 'zero_gap_identity': 20, 'strict_gap_positive_on_feasible_open_domain': 410}
CERTIFICATE PASSED in <elapsed>s
```

Read the stdout artifact for the current header, counts, runtime, and final
success line. The elapsed time changes on regeneration.
