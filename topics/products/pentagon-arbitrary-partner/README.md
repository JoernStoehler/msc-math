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
2026 ([project facts](../../../docs/project-facts.md), item 13.2). On 19 October it was
integrated in [selected Section 11](../../../thesis/chapters/10b-symmetric-partners.tex),
using the [geometric exposition](symmetric-partner-exposition.md). Independent
source/math review, the 105-page build and focused pp. 72–74 visual/interface
checks passed. Human proof/prose acceptance and whole-PDF PASS remain open.

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
naive extension to `n=3`. The separate
[uniform angular theorem](../pentagon-affine-products/odd-regular-angular-width.md)
now proves the width formula for two nonsymmetric odd regular factors with
at least five sides. It is distinct from this symmetric-partner theorem;
independent-affine tensor containment remains open.

[Local calibration stability](local-calibration-stability.md) gives an open
labelled-polygon neighborhood for each fixed odd regular factor. It uses exact
boundary-contact identities and strict-interior margins, not a quantified
perturbation radius or a statement about all convex-body neighborhoods.

## Restricted asymmetric-partner theorems (5 October)

The [research acceptance synthesis](../../empirical-studies/discovery-design/night-20261005/comparison/research-acceptance-synthesis.md)
records later independently agent-reviewed results, integrated here on 12 October.
Their acceptance is relative to the named mathematical inputs; it is neither
Jörn/Kai acceptance, external novelty closure nor selected-manuscript promotion.

| Family or reduction | Supported conclusion and boundary |
| --- | --- |
| [All-angle equiangular fan](equiangular-partner-global-20261005.md) | Arbitrary supports on a regular five-normal fan have capacity equal to the largest contained regular-pentagon scale times the regular capacity. The regular partner uniquely maximizes ratio in each fan; across angles the sharp bound is `(3+sqrt(5))/5`, with HKO-angle regular equality. Edge-loss bodies are included. |
| [Fixed-axis orientation-wide family](orientation-wide-global-20261005.md) | Normals `diag(x,1) R_theta n_i`, arbitrary supports, all `x>0` and angles have sharp HKO bound. Equality requires `x=1`, `theta=pi/10 mod pi/5` and a regular partner up to translation/dilation. This includes non-affinely-regular partners but fixes the distortion axes relative to Q. |
| [Aligned nonconformal family](aligned-nonconformal-global-20261005.md) | Normals `diag(x,1)n_i` have sharp ratio `3(1+sqrt(5))/10`, uniquely at `x=sqrt(5)/3` with the certified non-affinely-regular support class. The [HKO-angle slice](hko-angle-nonconformal-global-20261005.md) instead has the HKO benchmark, uniquely at `x=1` regular. |
| [General-matrix cone reduction](general-matrix-cone-reduction-20261005.md) | The inherited all-matrix fitting model reduces to a seven-variable homogeneous cone with 610 distinct rows and the stated quadratic target. The target's positivity remains open: this is a reduction, not an arbitrary-GL(2) bound. |

Complete cell/boundary/tail and equality checks support the restricted theorems;
a sampled angle grid would not. Arbitrary asymmetric partners, outer-axis
rotation/general GL(2), and arbitrary non-affine pentagon pairs remain outside
the proved global bounds. The [final disposition](../../empirical-studies/discovery-design/night-20261005/comparison/final-research-disposition.md)
parks further execution rather than declaring those questions solved.

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
not an empirical discovery. The [19 October sharp plateau theorem](symmetrization-plateau-20261019.md)
now gives the optimal universal interval [(3-sqrt(5))/2,(sqrt(5)-1)/2].
One explicit trapezoid has exactly that equality interval. The first onset for
a particular other partner remains a geometry question. Equal fitting vectors need not determine widths, area or moments.

The [local arbitrary-partner rigidity note](local-cover-rigidity.md) has an
[exact finite certificate](local-cover/README.md), but the continuum
signed-polygon estimate has not been independently formalized. The
[coupled one-cut lifting](local-cut-lifting.md) and
[generic-cell contact reduction](smooth-cell-reduction.md) are scoped analytic
research routes. None is a global fixed-partner theorem or an active thesis
proof. The [algorithm account](translation-cover-algorithms-20261019.md) identifies
a published exact unrestricted strategy and recovers the already solved
ten-direction fan baseline; numerical parallel-slide probes remain candidates,
not a global certificate. The minimum-area cover among arbitrary asymmetric partners remains
unresolved; the independent-affine and fixed-axis orientation-wide theorems
settle distinct restricted classes.

Reproduction commands live with the individual records and write owned outputs.
No producer, collection or certificate was rerun for this relocation. Scripts
use local file paths; retained JSON, figures and verification logs retain their
original bytes and historical provenance.
