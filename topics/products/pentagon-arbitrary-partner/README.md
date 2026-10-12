# Fixed regular factors with arbitrary convex partners

This packet combines an analytic symmetric-partner theorem with a heterogeneous
partner study. Its evaluator represents capacity by finitely many translation
fits; its geometry questions concern which changes those fits retain or lose.
The analytic conclusions do not come from sample statistics.

## Exact statements and the selected-thesis boundary

For the circumradius-one regular pentagon `Q` with normals
`n_i=(cos(2pi i/5),sin(2pi i/5))`, put `h=cos(pi/5)`, `D=Q-Q`,
`B=D^polar/2` and `W_Q(K)=max{r:r D^polar ⊆ K-K}`.
The [symmetric-partner proof](symmetric-partner-lemma.md) shows

```text
c_EHZ(K ×_L Q)=W_Q(K),
sys(K ×_L Q) <= (10+6sqrt(5))/25
```

for every centrally symmetric planar convex body `K` with nonempty interior,
including smooth bodies. Equality in the ratio holds exactly for the specified
regular decagon `B` up to translation and positive dilation. Explicit normal
triangle placements prove `c(B ×_L Q)=1`; the segment upper bound and monotonicity
then transfer this calibration to every symmetric partner.

This is an independently agent-checked argument, not human proof acceptance or
a novelty claim. Jörn selected this pentagon instance for inclusion on 3 October
2026 ([project facts](../../../docs/project-facts.md), item 13.2); it remains
absent from the selected manuscript. That is a pre-existing thesis promotion
gap, not a migration regression. No theorem has been promoted here.

The [odd-regular theorem](odd-regular-symmetric-partner.md) already extends the
calibration to every odd `n>=5`, with sharp decreasing constants

```text
kappa_n = 4(1+cos(pi/n))²/[n² sin(pi/n)sin(2pi/n)] < 1,
kappa_n -> 8/pi².
```

It includes affine-image transfer and the exact equality partner. It was
independently agent checked on 3 October; the generalization has no established
human acceptance or thesis selection. The separate
[heptagon proof](heptagon-symmetric-partner.md) is subsumed mathematically, and
retained as the shorter original calibration route. The triangle obstruction
in [preliminary calibration](calibration-extension-preliminary.md) excludes a
naive extension to `n=3`. The conjectural width formula for two nonsymmetric odd
regular factors in [the affine note](../pentagon-affine-products/research.tex)
is still open; it is not this symmetric-partner theorem.

[Local calibration stability](local-calibration-stability.md) gives an open
labelled-polygon neighborhood for each fixed odd regular factor. It uses exact
boundary-contact identities and strict-interior margins, not a quantified
perturbation radius or a statement about all convex-body neighborhoods.

## Fitting representation and empirical evidence

The [evaluator](evaluator.py) implements the BMP normal-triangle translation
criterion for the fixed pentagon. Five positive normal triples and their two
orders give ten unit-perimeter template classes. For each template, an LP
maximizes its translation-fit scale in `K`; capacity is the minimum of the ten
scales. The [retained evaluator/source contract](research-charter.md) specifies
support-function length, product axes, scaling, primal/dual checks and imported
source hypotheses. It also preserves the original research charter; its
future-tense kickoff is superseded by the executed records below.

[The first window](first-window.md), [retained validation](validation-v1.json),
and [collection metadata](collection-v1-metadata.json) own the heterogeneous
body collection and controlled paths. They preserve generators, raw geometry,
lineage and failure accounting. Related transformations are not independent
samples. [Harmonic fibers](harmonic-fibers.md) demonstrate that equal fitting
vectors can coexist with different geometry; [source comparison](source-check.md)
keeps deductions separate from external novelty. [Rational controls](rational-calibration-control.md)
provide a bounded exact check, not certification of the entire LP collection.

The evaluator uses floating-point LPs. Tiny or ill-conditioned inputs require
rescaling and tolerance analysis; scientifically consequential inputs need exact
reconstruction or interval bounds. Fitting contacts are not orbit labels.
Neither small control discrepancies nor 10 LPs per input prove universal
numerical reliability. The analytic symmetric-partner theorem uses its hand
argument and imported criterion rather than those outputs.

## Remaining routes and failures

For arbitrary asymmetric `K`, central symmetrization preserves its difference
body and hence its width ceiling. Capacity is concave and symmetric along the
fixed-width path from `K` to `-K`; its increase toward the midpoint is forced,
not an empirical discovery. The precise plateau onset remains a geometry
question. Equal fitting vectors need not determine widths, area or moments.

The [local arbitrary-partner rigidity note](local-cover-rigidity.md) has an
[exact finite certificate](local-cover/README.md), but the continuum
signed-polygon estimate has not been independently formalized. The
[coupled one-cut lifting](local-cut-lifting.md) and
[generic-cell contact reduction](smooth-cell-reduction.md) are scoped analytic
research routes. None is a global fixed-partner theorem or an active thesis
proof. The minimum-area cover among arbitrary asymmetric partners remains
unresolved; independent-affine-pentagon optimization settles only a subclass.

Reproduction commands live with the individual records and write owned outputs.
No producer, collection or certificate was rerun for this relocation. Scripts
use local file paths; retained JSON, figures and verification logs retain their
original bytes and historical provenance.
