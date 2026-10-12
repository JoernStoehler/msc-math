# Locked weighted-W second derivative

Selected narrow gate, actual production start 2026-10-05 01:42:44 UTC.
Exact coefficient retained at 01:47 UTC; independent review is owned by
[review.md](review.md), and integration by [report.md](report.md).
No alternative mixture, new weight jet, equality-family continuation or
ambient remainder argument is claimed.

```bash
uv run --script docs/empirical-viterbo-design/night-20261005/extremality-discovery/pstar-f11/general-facet-response/weighted-w-gate/replay.py
```

The read-only replay reconstructs the actual locked 21 fixed-spectator sections
and, separately, the inherited 24 accepted source germs. Their physical normal
path is exactly `A(z)=A0+zW0`, with supports one. W0 has nonzero rows

    label6=(1/2,1/3,0,0), label7=(3/2,-1,0,0), label8=(1,0,0,0).

All other rows vanish. This is the accepted coordinate scale, not a normalized
replacement. At tau=0 the new row is the actual moving row `a0(z)`.

For each locked participant use its actual old-row five-column pivot. The
spectator weights, including the new delta, are fixed at their locked base
values. Solve mass and closure exactly through that pivot. The retained data
include pivot positions/labels, base determinant, beta(z), beta_z, beta_zz,
Q(z), U(z), and the stationary full merged gradient. All base beta entries
are strictly positive and all base pivot determinants nonzero, so this finite
collection has a common existential positive germ. No numerical radius or
uniform equality-family germ is asserted.

The physical volume is independently reconstructed along this W path by the
retained vertex incidence systems and signed determinant triangulation:

    V(z)=1/2+13z²/1728.

With mass one, the feasible action gives `c_upper=1/(2Q)`. Thus the ratio
score is `U=c_upper²/(2V)=1/(8VQ²)`. Every reported U_zz is the **second
derivative**, not the coefficient of z².

## The important source-germ difference

The inherited old germs prescribe spectator weights as
`beta0[j]+z first_weight_jet[j]`, as in PROOF.md (7.2) and
`code/independent_exact.py`. Their jets are read without rerunning a producer
from the consolidation archive's nested member
`incoming/non_hko_f10_certified_result.zip`, inner member
`evidence/exact/certificate_base_exact.json`. Both archive and member hashes
are retained. Direct reconstruction gives Q(z)=1/2 and U_zz=-13/432 for all
24 inherited germs on exactly the same physical W path.

Fixed spectators give the same score for 23 old sections, but **not section
7**, which participates in the locked certificate. Its fixed pivot has labels
`(0,6,4,8,9)`, comprising two q and three p rows. The fixed-spectator objective
is

    Q7(z)=(168z+288)/(49z²+336z+576),
    Q7,zz(0)=-49/576, U7,zz(0)=67/216.

Its inherited source germ instead has spectator slope `-7/96` at word
position 2 and U_zz=-13/432. These are different weight germs, although
both are valid positive capacity upper sections on the same physical path
and have the same base gradient. The locked K uses the **actual fixed** germ;
the quarter old cancellation mean uses the **inherited affine-spectator**
germs, according to the clarified gate contract. No curvature is silently
borrowed between them.

All six locked duplicated contacts, including the two nonadjacent positive-loss
placements, have Q(z)=1/2 and U_zz=-13/432 with their actual fixed spectators.

## Exact locked coefficient and specified mixture

Let lambda7 denote the locked certificate weight of old section 7. Since
every other locked participant has curvature -13/432,

    K=-13/432+(49/144)lambda7
     =-43945466140737206307506341551065638413825810571355371 /
       2208918494790312948760161822561145693678566322097187840,

approximately `-0.019894562087456668`. In particular K<13/1296.

The fixed three-quarter/one-quarter mixture keeps full forty-component
gradient cancellation: both constituent base means are independently checked
to have zero full gradient, normalized positive weights and feasible positive
sections. The old quarter has zero facet-birth response. If R is the locked
exact response and L0=625/497664, then

    mixed response margin=(3/4)R-L0 ≈ 0.00006322498777967461 >0,
    mixed second derivative=(3/4)K-13/1728
                           ≈ -0.022444069713740647 <0.

Both quantities are retained as exact rational numbers in `curvature.json`
and checked by replay. These certify the selected response/curvature premise.
They do not supply the remaining equality-family kernel, uniform positive
continuation or coupled ambient Taylor estimates. Production stops at this gate.
