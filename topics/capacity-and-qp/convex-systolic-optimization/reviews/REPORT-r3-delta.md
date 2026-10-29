# Assembled mathematical review — r3 delta and final verdict

Reviewer: `/root/assembled_math_review`, 29 October 2026, 00:25–00:32 UTC. Receiving owner: `/root`; integration owner: `/root/paper_integration`.

**PASS for the assembled 48-page revision r3, combining the full r2 review in `REPORT-r2.md` with the independent delta review below.** No mathematical blocker remains in the reviewed revision. This is an agent mathematical assessment, not external refereeing, a formal verification, or human acceptance.

- PDF SHA-256: `689426594375053635c2f39676ca90288bf69bcaba98d8388ea09deb97d0b600`.
- `main.tex` SHA-256: `fd26a08951475622fdc3c29a613f532c321b5f8d0f00e8d42bf27e3169f60b01`.
- `sections/hko-seventeen-facets.tex`: `910db4726b53eae2720004c6357b838c4c479b1eb72436ae9bcdf745b4edc9ec`.
- `sections/hko-effective-step.tex`: `a9f8963db5a9082c26056c94a7cfa55750e127e363aea3ff537b7aa7928de376`.
- Full source and quantitative-companion hashes: `reviewed-r3-hashes.json`.
- Frozen exact inputs and PDF/text: `reviewed-r3/`.

I inspected the full r2-to-r3 source diff and the newly assembled PDF text, not just the proposed component drafts. The old proofs have no new mathematical changes beyond the expressly reviewed ascent scope updates. The original introductory weight-index correction persists. The new abstract, roadmap, section introduction and synthesis distinguish the seventeen-facet first-order result from the explicit estimate for the original twenty-row family.

## Exact weighted response and seventeen genuine facets

The upper bound for nonnegative speeds follows from the existing *all-repeated-word* response bound and pointwise nonnegativity: weighted response is at most `max(w_i)` times the total response. This alone would not establish equality. The new literal seven-word witness supplies the necessary converse.

I independently reconstructed its field arithmetic and KKT system in `check-weighted-witness.py`, using only Python's standard library and no imports from the submitted implementation. The fresh result in `weighted-witness.out` verifies:

- closure and sum-one normalization of the displayed weights;
- exact objective `tau/5`;
- the full seven-equation stationarity system and multiplier normalization;
- all seven displayed hinge identities;
- response `1/125` on facet 1 and zero elsewhere;
- the factor-exchange map's involution, symplecticity, normal permutation and cut-velocity equivariance.

The claimed strict positivity and signs follow immediately from `0<tau^2<1`; the reconstruction theorem already proved in the appendix then gives a genuine shortest characteristic. Simultaneous rotation preserves both factors and their profiles. Together with `S(q,p)=(J_2p,-J_2q)`, it acts transitively on the ten labels. Consequently every label supports a concentrated shortest-word response, and choosing one with maximal speed proves the exact formula, including all weak occupation limits.

The facet cone-volume normalization is correct: the new `kappa` is one tenth of the old total response, not the old total itself. Hence

`D+ log sys(H_(t,w)) = 4[kappa sum_i w_i - max_i(w_i)/125]`.

The exact inequalities `6 kappa < 1/125 < 7 kappa` give the stated positive seven-unit-speed slope `4(56 sqrt(5)-95)/46875`. The existing positive/negative facet-patch argument works after deleting the unchosen rows, so any chosen seven unit cuts give seventeen genuine facets for sufficiently small positive parameters. At most six nonzero nonnegative speeds have strictly negative derivative unless all speeds vanish. The paper correctly limits this minimality statement to these fixed ten profiles; it does not claim seventeen is the smallest possible facet count for arbitrary improving deformations.

## Quantitative improvement of the original twenty-row family

I read the complete retained `quantitative-proof/effective-step.md` and `minor-bound.md`, the exact constant checker and the final printed module. The printed proof summarizes the pivot and rounding estimates; their full analytic proof is in the companion. That companion is therefore part of the proof being approved and must accompany the PDF.

The important global issue is handled correctly: a derivative bound on exact base maximizers cannot simply be applied to a nearby moving maximizer. The proof first repairs its closure and then quantitatively rounds the repaired base weights to an exact optimizer in a chart that preserves a positive pivot.

The uniform pivot argument includes the retained old rows with velocity zero. Perturbed closure gives residual `-t sum beta D`, of norm at most `(5/4)t`; it does not incorrectly factor a common nonzero velocity out of all weights. Positive action bounds both factor masses. A normalized pentagon distribution with this small residual has three directions of quantitatively positive mass; choosing one of the at-most-two occurrences in each direction gives the `1/4500` bound. Two distinct q-directions and three distinct p-directions give an augmented determinant greater than `1/2`. The cofactor/Hadamard estimate gives spectral inverse norm below 66 and infinity norm below 200. These estimates are uniform in the order and support of the simple moving-row maximizing word.

Repairing through that pivot changes its weights by at most `400t`, leaving them above `10^-4`. The Neumann inverse estimate and differentiated closure give the written first- and second-derivative norm bounds. Differentiating the literal quadratic objective gives Taylor remainder `4*10^9 t^2`, derivative magnitude at most `44M`, and derivative Lipschitz constant `88M=17600`. No positivity of the chart on the larger auxiliary Taylor interval is assumed or needed for these norm bounds. The chart is feasible for the needed sufficiently small continuation through an exact positive-pivot optimizer.

The gap rounding is also valid on repeated occurrences. A factor closure vertex uses three distinct base directions and one occurrence of each, giving at most `5*2^3=40` possibilities. At balanced factor mass each pair is one of the primitive six-orders, so the checked positive gap applies. The identity

`d = 4 q0 (s-1/2)^2 + 4s(1-s)(q0-S)`

bounds both imbalance and bad-pair mass. Deleting marginal vertices of mass below `2 sqrt(epsilon)` leaves nonempty supports at the proved deficit scale; any remaining bad pair would have mass at least `4 epsilon`, impossible. Renormalization changes each marginal by at most `160 sqrt(epsilon)`. The resulting exact optimizer is within `2000 sqrt(d)` in occurrence-weight L1 norm and retains the chosen pivot. Thus its derivative really is covered by the existing all-word response theorem.

Completing the square in the loss and derivative error gives an upper bound for the **true global optimal action**:

`q(t) <= q0 + (2q0/125)t + 10^15 t^2` for `0<t<=10^-30`.

This is the correct direction for a capacity lower bound. The volume estimate likewise has the correct direction: on each old facet the moving gauge is at least `1+t f_i`, and other inequalities can only increase it. Radial integration and `0<=f_i<=2/5` give `V(t)/V0 <= 1-4 nu[f]t+(8/5)t^2`. Multiplying the positive action/volume upper bounds, retaining the quadratic costs and discarding only negative cross terms, gives

`q(t)^2 V(t)/(q0^2 V0) < 1-t/200`.

Taking reciprocals yields the printed strict relative gain. The constants leave ample margin: `A^2/4+R=309764000000000<10^15`, the linear gain exceeds `1/100`, and `4*10^16 t<1/200` on the stated interval.

The minimum primitive action gap remains a finite exact computational input. I inspected the checker and the recorded successful replay, rather than claiming a new full primitive/catalogue replay. The independent concentrated-witness calculation above is new execution in this review.

## Final scope reconciliation

The final paper now proves more than r2: an explicit improving parameter `t=10^-30` for the original all-ten-unit-cut twenty-row family, with relative gain greater than `5*10^-33`. The earlier r2 sentence saying no explicit parameter was certified is superseded by the reviewed r3 proof. The seventeen-facet subfamilies still have an existence/positive-derivative result; **the `t/200` bound is not proved for them**, and the paper/package keep that distinction.

All other approved r2 conclusions remain intact: sharp rigid full-affine HKO; non-Lagrangian triangle position beating every Lagrangian position; the attributed four-simplex bound with its equality family; the exact computer-assisted sharp six-simplex inequality and unique equality class; the all-dimensional two-paraboloid supremum and general planar squared-body constant; the correctly normalized right-quaternionic-I consequence without sharpness claim; and strict all-partner Viterbo for genuine truncated parallelograms with an open neighborhood, while the proposed optimizer fails.

Root owns the remaining visual PDF review, archive/manifest assembly, retained report integration, commits and delivery. Approval applies to the exact hashes above and the linked r2 review; a later substantive mathematical change needs a new delta review.
