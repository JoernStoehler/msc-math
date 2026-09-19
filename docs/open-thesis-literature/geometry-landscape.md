# Capacity landscape and a structured geometric discovery family

Checked 18 September 2026. This note distinguishes theorem reading from leads; it does not establish novelty of project results.

## The global question after HKO

Haim–Kislev–Ostrover, *A counterexample to Viterbo's conjecture*, Annals 203(2), 603–622 (2026), [final publication page](https://annals.math.princeton.edu/2026/203-2/p05), DOI 10.4007/annals.2026.203.2.5. The journal page verifies the final bibliography and counterexample scope. Full endpoint/proof examination is owned by the existing product proof notes. A thesis asking what remains true after the counterexample should distinguish particular capacities, symmetry assumptions, geometric families, and local versus global optimization; “Viterbo is false” does not erase all these questions.

Abbondandolo–Edtmair–Kang, [*On closed characteristics of minimal action on a convex three-sphere*, arXiv:2412.01777v1](https://arxiv.org/html/2412.01777v1), Corollary 1: EHZ and cylindrical capacity coincide on every convex body in R4. The inherited literature-closure plan directly checked the corollary; this pass reopened the primary text. The stronger surface-of-section theorem's uniform convexity hypothesis must not be transferred onto this capacity corollary. This explains why the capacity computed in the thesis controls cylindrical size, while not identifying it with Gromov width. The inherited correction should be integrated rather than re-searched.

Gutt–Hutchings–Ramos, [*Examples around the strong Viterbo conjecture*, arXiv:2003.10854](https://arxiv.org/abs/2003.10854): abstract checked only in this pass. Monotone toric domains in dimension 4 have coincidence of normalized capacities; in arbitrary dimension the stated coincidence is Gromov width with the first equivariant capacity. This supplies a potential alternative representation/control family, not a theorem that arbitrary product polytopes are monotone toric. The products note examines the Toda bridge separately.

## A structured family beyond random generic polytopes and Lagrangian products

Berezovik, [*Symplectically self-polar polytopes of minimal capacity*](https://doi.org/10.1007/s40316-025-00251-0), Ann.Math.Québec 49, 335–353 (2025), version of record 23 September 2025; [arXiv:2310.14998](https://arxiv.org/abs/2310.14998). Directly read journal introduction, Theorems 1.1–1.2, 3.2, 4.1, Conjectures 4.5–4.6 and numerical conclusions in §5. No independent proof revalidation.

A symplectically self-polar body satisfies X=X^omega, where X^omega consists of y with omega(x,y)<=1 for all x in X. Such bodies are centrally symmetric. This is a distinct structured class; arbitrary sampled polytopes do not satisfy the defining condition.

The article constructs bodies with EHZ capacity 2+1/n in dimension 2n, sharp within the self-polar class. Its stated general centrally symmetric lower bound is c_EHZ(X)>=(2+1/n)c_J(X); omitting c_J would make the assertion false under scaling. The construction has explicit volume and affine cylindrical capacity. Theorem 4.1 characterizes the planar minimal-volume self-polar body up to linear symplectomorphism. The higher-dimensional volume minimum remains conjectural; the article combines random constructions and restricted discrete enumeration, making it unusually relevant to empirical geometry rather than merely a formal inequality source. Author code is linked in the article. This survey did not execute or audit that code.

**Concrete use:** a reproducible structured benchmark with known capacity, nontrivial volume questions and a symmetry-preserving generator. It offers different representation information from re-correlating scalar summaries of the same random sample. Do not infer counterexamples to symmetric Viterbo from its volume conjectures or promise computational coverage beyond what was enumerated.

## A new operation for adaptive family exploration

Berezovik, [*Contractibility of space of symplectically self-polar convex bodies*, arXiv:2608.13280v1](https://arxiv.org/html/2608.13280v1),13 August 2026. Read full short HTML paper, especially Theorem 1.1, Lemma 2.1 and proof. The theorem states contractibility in the Hausdorff topology. Its constructive mechanism is potentially more useful here than the topology alone:

- symplectic polarity commutes with linear symplectic reduction;
- take the symplectic l2 sum of two self-polar bodies in doubled dimension;
- reduce along a continuously rotating isotropic subspace to connect the endpoints while retaining self-polarity.

**Inference for research design:** this gives a mathematically grounded deformation operation rather than arbitrary coordinate perturbations, useful if the empirical team chooses self-polar geometry. It does not guarantee polygon/polytope intermediates: l2 sums can introduce curved boundaries. Representation, capacity evaluation, and any discretization error would therefore need a separate feasibility study. No proposal to expand the thesis scope automatically.

Related sources encountered but not checked beyond abstracts: [Berezovik–Karasev2211.14630](https://arxiv.org/abs/2211.14630), equivalence between a self-polar volume conjecture and symmetric Mahler; [Berezovik–Bialy2501.12165](https://arxiv.org/abs/2501.12165), invariant hypersurfaces of 4-periodic outer-billiard orbits. Outer billiards must not be confused with the Minkowski billiards computing product capacity.

## New survey resource and access boundary

Haim–Kislev, *On symplectic measurements of convex domains*, Current Developments in Mathematics 2025, 37–87, published May 2026. [Publisher metadata/abstract](https://intlpress.com/ChapterDetail?bid=2050294586091163649&id=2050294587966017538) checked; author's [public page](https://www.pazithaimkislev.com/) links a [public manuscript](https://drive.google.com/file/d/1jwjSiABmg1dZloyC11_dfKxbHzfrlXGd/view). Browser extraction of that Drive document returned a loading/sign-in shell, so contents were not read in this pass. The abstract promises computation techniques for convex domains and the post-counterexample landscape. This is a useful centralized orientation source if access is convenient; do not cite unseen theorem statements from it. Author-page “to appear” labels lag final journal publication, so prefer final publishers for metadata.

## Search record and reopening triggers

Queries: `Viterbo conjecture symplectic capacities convex bodies 2026 2025 survey counterexample`; `Viterbo conjecture computational polytopes symplectic capacity algorithm complexity`; `symplectically self polar polytopes minimal capacity Berezovik 2025 arxiv`; `On symplectic measurements of convex domains Haim Kislev pdf`. Followed primary author publication pages and references of the self-polar article. Search aggregators were discovery aids only; one aggregator omitted the necessary scale factor c_J, illustrating why source statements matter.

Reopen this strand if choosing self-polar datasets/operations, claiming symmetric Viterbo current status, translating a capacity comparison across dimensions, or using toric representation of a specific product. Not covered: complete Mahler literature, arbitrary symplectic embeddings, higher-capacity classification, exhaustive new-paper surveillance. No thesis claim requires these entire fields to be surveyed.

## Mean width: non-invariant features with a theoretical use

Ahn–Kerman, [*On the symplectic capacity and mean width of convex bodies*, arXiv:2602.07220v1](https://arxiv.org/html/2602.07220v1). Introduction and §2 statements read; §3 not fully audited. Header says submitted 6 February 2026 while rendered internal date says 24 August 2026; preserve the version URL rather than silently assigning one date to both.

Theorem 1.1 recalls the Artstein–Avidan–Ostrover upper bound c_EH(K)<=M(K)^2/4 in normalization c(B)=1; for the thesis normalization c(B)=pi this becomes pi M(K)^2/4. M averages the two-sided support function over unit directions with probability measure. The paper studies minimizing this non-invariant geometric quantity over symplectic images. Thus useful features need not themselves be symplectic invariants: a known bound or optimization role can supply meaning. The same paper recalls square-root superadditivity under Minkowski sum. Neither statement makes normalized ridge area an intrinsic invariant or validates a correlation.

**Concrete opportunity:** theoretically interpretable geometric upper bounds, optimized over linear symplectic maps, could complement exact capacity evaluation. Their numerical sharpness and computation cost in this dataset remain unknown. Start by comparing existing values to a correctly normalized bound, not by asserting a new theorem or launching a large feature sweep. Search trigger: choosing support-function/mean-width descriptors or Minkowski-addition operations.
