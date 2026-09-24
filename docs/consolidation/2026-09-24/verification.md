# Verification evidence

The environment-independent source archive was hashed before editing. The
following fresh checks were run with `uv run --with sympy==1.14.0`:

```sh
(cd experiments/hko-local-maximum/non-hko && \
  uv run --with sympy==1.14.0 python verify.py \
    --output-dir /tmp/non-hko-exact)
(cd experiments/hko-local-maximum/non-hko && \
  uv run --with sympy==1.14.0 python code/independent_audit.py \
    --source . --rerun /tmp/non-hko-exact \
    --output /tmp/non-hko-independent.json)
(cd docs/empirical-viterbo-design/desk/pentagon-arbitrary-partner/local-cover && \
  uv run --with sympy==1.14.0 python verify_local_cover.py \
    --fixture-code pentagon_fixture.py --out /tmp/local-cover-check.json)
```

The archived `verify_integrity.py` also passed all 55 internal file hashes;
that is archive integrity evidence, not mathematical validity.

Results:

- The submitted non-HKO verifier passed all exact checks: 10 genuine facets,
  24 vertices, volume (1/2), capacity 1, 624 product cases, 24 sections,
  gradient rank 22, symmetry rank 15, two equality parameters modulo symmetry,
  full chart rank 40, and remaining upper-section curvature (-13/432).
- The independent checker passed using a 48-simplex product triangulation,
  960 direct gradient entries, independent symmetry/full-chart checks, the
  same 624-case product enumeration, and all 24 exact exceptional-line section
  identities. It states explicitly that this does not compute the actual
  capacity away from the equality family.
- The local-cover check passed its finite certificate: ordered
  (mathbb Q(sqrt5)) arithmetic, 10 triangles, translation quotient
  dimension 18, constraint rank 28, gradient-difference rank 18, and minimum
  positive marginal (33/218-39sqrt5/1090). Its continuum implications are
  therefore retained only as a research note.

The exact summaries and console logs are retained beside their producers in
`experiments/hko-local-maximum/non-hko/verification/` and
`docs/empirical-viterbo-design/desk/pentagon-arbitrary-partner/local-cover/verification/`.

The independent packet checks were run in this coordinator shell, not by live
Herdr subagents: `HERDR_ENV` was unset, so the Herdr rules prohibited control
of Jörn's focused session. This is reported as a tooling limitation, not
presented as subagent review.
