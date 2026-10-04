# Independent production-capacity anchors

Observed 4 October 2026, 01:35 UTC. Root owns the implementation and integration;
the independent acceptance owner reviews the mathematical oracle and assertions.

`crates/symplectic/tests/production_capacity_anchors.rs` exercises the public
`capacity_from_dual_vertices` entry point with four tests and fourteen fixed
bodies. Expected values come from planar rectangle areas, conformality and
symplectic invariance, independently of the candidate enumeration and KKT
implementation being tested:

- Two axis-aligned boxes have capacities 4 and 8, with exact rational product
  outputs checked. Coordinates are ordered `(q1,q2,p1,p2)`.
- Three uniform scalings of the unit cube have capacities 1, 16 and 64.
- Eight integer symplectic shears preserve the two box values and force the
  public dispatcher onto its general route. The shear is `S(q,p)=(q+Bp,p)`,
  with symmetric `B=[[s,s],[s,0]]`, for `s=-2,-1,1,2`; dual rows use `S^-T`.
- Integer dual rows `+/-3e_i` define `[-1/3,1/3]^4` and give exact capacity
  `4/9`. This also demonstrates that dyadic input does not imply dyadic
  reconstructed primal coordinates or dyadic capacity output.

All supplied binary64 coordinates are exact. Every result must enclose its
known value with width at most `1e-10` times that value; merely returning a
broad containing interval cannot pass. These finite fixtures do not prove
all-input correctness, cover every numerical guard, or reconstruct orbits.

Command: `cargo test -p symplectic --release --test production_capacity_anchors`.
Result: **4 passed, 0 failed, 0 ignored**, runtime 6.06 seconds, retained in
`production-capacity-anchors.log`. The test file was then formatted without
changing behavior. `docs/algorithm-testing.md` distinguishes this bounded
production suite from the broader retained legacy campaign.

The `4/9` example motivated narrow corrections to the crate README, development
contract, geometry/API comments and Chapter 13. Exact rational outputs remain
guaranteed where stated; the exact dyadic qualifier applies to the supplied
binary64 rows. No capacity algorithm was changed in this continuation.
