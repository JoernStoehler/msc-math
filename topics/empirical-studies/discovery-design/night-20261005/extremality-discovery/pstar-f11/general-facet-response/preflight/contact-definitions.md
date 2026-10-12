# Supplied-contact census definitions

Production preflight started 2026-10-05 01:06 UTC; exact first reduction took
18.29 seconds. This packet inventories supplied contacts, without asserting
complete touching-facet coverage or a balanced response cover. It neither
reruns inherited producers nor introduces new words.

The 24 vertex-family witnesses supply 12 primitive cyclic positive words.
The ridge packet has **24 adjacency rows**, derived from those 12 words;
it is not a separate list of 24 contact families. Six primitive words have
positive facet-0 weight. The eight vertex cones incident to facet 0 are
the two top q vertices times all four p vertices. For each, select the
incident facet-0 slot `t_j=1`, set the other slots to zero, and write the
retained section as `beta0+r d_j`. Its retained pure-slot curvature is zero,
so Q=1/2 exactly. Here r is the former new-label weight, not a new HK search
parameter.

Delete weights identically zero in QQ(v,s,r), rename new label 10 to old
label 0, and merge only cyclically adjacent equal labels after that deletion.
Rotate to start with the smallest facet label. Reversal is not identified.
Deduplication requires the same cyclic word and every exact rational weight;
distinct domains and original vertex sources remain in `aliases`. The eight
facet limits yield four nonconstant contact families and two constant words;
the four constant source limits merge in pairs. Those two constant words
are already among the twelve primitives. The supplied stationary continuum
adds one further family. Thus 21 source candidates reduce to 17 contacts,
of which 11 have positive facet-0 weight on their stated domains.

The exact positive domain of each contact is recorded by its explicit
rational beta inequalities `beta_i(v,s,r)>0`, together with a common small
equality-family parameter patch and nonvanishing retained denominators and
closure minor. For nonconstant families, `0<r<=1/1200` is a sufficient
control interval at the base, not a claimed maximal domain. Identically zero
r=0 weights have strictly positive r slopes at the base; the other weights
remain strictly positive throughout that interval. This provides a common
smaller family patch by continuity. All reduced r=0 limits are retained with
matching word/weight lists. They are limits, not positive representatives of
the original nonconstant word.

For each contact the checker derives mass, closure and Q=1/2 exactly, and
checks `H beta=1+B mu`. Nine contacts have closure rank 5, including seven
with positive facet-0 weight. They have an explicit regular five-column
minor, unique mu, and a feasible positive weight continuation under arbitrary
small old-normal changes for each fixed positive parameter point. Their
full forty-component merged gradient is derived as

    G=-4 sum_i beta_i[Omega sum_j sign(j-i) beta_j a_j - mu] - 2 DV,

with the summand placed in the corresponding facet row. All fifteen symmetry,
two equality-family and one W contractions vanish identically over
QQ(v,s,r). The inherited old constant rank 22 and full kernel of dimension
18 therefore put all nine gradients in the old row span. None supplies a
new transverse W mechanism. Uniform feasibility as r tends to a degenerate
limit is a separate estimate; only the already accepted stationary endpoint
has that supplied uniform argument.

Eight contacts have closure rank 4, including four with positive facet-0
weight. The checker records a stationary multiplier representative by setting
its free coordinates to zero. Its gradient is a **representative algebraic
row**, not a gradient of a supplied ambient feasible section. Nonzero W
contractions of these gauge-dependent rows cannot be called a new mechanism.
The first concrete bank-completion gap is a regular positive full old-normal
continuation for these singular primitive contacts; retaining zero labels
would not create positive baseline weights.

For the regular seven facet-0 contacts, the substantive missing input remains
the direction-dependent normalized balance with response exceeding the actual
facet-cap integral. The eight cap vertex-sign expressions are retained, but
no sign-cell partition or cap functional integration was performed. For a0,
`omega(a0,D)=2d` in coordinates `(q1,q2,p1,p2)`, so a general duplication
response includes the phase term `2d delta(beta_f-delta)` as well as the
closure-response terms. No reduction to the p1-only endpoint is justified.

`contact_census.py` is a read-only reproduction of the reduction and exact
checks. It compares its derived result with the retained JSON, omitting only
the elapsed-time observation, and prints counts. Source hashes, source byte
sizes, exact formulas, aliases, minors and all kernel residuals are in
`contact-census.json`. The existing F10 chart/rank/old balance and endpoint
continuation are inherited inputs; the general direction balance, complete
contact coverage, maximal domains and uniform transition estimates are absent.
