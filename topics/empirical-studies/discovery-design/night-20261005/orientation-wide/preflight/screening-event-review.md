# Independent review: dominance and ONE-circuit kinetic events

Reviewed 2026-10-05 00:13 UTC for `direct_cover_lead`. Reviewed
`active_events.py` and `active-events.json`; neither was edited or rerun.
Retained independent read-only audit: `review_screen.py`.

## Verdict and independent evidence

The supplied dominance witnesses prove exact pruning from 121 distinct
bilinear candidates per circuit to survivor counts \((5,6,8,7,6)\). The
ONE-circuit-zero event extraction is complete on
\(0\le u\le U=1/\sqrt5\): it has only the two orientation endpoints and
one open strip. Its midpoint upper hull therefore describes that entire
strip, with the recorded linear-fractional x boundaries. Endpoint and raw
tie data are retained. No missed quadratic-extension branch was found for
this circuit. This is not a five-circuit joint arrangement or a body
positivity certificate.

The independent checker uses the retained reviewer's rational-pair arithmetic
definitions, without importing or running the producer. It verified all 605
distinct coefficient records; all 17,514 supplied dominance witnesses,
endpoint differences and tie flags; acyclic chains ending at survivors; and
independently checked that each claimed survivor is undominated. For circuit
zero it rederived the twenty pair slope/intercept polynomials and ten triple
concurrency polynomials, checked every recorded root set, and reconstructed
the whole upper hull by intersecting each raw line's dominance half-lines
against every other raw line. The raw midpoint and two endpoint hulls, all
listed finite ties, and maximum-slope ties matched exactly.
The completed replay took 5.72 seconds and passed; it created no files.

Replay, without writes:

```sh
python -B topics/empirical-studies/discovery-design/night-20261005/orientation-wide/preflight/review_screen.py
```

The checker also treats an unresolved in-strip algebraic root as an error,
rather than silently accepting a sampled hull.

## Why screening preserves the complete denominator

The coefficient order is \((c,a,d,b)\), representing
\(c+ax+du+bxu\). For a proposed dominator-minus-candidate difference, both
the x slope \(a+bu\) and intercept \(c+du\) are affine in u. Their
nonnegativity at zero and U proves nonnegativity throughout the closed strip,
and hence domination for all \(x\ge0\). Distinct coefficient vectors cannot
mutually dominate: domination in both directions would force slope and
intercept differences to vanish at both endpoints and thus identically.
The dominance graph is therefore acyclic, and every eliminated candidate
has a finite path to a retained maximum.

For \(0<u<U\) and \(x>0\), domination of distinct candidates is strict:
a nonnegative affine slope/intercept can vanish inside this interval only
if it is identically zero, and both cannot be identically zero for distinct
coefficient vectors. Thus no removed candidate can create a hidden interior
positive-x event or isolated maximizer. Removed candidates can tie at the
orientation endpoints, or at the formal boundary \(x=0\); the raw catalogue
is used there to restore ties. These ties are not new survivor classes.

The producer's weighted support choices are correctly bilinear: for a
covector \((n_x,n_y)\) and a template point \((v_x,v_y)\), the pairing with
\(G_xC_un\) is
\(n_yv_y+x n_xv_x+u\tau^2n_xv_y-xu n_yv_x\). Nonnegative circuit weights
allow the maximum over each of the three independent support vertices to
be distributed over all choices. The ten templates and three active normal
indices yield 270 raw choices per circuit before deduplication. The source
coordinate/template catalogue remains the same imported complete catalogue
as in the accepted slices.

## Why all event roots were covered

For five survivors there are ten pairs, each giving one slope-difference
and one intercept-difference polynomial, plus ten triple-concurrency
determinants: thirty polynomials total. They were all independently rederived.
None is identically zero in this packet. Linear roots are computed exactly.
For each quadratic, the independent audit either factors at a known endpoint
root or proves strict sign on the strip from both endpoint values and its
interior vertex value, when that vertex exists. The latter proof excludes
every root, including even-multiplicity roots, without needing to adjoin an
outside-strip nonsquare discriminant. An endpoint root permits exact
factorization; the other root is checked against the strip as well. Every
polynomial passed one of these complete alternatives. The only in-strip
roots are zero and U.

The producer similarly stops with an explicit error if a nonexcluded
quadratic root needs an extension of \(\mathbb Q(\sqrt5)\); it does not
silently drop that branch. Such a branch will require separate algebraic
isolation if encountered in another circuit. Persistent zero determinants
would require explicit persistent-collinearity handling; their absence was
checked here rather than assumed globally.

On the open strip, every pair's slope order is fixed and every triple's
orientation in slope/intercept space is fixed. Hence the upper convex hull
of the five lines is combinatorially fixed. Intercept-difference events cover
crossings through \(x=0\); slope-difference events cover parallel lines and
changes at infinity. Triple events cover the finite adjacent-boundary order,
concurrency, shared ties, and isolated line appearances. There are none
inside this strip. Its exact midpoint therefore suffices to select the
whole strip's active order, and the parametric intersections give the moving
x boundaries. Raw endpoint hulls retain all isolated and shared ties.

The midpoint active order for \(x>0\) is \(60,107,36,14,12\). Endpoint
catalogues correctly differ and include additional raw ties. The final
unbounded x tail is included, not truncated at a numerical cutoff.

The approved preflight stops at this one complete circuit. No assertion of
whole-domain five-circuit event coverage, joint cell counts, or a global HKO
inequality follows from it alone.
