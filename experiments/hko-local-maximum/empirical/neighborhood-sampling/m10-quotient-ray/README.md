# HKO 25-dimensional quotient-ray panel

This directory retains the accepted seed-44 frozen panel produced by
`m10-quotient-ray` at commit
`03d8322557af0f4f54c6b5ce9bed431e46d87fc4` (tree
`532fc031cbd337ec27d801bbef9ff85e60285e6d`). The exact launch input is
`launch-packet.json`; its SHA-256 is
`75ae5b6da56d37d02bedc241861fb66a130234475161003431d006d66674e79b`.
The artifact-bundle root is
`6fa5260e92576c2513c0a8d5583130bdddb5bacdfe7a19a972872a3d7113e1b5`.

## Accepted observation

For the 32 seed-44 directions sampled uniformly from the declared Euclidean
unit sphere in the retained 25-dimensional affine slice, every ray has a
sampled, route-resolved nominal above-to-below `sys` transition. The transition
midpoints range from `0.0438428` to `0.112061`, with median `0.0784326`. Every
positive-radius random evaluation has nominal chart, gauge, and evaluator
labels. No retained shell, bisection row, or one of the four post-transition
probes per ray shows nominal re-entry. All seven controls passed.

The route interval is indeterminate at HKO itself but resolves above one by
radius `1e-4` on all 32 random rays. On every random ray, the route-supported
above-to-below bracket equals the nominal transition bracket endpoint for
endpoint. This is evidence that route-interval indeterminacy does not dominate
generic positive-radius states in this panel.

This result is a finite, measure-dependent mechanism/readiness screen. It does
not establish a positivity inradius, star-shapedness, trapping, a population
frequency, the first mathematical exit on a ray, absence of thin tubes or
lower-dimensional exceptional sets, or a mathematical capacity certificate.
The pointwise chart checks do not certify entire segments. The older retained
product-plane panel used a different measure and gauge, so differences between
the two radius distributions are not quotient-versus-product effects.

## Artifact roles

- `launch-packet.json`: reviewed frozen settings and source/tree hashes.
- `manifest.json`: execution and provenance record.
- `basis.json`: the 40-by-15 symmetry-orbit basis and 40-by-25 orthogonal
  complement used as the local affine slice.
- `controls.jsonl`: HKO value, backend, exact finite-symmetry, and nonlinear
  rotated-pentagon controls.
- `evaluations.jsonl`: all 1,171 pointwise evaluations.
- `transitions.jsonl`: 101 route-side and nominal transition brackets.
- `ray-outcomes.jsonl`: per-ray aggregation for 32 random rays and two
  deterministic sentinels.
- `summary.json`: counts and completion state.
- `artifact-bundle.json`: BLAKE3 identities for the seven generated artifacts.

The generated bundle does not itself contain `launch-packet.json`; this
directory preserves that packet beside the bundle. The manifest binds its raw
BLAKE3 and effective contents.

## Reproduction

From the clean recorded commit, use the command documented in the parent
`README.md`, with this retained packet and a fresh empty output directory. The
recorded wall time was 1,419.31 seconds for 1,171 capacity evaluations. The
build provenance is internally consistent but is not a machine attestation
that every linked dependency object came from the recorded source trees.
