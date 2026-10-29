# Independent audit of weighted radial profiles and seventeen facets

29 October 2026. Reviewer `/root/hko_ascent_math_audit`.

**PASS.** The exact weighted response formula, seven-cut ascent, and exclusion
of positive first variation for at most six nonzero speeds in this fixed
profile cone are valid. The resulting small-parameter body has seventeen
genuine facets. No local-maximality statement for arbitrary sixteen-facet
perturbations follows or is asserted.

Frozen module inspected:

    /tmp/msc-math/restart/multicut/hko-seventeen-facets.tex
    SHA256 910db4726b53eae2720004c6357b838c4c479b1eb72436ae9bcdf745b4edc9ec

Witness checker inspected and replayed:

    /tmp/msc-math/restart/multicut/check_radial_profile_witness.py
    SHA256 0ccd34ac43b363b03c28e3bea32ec113b37302886e5c13048de3b519629d2757

The replay output is `radial-witness-replay.json` in this audit directory.
It reuses the previously audited exact_core number-field arithmetic. I read
the literal word, weights, hinge arguments, reconstruction and all asserted
predicates, and checked the analytic implication separately.

## Witness and symmetry

For the displayed word (0,6,5,2,1,8,4), the exact weights sum to one, are
positive, close the word, and give Q=tau/5. Solving the rank-five KKT system
and checking every active equation gives the reconstructed loop. The checker
verifies every original facet inequality at both endpoints of every segment,
so convexity supplies containment along the segments. The radial coordinate
is constant along each segment, and its literal hinge argument is the one
printed in the module.

Since 0<tau^2<1, all hinge arguments except the fifth are negative. That
segment is labeled facet 1, has weight 1/25 and hinge value 1/5. Thus the
response is exactly 1/125 on facet 1, and zero on every other facet.

Simultaneous rotation R by 72 degrees is orthogonal, commutes with the ambient
J(q,p)=(-p,q), and therefore is symplectic. It preserves both regular
pentagons, cyclically permutes their normal labels, and commutes with J2 in
the profile formula. It consequently transports the labeled profiles.

The map S(q,p)=(J2 p,-J2 q) is an orthogonal involution commuting with the
ambient J, hence also symplectic. For H=P x (-J2 P), it maps H to itself.
It sends (n,0) to (0,-J2 n), exactly the corresponding p-normal, and transports
the associated D row as well. The exact checker verifies this on all normal
and velocity rows and checks symplectic pairings on the standard basis.
Thus C5 together with S is transitive on the ten labeled facets and produces
a concentrated witness on each one.

## Exact weighted formula

For w_i>=0, put W=max_i w_i. The already audited complete all-word response
bound and pointwise nonnegativity give

    sum_i w_i mu(f_i) <= W sum_i mu(f_i) <= W/125.

A concentrated witness on a facet where w_i=W attains W/125. This also
covers W=0. Continuity passes the same value to the weak occupation closure.
Therefore the equality is exact for every nonnegative speed vector, not
merely for unit speeds or a selected subset.

The cone-volume mass of a single profile is

    kappa = nu(f_all)/10 = 8(5+sqrt(5))/46875.

For a fixed finite w, the split-bank derivative theorem applies to its rows
a_i+t w_i D_i and gives exactly

    D+ log sys = 4[kappa sum_i w_i - (max_i w_i)/125].

The proof requires no uniform neighborhood as w varies.

## Seven cuts, six-cut obstruction, and genuine facet count

The exact inequalities 6kappa<1/125<7kappa hold, and the witness replay checks
them in the ordered number field. Seven unit speeds give derivative

    4(7kappa-1/125) = 4(56sqrt(5)-95)/46875 >0.

For a nonzero vector with at most six positive entries, sum_i w_i<=6W, so its
derivative is strictly negative. All zero speeds give zero derivative. Thus
seven is the smallest possible number of positive speeds giving ascent
within these ten fixed profiles.

For the facet count, on the relative interior of every old facet one can
choose points with ell<3/5 and points with ell>3/5. The first open patch
keeps the old facet while satisfying its own split row strictly; all other
rows remain strict for small t because all other base inequalities were
strict there. At a point x on the second patch of a selected facet, the
radially rescaled point x/(1+t w_i D_i.x) lies on its new hyperplane and
strictly inside its old inequality, with all competing inequalities still
strict for small t. Both patches are three-dimensional relative open patches.
Hence every old row and every selected new row is a genuine facet. Distinct
rows remain distinct for sufficiently small t, so seven selections yield
exactly seventeen facets.

## Scope and editorial recommendation

This is a useful printed refinement: the exact weighted response law explains
why a particular number of these cuts is needed, rather than merely shaving
three constraints from an example. The literal concentrated witness and short
symmetry argument are small enough to print; keep replay details in the
companion.

The explicit t=10^-30, relative-gain >t/200 effective result audited separately
currently concerns all ten unit cut profiles. Its numerical gain constant
must not be transferred verbatim to this smaller-slope seventeen-facet family.
The present module correctly claims sufficiently small positive t without a
specific effective parameter or gain bound.
