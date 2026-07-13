# Projection reduced-gradient sign replay

This packet preserves a structured four-run case study used for the thesis
high-trust delegation question. It is a reproducible mutation check, not a new
research experiment. Its source decision is the private, local-only artifact
`experiments/ai-use/artifacts/ragged-frontier-2026-07-12/section13-q2-q4-benchmark-decision.md`,
which is intentionally absent from a clean checkout.
Frozen base commit: f3d36cc968716132af582282dbe6c137a2857ec4.

## Design and task contracts

All four isolated runs received the same known defect in
crates/library/src/kkt/projection_solver.rs. The retained-mode equation must be

    H' alpha = -V^T H beta0,

because the reduced gradient is H' alpha + V^T H beta0. The historical defect
used the positive sign. The benchmark crossed model family (GPT-5.5 or
GPT-5.6-sol) with a minimal repair contract or a verifier-first contract.

The exact spawn prompts are encrypted in the source rollout records. They are
not reproduced here and are never passed verbatim to the replay. The contracts
below are the benchmark runner's contemporaneous reconstruction:

- minimal: derive and repair the known reduced-gradient sign, with ordinary
  targeted Rust verification;
- verifier-first: derive the same equation and provide a regression that fails
  before the repair and passes after it, including a direct stationarity check.

The replay reconstructs the mathematical fixtures and discriminating
assertions of the regressions from their source rollouts. It does not claim to
reproduce the original diffs byte-for-byte or recover hidden prompt wording.

## Recorded runs and source outcomes

| Case | Model / contract | Thread ID | Recorded regression and outcome |
|---|---|---|---|
| 55-min | GPT-5.5 / minimal | 019f580d-b500-7573-ae5f-0d90472186cf | reduced_gradient_sign_gives_stationary_point; correct repair; fail-before not demonstrated |
| 55-ver | GPT-5.5 / verifier-first | 019f580e-0ce8-79c3-80de-774115351de7 | retained_modes_use_negative_reduced_gradient; correct repair; fail-before demonstrated |
| 56-min | GPT-5.6-sol / minimal | 019f580e-2677-7b12-b041-fd0854163903 | strengthened one_free_variable; correct repair; test remained sign-insensitive under independent mutation |
| 56-ver | GPT-5.6-sol / verifier-first | 019f580e-4820-73c0-ab7d-bd9824559697 | reduced_stationarity_uses_negative_gradient; correct repair; fail-before demonstrated |

The reconstructed cases use the recorded analytic points:

- 55-min: C=(1,1), H=diag(2,1), expected (1/3, 2/3);
- 55-ver: C=(1,1), H=diag(-2,-8), expected (0.8, 0.2);
- 56-min: existing H=I one-free-variable fixture, expected
  (1/6,1/6,1/6,1/6,1/6,5/6); this is intentionally nondiscriminating;
- 56-ver: C=(1,1), H=diag(1,3), expected (3/4, 1/4).

## Replay verifier

sign_replay.py is standard-library-only. For every case it:

1. creates a temporary detached worktree at the frozen base;
2. injects that case's recorded regression into the projection solver source;
3. sets the coefficient to either the bad positive or correct negative sign;
4. runs cargo test --manifest-path crates/library/Cargo.toml <test> --lib;
5. records return code, expected/observed pass, and a short output tail; and
6. removes the detached worktree in finally, including when a test or
   transformation fails.

The caller's worktree is never modified. Focused Python tests cover coefficient
replacement, marker rejection, and the eight-row dry-run contract.

Expected mutation matrix:

| Case | Bad positive sign | Correct negative sign |
|---|---:|---:|
| 55-min | fail | pass |
| 55-ver | fail | pass |
| 56-min | pass (nondiscriminating) | pass |
| 56-ver | fail | pass |

Full replay observed on 2026-07-12 from this worktree (all eight rows matched
expectation; failing Rust tests returned 101 and passing tests returned 0):

| Case | Sign | Expected | Observed | Return code | Match |
|---|---|---:|---:|---:|---:|
| 55-min | bad-positive | fail | fail | 101 | yes |
| 55-min | correct-negative | pass | pass | 0 | yes |
| 55-ver | bad-positive | fail | fail | 101 | yes |
| 55-ver | correct-negative | pass | pass | 0 | yes |
| 56-min | bad-positive | pass | pass | 0 | yes |
| 56-min | correct-negative | pass | pass | 0 | yes |
| 56-ver | bad-positive | fail | fail | 101 | yes |
| 56-ver | correct-negative | pass | pass | 0 | yes |

## Cost telemetry

The source decision records elapsed seconds and token counters (these are
telemetry, not directly comparable monetary costs):

| Condition | Elapsed | Uncached input | Cached input | Output |
|---|---:|---:|---:|---:|
| 5.5 minimal | 227 s | 58,161 | 638,976 | 4,680 |
| 5.5 verifier-first | 219 s | 38,608 | 540,416 | 4,376 |
| 5.6 minimal | 126 s | 46,311 | 603,648 | 3,106 |
| 5.6 verifier-first | 155 s | 45,420 | 708,096 | 3,846 |

## Reproduction commands

From repository root:

    python3 experiments/ai-use/sign-replay/test_sign_replay.py
    python3 experiments/ai-use/sign-replay/sign_replay.py --dry-run
    python3 experiments/ai-use/sign-replay/sign_replay.py --json

The full replay compiles the small Rust crate as needed and can take several
minutes. A successful full run exits zero and reports eight rows whose
matched_expectation field is true. --dry-run performs no checkout or Rust build.

## Limits and interpretation

This is one known-defect task, not a measure of bug-discovery ability. There
was one run per cell, no randomization or within-condition replication, and the
exact prompt text is unrecoverable. The case study therefore cannot attribute
an effect causally to verifier-first prompting or model generation. Agents were
told the defect category, and prompt structure changes both reasoning order and
required evidence. The
replay verifies that recorded regression claims have the expected sign
sensitivity on the frozen base; it does not establish a causal effect size for
verifier-first prompting.

The bounded thesis conclusion is: in this four-run case study, all agents
repaired the production sign, but one plausible regression remained insensitive
to it. Replaying each regression under both signs showed why fail-before/pass-
after mutation checks provide materially stronger evidence than an ordinary
passing suite. Do not generalize this case study to proof search, open-ended
coding, or all model generations.
