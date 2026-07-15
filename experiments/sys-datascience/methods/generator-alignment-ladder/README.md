# Target-free SO(4) alignment dose ladder

This packet asks a narrow geometry question: on the eight retained-orientation
base identities, what changes under the controlled family
`R_theta = U1 A_theta U2` in `SO(4)`?  It is designed to decide whether a
later target ladder is worth exposing, not to make a capacity or `sys` claim.

## Transformation and scope

Coordinates are `(q1,q2,p1,p2)` and matrices act on primal points.  Dual
normals therefore use the inverse transpose.  With

```text
Q(theta) = [[cos(theta), -sin(theta)], [sin(theta), cos(theta)]]
A_theta = diag(Q(theta), I_2),
```

the five arms use `theta = 0, pi/4, pi/2, 3pi/4, pi`.  Thus `A_0 = I`, while
`A_pi = diag(-1,-1,1,1)` is orientation-preserving, orthogonal, and
anti-symplectic.  `U1` and `U2` are independently seeded Haar `U(2)` draws,
but are held fixed across the five angles of each base.  The declared
Kähler-departure coordinate is `sin^2(theta/2)`: it is a named dose coordinate,
not a distance on a quotient or a capacity coordinate.

The producer copies the documented `generator-orientation-smoke` source base
contract bit-for-bit, including area-normalization operation order (seed
`20260714`, 128 bounded attempts, two bases in each of `3x3,4x4,4x6,6x6`).
Its report binds the source orientation report revision
`8174467dbd171281eb5746480b06629aa41ebfa7` and raw-row LFS object
`sha256:b5ded0a5e83d41f35ca035660d222326a161ce5001fd18c12f74f0ed9f3bc367`.
The retained analysis hydrates that pinned object and verifies all eight base
IDs and exact geometry IDs. In a checkout without the object, the analyzer
fails closed if asked to make this comparison; do not reuse the old panel's
identity until it has passed again.

Run the full compact panel from the repository root after committing source:

```bash
CARGO_TARGET_DIR=/workspaces/msc-math/target cargo run -p exp-sys-landscape --release \
  --bin sys-datascience-generator-alignment-ladder -- \
  --out-dir experiments/sys-datascience/methods/generator-alignment-ladder/artifacts/panel
uv run --script experiments/sys-datascience/methods/generator-alignment-ladder/analyze.py \
  --rows experiments/sys-datascience/methods/generator-alignment-ladder/artifacts/panel/rows.jsonl \
  --report experiments/sys-datascience/methods/generator-alignment-ladder/artifacts/panel/report.json \
  --orientation-rows experiments/sys-datascience/methods/generator-orientation-smoke/artifacts/panel-2-per-bucket/rows.jsonl \
  --out-dir experiments/sys-datascience/methods/generator-alignment-ladder/artifacts/panel
```

`rows.jsonl` has every requested base/angle row or a terminal fail-closed
status.  It retains matrix contracts, exact reconstruction/incidence/volume
checks, raw/euclidean/symplectic dual-Gram signatures, and paired controls.
`report.json` freezes the producer source closure before outputs are written.
`analysis.json`, `paired-by-base.tsv`, and `paired-by-theta.tsv` are generated
from those rows and classify only this finite panel's response shapes. These
retained artifacts deliberately contain no wall-clock fields: the rows, report,
analysis, and paired tables are deterministic for a fixed clean producer
revision. The report records the stable repo-root Cargo reproduction command,
never an absolute binary path.

## Interpretation

Passing rows establish the stated finite-panel geometry and reconstruction
contracts.  Euclidean response signatures are controls and must remain fixed
under all orthogonal arms; a change is a packet failure.  Symplectic signatures
are descriptive post-reconstruction responses.  The analysis labels a response
as monotone, reverse-symmetric, endpoint-controlled, or multi-directional only
as a property of the retained five-angle sequence; it does not rank laws,
estimate a population effect, or show a capacity dose-response.

No capacity backend, target field, `sys` value, or target-derived selection is
used.  This family is not claimed to exhaust or uniquely parameterize
`U(2) \\ SO(4) / U(2)`.  Whether that double-coset statement is appropriate,
and whether anti-symplectic maps preserve the capacity used later, are explicit
proof-review cruxes recorded in the report rather than assumptions of this
packet.

## Current compact panel

The committed `artifacts/panel/` run contains all 40 requested rows and passes
the formula controls. All eight bases retain exact reconstruction and labeled
incidence; maximum relative volume change is `3.78e-15`, and the Euclidean
dual-Gram control changes by at most `5.69e-14`. The mean direct symplectic
Gram response norm across bases is approximately `0`, `33.9`, `61.1`, `73.0`,
and `72.6` at the five increasing angles. Six of eight finite base sequences
are non-decreasing and endpoint-controlled under this scalar norm; none is
reverse-theta symmetric under the declared strict comparison; all eight have
non-collinear successive response vectors. These are finite-panel
classification facts, not a monotone capacity result or a general statement
about `SO(4)`.

Disposition: this supports a later decision about whether a small *target*
ladder is worth exposing, provided the double-coset and anti-symplectic
capacity proof cruxes have been reviewed. It does not itself authorize that
target exposure.
