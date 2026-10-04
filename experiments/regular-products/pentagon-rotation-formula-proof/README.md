# Pentagon Rotation Formula Proof

This folder retains the exact executable component of the original
computational proof for the formula for

```text
sys(P_5 x_L R(theta)P_5).
```

The current thesis proves the rotation formula analytically in
[Chapter 9](../../../thesis/chapters/09-rotated-regular-polygons.tex).
[Chapter 10](../../../thesis/chapters/10-affine-pentagons.tex) gives the
independent-linear-deformation extension. Neither argument depends on this
certificate or on rerunning it. This packet preserves the earlier route;
the reduction, sign-to-action implications and continuity argument remain
mathematical inputs to that route.

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

Exact SageMath classifier for the original proof route. The default invocation runs the full
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

1. To inspect the original computational result, read this README and
   `executable_proof.full.stdout.txt`.
2. To inspect the proof code, read `executable_proof.sage.py`.
3. For the current proof, read
   `thesis/chapters/09-rotated-regular-polygons.tex`; the selected build is
   documented in `thesis/README.md`.
4. Do not open empirical JSONL/PNG/HTML artifacts for proof verification.

## Proof Surface Routing

Use these files for different questions:

| Question | Source |
| --- | --- |
| What is the executable proof? | `executable_proof.sage.py` |
| What did the full proof run print? | `executable_proof.full.stdout.txt` |
| Where is the current analytic rotation proof? | `thesis/chapters/09-rotated-regular-polygons.tex` |
| Where is the independent-linear-deformation proof? | `formal/pentagon-affine-products/research.tex`, with thesis exposition in `thesis/chapters/10-affine-pentagons.tex` |
| Where does the thesis describe the original certificate and its trust boundary? | `thesis/chapters/14-code-data.tex` |
| Where is older formal source material? | `formal/legacy/pentagon-rotation-capacity.tex`, treated as stale source material |
| Where are empirical figures and viewer artifacts? | `../rotated-regular-products/` for the profile sweep; `../pentagon-rotation-empirics/` for orbit and branch diagnostics |

The legacy formal file is useful for earlier notation and active-branch
material. The maintained analytic source and proof-route comparison are in
`formal/pentagon-affine-products/README.md`. The certificate is the finite
computational component of the historical lower-bound route, not an input to
the current analytic proof.

## Commands

Exact proof prefix:

```bash
PYTHONOPTIMIZE=0 conda run --name sage python experiments/regular-products/pentagon-rotation-formula-proof/executable_proof.sage.py --limit 50
```

Exact full proof:

```bash
set -o pipefail
PYTHONOPTIMIZE=0 conda run --name sage python experiments/regular-products/pentagon-rotation-formula-proof/executable_proof.sage.py --progress-every 500 \
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
