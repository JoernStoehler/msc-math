# Fixed-policy equality-family continuation and score definitions

Producer first gate started 2026-10-05 01:58:04 UTC. Targeted QQ(v,s) checks
passed; independent acceptance and the assembled implication belong to
[review.md](review.md) and [report.md](report.md). No complete contact catalogue,
rank-4 completion or full F10 producer was executed.

```bash
uv run --script docs/empirical-viterbo-design/night-20261005/extremality-discovery/pstar-f11/general-facet-response/new-direction-ambient/replay.py
```

The replay is read-only. It reconstructs the source-defined old equality-family
sections, only the six locked duplicated policies, moving-kernel identities,
and specified base rank/balance minors. Source hashes and historical member
hashes are retained in `family-policies.json`. Initial targeted computation
took 20.60 seconds on this workstation.

## Exact policies and source aliases

The old24 family aliases use the actual source stationary continuation from
PROOF.md (6.1): the retained maximal stationary minor, free coordinates and
free values from nested `evidence/exact/certificate_family_exact.json`.
They solve every stationary, mass and closure equation over QQ(v,s), recover
the actual static source beta and mu at `(1/2,1/12)`, and have Q=1/2.
The locked old15 aliases are source indices
`0,3,4,5,6,7,8,11,12,14,16,17,18,20,21`.
These are stationary family aliases, not a closure-only continuation of frozen
base weights. Their full gradients recover the source rows at the base.

The six duplicated policies keep exactly their accepted r,delta and slot:

| Contact | Slot | Fixed r | Fixed delta |
| --- | --- | --- | --- |
| contact-03 | 0 | 1/4000 | 1/4000000 |
| contact-15 | 0 | constant | 1/9000 |
| contact-02 | 0 | 33/2564 | 33/2564000 |
| contact-05 | 2 | 526/5657 | 269/6491 |
| contact-16 | 0 | 62/5891 | 31/2945500 |
| contact-05 | 2 | 345/3748 | 389/9472 |

Substitute these parameters into the retained family beta functions, subtract
delta from old0, and insert the new copy at the actual slot. At tau=0 the new
normal is the same actual moving row a0. The replay derives their stationary
mu directly, verifies all equations and Q=1/2, and recovers every locked base
beta and full gradient. All base weights are strictly positive. Every closure
pivot is an old-row five-column minor, retained with actual word positions,
facet labels, rational function determinant and nonzero base value.

Every old24 and new6 full gradient annihilates the moving fifteen symmetry
columns, both equality-family derivatives and the actual inherited W(v,s),
identically over QQ(v,s). The eighteen kernel columns have rank18 at the base.
These exact identities, together with the base rank22 minor, imply constant
old rank22 and membership of each new gradient in the old row space on a
common smaller parameter patch. No base-only span test is substituted here.

## Rank, mass and the required score-germ split

Use the inherited constant coordinate-selection Y with indices
`[0,1,2,3,4,5,6,7,8,9,10,11,12,14,15,16,18,24,25,26,28,29]`.
Let H=G_old Y. The retained 22-row minor of H is nonzero at the base.
Append normalization to H^T; the retained 23 old coefficient columns give an
exact nonzero 23-by-23 base determinant. These matrices and column/row maps
are explicit in the packet. Invert that same minor on a smaller patch and
hold the remaining old coefficient at its retained base value. This defines
a regular smooth coefficient policy without an assumed generic IFT or an
unbounded symbolic inverse expansion.

Crucially, the actual mean has **45 score germs**, with 21 locked coefficients
`alpha_i=(3/4)lambda_i` and 24 additional old SOURCE-germ coefficients beta_j.
The fifteen locked old scores keep their FIXED-spectator germs; the additional
old24 scores keep their inherited affine-spectator germs. They are not merged
into one score merely because their touching gradients coincide.

For the gradient solve only, set `b_j=alpha_old_j+beta_j`. Fix the six new
alpha coefficients. Solve the 22 quotient gradient equations plus mass for
23 aggregate old b_j, keeping one b fixed. The old common kernel and rank22
then imply full forty-component cancellation. Return actual source-score
weights by

    beta_j(v,s)=b_j(v,s)-alpha_old_j,
    beta_j(base)=(1/4)mu_old_j>0.

All positivity checks apply to beta after subtraction, not just to b.
Regularity and strict base positivity give a common smaller patch where all
24 beta remain positive. Normalization gives `sum beta=1/4` identically;
the locked21 alpha sum to3/4. The inherited old positive normalized null mean
is independently bound to its source; its family continuation may also be
defined by the projection formula PROOF.md (6.5). It supplies positivity
and the rank/mass argument, not a silent replacement of this selected beta
policy.

## Actual scores and preservation of the accepted base jets

For each score, keep its selected old closure pivot. At family contact the
spectator values are the corresponding family beta functions. For locked21
scores keep spectators fixed while varying z,y,tau, including the new delta.
For the additional old24 source scores prescribe spectators as
`beta_family_j+z d_j`, using the inherited exact base first-weight jet.
Solve the pivot weights against the actual old normals and mass one; a born
new row is always `a0(A)+tau D`, in the transported normalized chart.

The pivot inverses and all base weights are regular and positive, hence these
45 explicit score germs have a common feasible positive neighborhood after
shrinking the compact family patch. Their base spectator data and pivot maps
are exactly those used by the accepted weighted-W gate. Therefore the actual
base beta_z and beta_zz are preserved; the six new policies retain those
jets explicitly. Old7's fixed-spectator curvature remains 67/216 in locked21,
while its additional inherited source score remains -13/432. The source-germ
split above preserves the previously accepted response and negative mixed
curvature. Continuity, not an asserted constant whole-family curvature,
extends their strict signs to a smaller patch.

## Exact one-vertex cap coefficient and uniform tau remainder

Let N(A) contain incident rows `(0,1,6,7)` of the unique positive old vertex
v=N(A)^(-1)1. Define gamma=D N(A)^(-1), and h=D.v. At the base D0,

    gamma=(-2/3,7/12,3/8,1/8), detN=-256/7,
    v=(5/12,1/2,0,1/2), h=5/12.

The three transverse gamma coefficients and h are strictly positive.
The other facet-0 vertices have strict negative D values (minimum absolute
margin1/12), while all old vertices outside facet0 have strict a0 slack.
These finite predicates persist on a common old-normal/family/direction patch.
For sufficiently small positive tau the new inequality excludes only v and
intersects all four incident old edges before their neighboring vertices.
The removed set is therefore exactly the simple-vertex simplex. Its slack
coordinates y=N(v-x) have Jacobian1/abs(detN), and the new normal is

    a0(A)+tau D=(1+tau gamma0)a0+tau sum_{j=1}^3 gamma_j a_j.

The cut depth at v is tau h, so elementary simplex volume gives exactly

    DeltaV=tau L(A,D)/(1+tau gamma0),
    L(A,D)=h^4/[24 abs(detN) gamma1 gamma2 gamma3].

All denominators are bounded away from zero on a smaller compact patch,
including `1+tau gamma0`. Thus L is smooth, L(base,D0)=625/497664, and
`DeltaV=tau L+O(tau²)` uniformly. This argument uses the actual transported
row and all new intercepts inside the four old edges; it does not assume a
global clipping sign partition or joint smoothness across other chambers.

These are production premises for the lead's selected coupled-scale assembly.
The final ambient implication, its independent review and its honest scope
remain with the integrating owner.
