# Pentagon Rotation Executable Proof Plan

Provisional planning note for the SageMath executable proof spike.

This is not a final decision record and not thesis prose. It records the
current reasoning so that later work can pivot without rediscovering why this
route was tried.

## Goal

Close the non-writing part of the formula for

```text
sys(P_5 x_L R(theta) P_5)
```

on the fundamental domain.

The intended final state before thesis-writing is:

```text
1. A readable SageMath script encodes the finite proof obligation.
2. The script exits successfully only if all proof assertions pass.
3. Every raw 2- or 3-bounce sigma is classified.
4. The output prints only formulas and counts that humans should inspect.
5. Formal/research notes can cite the executable proof and record the remaining
   Jörn/Kai review gate.
```

## Current Proof Shape

The proof is intended to be an executable exact certificate.

For a fixed sigma, Sage constructs the KKT matrix over

```text
Frac(K[t]),  K = maximal totally real subfield of CyclotomicField(20),
t = tan(theta / 2).
```

Then Sage computes exact rational functions:

```text
det(M_sigma)(t)
beta_sigma(t)
Q_sigma(t)
action_sigma(t) = 1 / (2 Q_sigma(t))
gap_sigma(t) = action_sigma(t) - action_min(t)
```

The open half-domain is

```text
0 < t < tan(pi/20).
```

Sign checks use exact algebraic root isolation after converting coefficient
polynomials to Sage's real algebraic field `AA`. The endpoint
`tan(pi/20)` naturally lives in the real subfield of `CyclotomicField(40)`,
or in `AA`.

## Why This Field Choice

`QQ` is not sufficient because the unrotated regular pentagon already contains
constants such as `cos(k*pi/10)` and `sin(k*pi/10)`.

The real subfield of `CyclotomicField(20)` contains the pentagon constants and
has small degree. It is enough for symbolic branch expressions because the
rotation is represented by `t = tan(theta/2)`, so `sin(theta)` and
`cos(theta)` are rational functions in `t`.

`CyclotomicField(40)` is not needed for KKT expression construction. It is
only needed, or replaceable by `AA`, for exact endpoint/sign checks involving
`tan(pi/20)`.

## Source Code As Proof

The intended readable artifact is the source code plus selected formulas of
interest, not a giant printed table of `True` values.

The script should fail closed by using assertions:

```python
assert len(unclassified) == 0
assert intended_action == expected_action
assert gap_certificate.status in allowed_statuses
```

If a proof obligation fails, the script should exit nonzero.

Default output should be compact:

```text
action_min(theta) = ...
sys(theta) = ...
raw_sigma_count = ...
classified_count = ...
strict_gap_positive = ...
endpoint_tie_only = ...
infeasible_on_open_domain = ...
CERTIFICATE PASSED
```

Long per-sigma formulas should be behind a verbose or dump option, not default
output.

## Proof Index: Raw Sigmas First

The first complete certificate should use raw sigmas as the proof index.

Reason:

```text
raw sigma -> KKT system -> exact branch expression -> exact sign certificate
```

is the shortest trust path.

Canonical affine signatures are useful for human reports, but using them as
the proof index adds an extra proof obligation: one must prove the
canonicalization preserves feasibility, action gaps, endpoint behavior, and
all relevant degeneracies. That may become useful for performance, but should
not be the first proof route unless raw-sigma runtime is too high.

## Alternatives Considered

1. Sage enumerates and certifies everything.
   - Clean single proof surface.
   - Risk: reimplementing enumeration can drift from Rust.

2. Rust exports raw sigmas, Sage certifies them.
   - Reuses Rust enumeration and keeps Sage focused on exact algebra.
   - Requires explicit convention checks.

3. Canonical signatures first.
   - Smaller and more readable.
   - Risk: canonicalization becomes part of the proof.

4. Manual branch formulas.
   - Good for formulas of interest and debugging.
   - Bad as the primary exhaustive proof route.

## Early Sanity Checks

The executable proof should include small checks before the full run:

```text
1. Intended 2-bounce sigma action equals (1 + cos(pi/5))^2 sec(theta).
2. The resulting systolic ratio prefactor is (5 + 2 sqrt(5)) / 10.
3. Sage dual vertices and adjacency match the Rust/analyze facet convention.
4. Imported or enumerated raw sigma count matches the Rust producer.
5. Selected 3-bounce preflight branches have the expected positive open gap or
   identity status.
6. Every sigma receives exactly one final status.
```

## Current Development State

The current preferred route is direct Sage enumeration plus direct Sage
certification. This keeps the executable proof to one source file. The
enumeration drift risk is addressed by hard count checks against the existing
Rust/analyze output:

```text
structural 2-bounce sigmas: 7200
structural 3-bounce sigmas: 43200
open-domain transition-pruned raw sigmas: 3340
```

The transition pruning is not sampled in the proof script. It uses exact sign
certificates for all ordered facet-pair symplectic products on
`0 < t < tan(pi/20)` and asserts that mixed transition signs have no interior
roots. Same-factor products are identically zero.

Every invocation currently begins with the same preflight:

```text
minimum action branch;
systolic-ratio prefactor;
transition-sign constancy;
open-domain raw sigma count 3340;
representative strict-gap, zero-gap, zero-Q, and singular statuses;
```

After preflight, the default run classifies all 3340 raw sigmas. A development
run with `--limit N` classifies only the first `N` raw sigmas but uses the same
classification logic and accepted status set. Only an unlimited run can print
`CERTIFICATE PASSED`.

Observed local timings:

```text
historical 50-prefix run, original coarse statuses: 13.65s
historical 50-prefix run, stricter feasible-cell statuses: 14.87s
historical 100-prefix run, original coarse statuses: 24.78s
historical 500-prefix run, original coarse statuses: 143.87s
historical 500-prefix run, stricter feasible-cell statuses: 154.94s
pre-CLI-cleanup full run, all 3340 sigmas, stricter feasible-cell statuses: 2025.02s
post-CLI-cleanup --limit 5 prefix: 4.08s
post-CLI-cleanup --limit 50 prefix after transition-table cleanup: 14.09s
```

Full run status counts:

```text
no_kkt_solution: 25
zero_q_identity: 1680
singular_kkt_forced_zero_beta: 470
not_feasible_on_open_domain: 735
zero_gap_identity: 20
strict_gap_positive_on_feasible_open_domain: 410
```

The pre-CLI-cleanup unlimited run printed `CERTIFICATE PASSED`. The current
default command uses the same stricter classification logic, always runs the
representative preflight first, and prints `CERTIFICATE PASSED` only if no
`--limit` is supplied.

The earlier full run in `1902.57s` is superseded. It accepted a coarse
`not_strictly_feasible_open` status, which proved only that a branch was not
feasible on the whole interval. The current script cuts the open interval at
all beta, `Q`, and gap zeros/poles and checks every feasible cell.

Only an unlimited run may print `CERTIFICATE PASSED`; `--limit N` prints
`LIMITED PREFIX PASSED`.

## Performance Policy

Do not optimize first. Measure first.

A long final proof run is acceptable if it is clear and reproducible. A runtime
on the order of hours can be acceptable for a final theorem certificate.

Initial instrumentation should record per-sigma:

```text
determinant time
solve time
root/sign time
numerator and denominator degrees
interior root counts
final classification status
```

If the raw-sigma proof is too slow, then add caching or symmetry
canonicalization with a separate audit table showing how every raw sigma maps
to a certified representative.

## Open Decisions

1. Whether canonicalization is needed for performance.
2. What exact output files, if any, should be tracked as certificate summaries.
3. How much of the certificate summary should be copied into `formal/` versus
   kept in `experiments/` with a formal reference.
