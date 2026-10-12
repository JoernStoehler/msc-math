# Positive base response for D=(1,-1,0,1)

Production started 2026-10-05 01:21 UTC. The exact finite positive balance was
independently accepted at 01:33 UTC; source-backed packaging and replay were
completed at 01:35 UTC. Independent acceptance is owned by [review.md](review.md),
and scientific integration by [report.md](report.md).

The result is a **pointwise base balanced response** for the specified split
direction. It is not a whole-facet response cover, a uniform moving-old theorem,
or a full F11 neighborhood theorem. No rank-4 contact completion, new word
search, producer rerun or direction grid was used.

```bash
uv run --script docs/empirical-viterbo-design/night-20261005/extremality-discovery/pstar-f11/general-facet-response/discriminator/replay.py
```

The replay is read-only and needs only SymPy. It derives the seven source
schemas, all cyclic placements, stationary multipliers, closure jets, full
gradients and capacity responses from the retained mathematical inputs. It
then verifies the finite positive balance and the physical cap integral.
No optimizer or stored numerical dual is part of this verification.

## Bank, domains and all cyclic placements

Use the base `v=1/2,s=1/12`, facet 0, and coordinate order `(q1,q2,p1,p2)`.
The seven regular facet-0 schemas are exactly those accepted in the preflight.
Their complete positive base domains follow from their explicit affine beta
inequalities, rather than the earlier small-r control interval.

| Source contact | r domain | Duplicate-weight domain | Touching slots |
| --- | --- | --- | --- |
| contact-00 | 0<r<1/4 | 0<delta<r | 0,1,2 |
| contact-02 | 0<r<1/18 | 0<delta<r | 0,1,7 |
| contact-03 | 0<r<1/4 | 0<delta<r | 0,1,6 |
| contact-05 | 0<r<1/4 | 0<delta<r | 0,1,2 |
| contact-11 | constant | 0<delta<1/9 | 0,1 |
| contact-15 | constant | 0<delta<1/9 | 0,1 |
| contact-16 | 0<r<1/9 | 0<delta<r | 0,1 |

Every reduced source word begins with facet 0. Slot j inserts the new label
10 immediately before word position j. Thus slots 0 and 1 are the adjacent
before/after placements; the other touching slots are genuinely nonadjacent
placements within a q block. This finite inventory has 48 slots, of which 18
are touching and stationary throughout the full positive parameter domain.
For each of the other 30 slots, `1/2-Q=delta T(r)` with an affine T strictly
positive in the open r domain, or a positive constant for a fixed contact.
The replay checks these exact endpoint signs, so no isolated positive-domain
touching case is discarded. Boundary vanishing gaps are recorded algebraic
limits; positivity and regular continuation are not asserted at such limits.

All touching merged gradients and responses have total degree at most two
in `(r,delta)`. This degree is derived from the actual formulas. The full
forty-component gradient changes with delta for nonadjacent placement; it is
not replaced by the original seven-schema gradient. All 18 touching gradient
functions annihilate the fifteen symmetry columns, both equality-family
tangents and W exactly at the base. Hence they remain in the inherited old
rank-22 row space; no new W mechanism is asserted.

The adjacent source convention is

    ell=2delta[k_f.D +/- (beta_f-delta)(Omega a_f).D],

with plus before and minus after. Here `(Omega a0).D=-2`, whereas
`omega(a0,D)=+2`. For every slot the replay also derives ell by direct
symplectic pair differentiation through the fixed regular closure pivot,
including the intra-block phase and the delta-dependent stationary multiplier.

The old zero-loss rows are the **specific static 24 sections** in
`experiments/hko-local-maximum/non-hko/certificate/certificate_base.json`.
Their actual beta and pivot choices, gradients and positive null relation
are reconstructed and checked. They are not the 24 vertex families or a
larger collection of unspecified old policies. That source, the family code,
the preflight census and the endpoint W source have hashes in the certificate.

## Exact physical cap

On facet 0 the chosen direction is `g=q1+p2-1/2`, with
`-5/12<=q1<=5/12`. Only the facet vertex with `q1=5/12,p2=1/2`
has positive g. At height `p2=1/2-t`, the momentum polygon has horizontal
width `2t`. The positive cap has `0<q1<5/12` and `0<t<q1`.
Therefore its nonshrinking facet fraction and removed-volume derivative are

    fraction=(5/12)^2/3=25/432,
    L0=(1/2) integral_0^(5/12) integral_0^q (q-t)2t dt dq
      =(5/12)^4/24=625/497664.

The factor 1/2 is `1/||a0||`. The replay integrates these rational formulas
exactly. The lead and independent reviewer own the eight vertex predicates
and the comparison with the earlier direction controls.

## The finite positive certificate

There are 21 participants: 15 old zero-loss sections and six strictly positive
split contacts. The old source indices are
`0,3,4,5,6,7,8,11,12,14,16,17,18,20,21`.
The six split points are:

| Contact | Slot | r | delta | Loss sign | Approximate balance weight |
| --- | --- | --- | --- | --- | --- |
| contact-03 | 0 | 1/4000 | 1/4000000 | negative | 0.0396488 |
| contact-15 | 0 | constant | 1/9000 | negative | 0.0685311 |
| contact-02 | 0 | 33/2564 | 33/2564000 | negative | 0.0238780 |
| contact-05 | 2 | 526/5657 | 269/6491 | positive | 0.113577 |
| contact-16 | 0 | 62/5891 | 31/2945500 | negative | 0.0610636 |
| contact-05 | 2 | 345/3748 | 389/9472 | positive | 0.0210317 |

Every weight is an explicit strictly positive rational number in
`certificate.json`. Negative-loss contacts are intentionally included to
cancel the full old-normal gradient; the positive response is the weighted
sum, not a claim that each participant improves the capacity separately.
The replay verifies every old and new beta is strictly positive, each contact
parameter and delta lies strictly inside its stated domain, all masses and
normal closures hold, and Q=1/2. The fixed old closure minor then supplies a
genuine positive feasible continuation for each of these finitely many split
points for sufficiently small tau.

The exact certificate identities are

    sum lambda=1,
    sum lambda G=0 in all40 old-normal components,
    sum lambda ell > 625/497664.

The exact weighted response is

    4537636582135202405787010729639260744067557602097870481686104124935418941 /
    2579976532157362915851013972325193451683589749626687753975231282995200000000,

approximately `0.0017587898671081533`. Its exact excess over the cap is

    105099778550355069607210803231111190044497622554322065293508100807268934221 /
    208978099104746396183932131758340669586370769719761708071993733922611200000000,

approximately `0.000502922454556713`, strictly positive. This establishes the
selected pointwise balanced response. It does not certify response at every
direction or every contact parameter, nor optimality of this finite mixture.

## Discovery versus certification and stopping boundary

The degree-two Bernstein sufficient dual proposal had bound `0.0038015`,
above the cap; it was inconclusive. An initial exact finite primal candidate
also lay below the cap and was inconclusive. A numerical exchange procedure
then minimized quadratic residuals analytically over triangle edges and
interior stationary points, without an r/delta grid. Its finite candidate
support was rationalized and moved strictly inside the positive domains,
then the full balance was solved exactly. Numerical dual convergence and
numerical residual tolerances are not proof premises of the retained result.

Earlier proposal files are retained as inconclusive discovery artifacts.
The accepted candidate has SHA256
`88f05ef07c2d813858ec960d4c77a05f3e3bcc52f396588db6ca24fa32796a0a`.
The final certificate embeds that candidate unchanged and binds its source
hash. Formula generation took 3.29 seconds; the exact final finite solve took
0.51 seconds; the read-only source-backed replay took 5.08 seconds on this
workstation. No exhaustive response optimum or whole-direction partition was
attempted. Production stops at this accepted base certificate.
