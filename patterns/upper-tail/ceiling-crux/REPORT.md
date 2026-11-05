# Proposal: a sharp volume bound for a small feasible geometric optimizer

17 October 2026. The proposed inequality

$$\frac{\min\{c_2(P,Q),C_\star(P,Q)\}^2}{2|P||Q|}
\le \rho_H:=\frac{3+\sqrt5}{5}$$

survives a bounded test on **14 nonregular random polygon pairs whose exact
two-bounce width ceiling exceeds HKO**. On 60 additional constructed HKO
perturbations, width alone already settles this inequality; separately, the
standalone star ceiling also passes all 60. This packet is a proposal and a
bounded crux check, with no new global theorem, universal conjecture
endorsement or working broad predictor.

Here $c_2$ is the difference-body two-bounce capacity and $C_\star$ is the
existing [full-tangent chord-star feasible capacity ceiling](../chord-star/REPORT.md).
The mathematical construction is unchanged: centroid normalization, cone-volume
closure center, both full tangent-projector chord endpoints for every facet,
both signed angular words, both factor roles and exact opposite-factor linear
responses. Eta and the full paired capacity oracle are not computed.

## Frozen tests and their interpretation

The first cohort includes all 14 retained 5x5/5x6/6x6 geometries flagged by the
existing geometry-only width feature as above $\rho_H$. Names and formula were
frozen in PLAN.md before star evaluation; all exact width recomputations confirm
eligibility. Root then selected the complete 60-row existing contact-stability
geometry packet; PLAN-amendment.md freezes its membership before evaluation.
No historical capacity targets were joined. These existing geometric controls
are a falsification test, not fresh held-out predictive evidence.

| Frozen cohort | Rows | Exact width-ratio range, displayed numerically | Standalone star-ratio range |
|---|---:|---:|---:|
| Hard-width random products | 14 | 1.059063–1.329568 | 0.475639–0.790584 |
| Constructed HKO perturbations | 60 | 0.904568–1.046614 | 0.904184–1.046614 |

The largest random star ratio is 0.790583898337 at `random_5x6_s0_145`.
The largest constructed star ratio is 1.046614083360 at
`contact_support_s0_e3of1000`, below $\rho_H\approx1.047213595500$.
The exact quadratic-field HKO control recovers $C_\star=5$ and
$S_\star=\rho_H$. This supplies neither a new capacity value nor an
improvement of a known true-ratio bound.

## Remaining proof question

Let $V=|P||Q|$ and $q_\star=1/(2C_\star)$. The global geometric implication
still requiring proof or a counterexample is

$$c_2^2>2\rho_HV\quad\Longrightarrow\quad
q_\star\ge\frac1{\sqrt{8\rho_HV}}.$$

Thus any pair whose widths permit a ratio above HKO would need a sampled star
point (a chord endpoint or the center) and one signed angular word whose exact
opposite-factor response reaches this volume-normalized payoff threshold.
The existing coverage theorem's relative factor eta does not establish this
absolute threshold. No proof mechanism for the implication was established
here. Proving it would imply global HKO maximality for planar polygon products;
refuting the surrogate would not refute true HKO maximality.

The stronger standalone inequality $C_\star^2/(2V)\le\rho_H$ also passes these
74 controls, but is not promoted to a universal conjecture. No further cohort,
ray tuning, broad coverage scan or calibration was selected.

## Verification and reproduction

The originating runs check 3,304 direct four-dimensional HK sums at evaluated
star/response pairs. The separately authored `verify.py` independently checks
one winning witness per body: exact primal areas/centroids, centered normals,
nonnegative normalized closure weights, projector-star membership, signed
angular word eligibility, the literal four-dimensional HK sum and exact
comparison of its capacity-ratio ceiling with HKO. Checking a feasible partner
at a valid star point suffices: the star response maximum is at least that
witness payoff, so its capacity ceiling is at most the checked witness ceiling.
Full response-maximization replay is unnecessary for this finite implication.

`compact-witnesses.json.gz` retains all 74 original geometries and the exact
winning witnesses; `verification.json` records the independent replay. The
large complete response traces remain scratch outputs. Reproduce the compact
check with `python3 -B verify.py` from this directory; its receipt must report
PASS. Inputs, helpers, frozen plans and outputs have hashes in `sources.json`.
The original construction scripts can be copied with the plans to a fresh
scratch directory and run as `python3 -B probe.py` then `python3 -B extend.py`.
They use the explicit local registered cache, not a producer or capacity oracle.

Offline cache verification passed. Random-product input SHA256:
`66bf82010e92e0f26b0df226f4e6c0eef05d21eb22a0967c7f669530f6545736`;
constructed input SHA256:
`f70e7f9bb6444e2b132dbaedf73c8f4a1f8324e72c37f79dbb7c2908e27bc2c8`.
Main runs used 12.18 and 20.99 seconds wall, 12.15 and 20.04 seconds CPU.
All outputs began in `/tmp/msc-math/upper-tail-next/20261017/ceiling-crux/`.
No network, cloud, installation, fitting, producer rerun, full capacity oracle,
thesis edit or public action occurred. Root owns integration and commits.

[review: /root/closure_review]
