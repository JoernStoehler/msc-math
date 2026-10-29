# Assembled mathematical review — r4 finite-parameter delta

Reviewer `/root/assembled_math_review`, 29 October 2026, 00:52–00:57 UTC. Read-only review; no new agents or canonical edits.

**PASS for the new finite-parameter theorem and its integration into r4**, together with the earlier full r2 review and r3 delta. No mathematical blocker was found. The near-zero proof still uses its complete quantitative companion in this revision; root has separately requested that its analytic details be printed in r5. This report does not approve an unseen r5 conversion.

Exact reviewed objects:

- 50-page PDF: `ea6c200b3969453a04020b53cf9ceb54beae4ccfcb08c3c226dd7dc9c2795770`.
- `main.tex`: `a8a33b04b55e7a07323af2e47ab47686e8f871f66a4f166bcc842780ad2f9a83`.
- `sections/hko-moderate.tex`: `deb39b76491c1b15833f62d02bb1963ad300a4897df3981845e59af1a88db489`.
- `sections/hko-effective-step.tex`: `e52910504635cc64c4864b0c69186303e593e4b1ef3b76edb772f78acd48ab99`.
- `certificates/hko-t03/MANIFEST.sha256`: `238ec061eb3750f12cf630a89fe936a42ab16ab5279a6bf690b9207654a8bc0e`.

The initial assignment gave an earlier PDF hash; root and the editor confirmed the above as the frozen final r4. `reviewed-r4/` preserves the actual source, PDF/extracted text and full finite packet. `reviewed-r4-hashes.json` pins every copied file. I checked the packet's manifest with `sha256sum -c`; all entries matched.

## What was independently reviewed

I read the complete r3-to-r4 source diff, the actual new theorem, the full coverage proof, all six acceptance programs, the retained independent mathematical and fresh-execution reports, and the assembled PDF text at the new section and relocated Appendix B. I did not rerun the multi-minute curvature/leaf producers; their completed independent replay is explicit evidence, not a new execution claimed by this review.

I directly retrieved Haim–Kislev's [primary arXiv v3 text](https://arxiv.org/html/1712.03494), especially Theorem 1.5 and the characteristic/action definitions. It supplies a minimum-action characteristic with each facet-normal velocity used in at most one segment, for arbitrary convex polytopes with interior. I also retrieved Lawrence's [primary NIST report](https://nvlpubs.nist.gov/nistpubs/Legacy/IR/nistir89-4123.pdf), §2, and visually inspected the retained image of printed page 4 to read the numerator omitted by OCR. The signed volume expression and its hypotheses agree with the code.

`check-moderate-witness.py` and `moderate-witness.out` provide a new independent exact check using my prior standard-library field implementation, with no imports from the submitted implementation. This checks the literal printed weights and normalization identities, not the complete global upper-bound computation.

## From a physical minimizer to complete finite coverage

The graph restriction is justified geometrically, rather than by assuming every algebraic maximizing word can be reconstructed with a full-rank constraint minor. A segment with velocity proportional to the compatible complex structure applied to facet normal `n_i` lies on that facet: the characteristic condition places the normal in the normal cone at the point, and its supporting hyperplane is the facet's hyperplane. Consecutive facets therefore meet. In the paper's convention the arriving velocity is `-J n_i`, and the derivative of the next support functional is `n_j·(-J n_i)=n_i^T J n_j>=0`. Zero-pairing transitions are retained.

Complete exact four-row enumeration supplies every vertex. The origin is strictly feasible and the old bounded H contains the new body. Four independent active constraints at each retained point prove simplicity. Since every row occurs among these active sets, each is a genuine facet. Pairwise facet intersections are exhaustively determined because every nonempty compact face contains a vertex.

The prefix obstruction is valid even if the prefix itself has no positive closure weights. Its direction lies in the closure/normalization kernel; zero-padding it into a longer word preserves both constraints and the positive quadratic curvature. Cyclic rotation of such a direction preserves its quadratic action by closure. The exact symmetry check acts on all twenty normals and verifies the sign of every pairing; reflection is coupled with word reversal. Thus canonicalization preserves the obstruction and the scalar optimization problem.

The C++ traversal has no independent maximum-length cutoff. It visits simple directed cycles from their least labels, stops only at an exactly certified curvature obstruction, and checks every surviving closed word against the leaf list. The curvature congruence test is sound: a positive diagonal suffices, negative pivots can be eliminated by exact Schur complements, and a nonzero off-diagonal with all diagonals zero supplies an indefinite two-dimensional restriction. Hence a positive-weight global maximizer cannot be pruned. The reported zero-unknown-cycle result closes the enumeration obligation at the supplied graph and word lists.

## Stationary families and boundary cases

Deleting zero-length segments from the geometric minimizer leaves a positive-support geometric cycle. This is the support at which equality-constrained stationarity is necessary. There is no need to assert that every arbitrary subword of every algebraic leaf is realizable in the graph.

The implementation first checks full constraint consistency, then retains independent equality rows and the matching right-hand-side entries. Its KKT signs agree with `Q=beta^T G beta/2`. For a consistent singular family, the kernel equations give `e·eta=0`, so its objective is constant. A negative particular solution is not used as an infeasibility test.

The 984 families at or below the attained target cannot raise the maximum. The 21 families above the independent inclusion bound are impossible: at `t=3/10`, each moving row has H-support `28/25`, so `(25/28)H` lies in K and capacity monotonicity bounds the global action by `(tau/5)(28/25)^2`. For each remaining family, the full-row-rank constraint matrix makes the KKT-kernel projection to weights injective. Feasible parameters therefore form a bounded polyhedron because the weights lie in a compact simplex. If nonempty, it has a vertex cut out by independent active zero-weight equations, including when the feasible polyhedron is lower-dimensional. The complete basic-vertex search is consequently a valid emptiness test. All 25 remaining above-target families are rejected by that exact test.

The explicit positive word supplies the lower action bound independently of the leaf program's initialized best value. My separate calculation verifies its printed seven weights close the actual twenty-row `t=3/10` normals, sum to one, and give `Q=205 tau/1019`. Their positivity follows from `0<tau^2<1`. This closes the distinction between a feasible capacity upper bound and the equality claimed by the theorem.

## Volume, constants, and reader-facing scope

For active normals as rows, Lawrence's coefficients are `ell N_I^{-1}`. The determinant is taken in absolute value, but the product of those coefficients retains its sign. The two generic functionals have every incident edge slope nonzero, satisfying the theorem's hypothesis. Agreement of the two evaluations is an arithmetic cross-check; the primary theorem supplies the mathematical identity.

I independently checked the literal radical conversion of the reported field volume, `c=1019/(410 tau)`, the relative factor `1019/1025`, and the exact displayed positive difference from HKO. The results support the printed ratio `1.0519141388294875...`. No decimal rounding is used to decide the strict inequality.

The new abstract and roadmap explicitly separate the finite `t=3/10` computation from arbitrarily near ascent. They do not infer the finite improvement from a positive derivative, transfer its gain to the seventeen-facet families, claim monotonicity on `[0,3/10]`, or claim optimality along the family. The weighted seventeen-facet statement is unchanged. Appendix B retains the former tiny-step proposition and all its definitions; the main reference resolves to the relocated proposition, with its original all-ten-unit-cut qualification. No unproved arbitrary-two-hinge claim appears.

The exact finite packet is essential to this computer-assisted theorem. Its older frozen README/proof status sentences describe the producer's pre-audit state; the retained independent reports supply the completed approval and replay evidence. I have not silently interpreted producer success as independent acceptance.

The r4 verdict covers the mathematical argument with its specified companions. It does not imply external refereeing, exhaustive novelty, human acceptance, or approval of later source changes. Root owns final visual review, r5 conversion, retained report integration, archive assembly and delivery.
