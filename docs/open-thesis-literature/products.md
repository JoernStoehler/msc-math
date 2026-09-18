# Polygon products, billiards and covering problems

Checked 18 September 2026. This is an open discovery pass seeded by the project's pentagon results, not a novelty certificate or a second empirical campaign. Primary statements were read at the depths recorded below. Source algorithms have not been implemented or compared experimentally here.

## Consequential findings

- Product solver literature is richer than a citation to the three-bounce theorem suggests: Krupp–Rudolf explicitly treat **two polygonal factors** and fixed-face action constancy. Check this before describing finite product enumeration or constant branch actions as new.
- The covering literature contains an actual **experimental optimization → conjecture** predecessor for truncated-square pentagons, plus unresolved nonregular centrally symmetric hexagons. These offer scientific comparators beyond another random-product dataset.
- The project's independently linear-deformed regular-pentagon theorem does not settle arbitrary polygon partners. Current empirical ownership of that broader class remains appropriate.
- Symplectic equivalence to a ball, equality of capacities, and EHZ systolic equality are distinct outcomes. Neighboring product papers supply examples where the first two can be investigated, not just the last.

## Source map

### BMP: normal triangles and unresolved product geometry

Balitskiy–Mitrofanov–Polyanskii, [arXiv:2603.12495v1](https://arxiv.org/html/2603.12495v1), 12 March 2026. Read introduction, §3 statement/context, §§6–7 and reference list; no full proof audit.

Theorem 1.2 covers arbitrary quadrilateral and affinely regular hexagon factors paired with arbitrary planar convex bodies. Theorem 3.3 reduces capacity thresholds to covering normal triangles. Section 6.1 gives the regular-pentagon HKO cover and explicitly leaves its global minimum-area property unproved. Section 6.2 reports experimental optimization of truncated-square pentagons, leading to Conjecture 6.1: a specified trapezoid partner is optimal and the optimum is strictly below one. Section 7.2 leaves nonregular centrally symmetric hexagon covers unresolved, with examples suggesting four isolated minimizers. Question 1.3 asks for the optimal relaxed product constant; their bound is strictly below sqrt(2), while HKO supplies the lower bound.

**Use:** separate proven exclusions, proposed optimizers and actual search outcomes; offer a predecessor for the empirical methodology. **Next use:** read the full normal-triangle proof when relying on a capacity implementation. Existing P5-partner owner already handles that contract.

### Krupp–Rudolf: an algorithm for polygonal factors already exists

[Shortest Minkowski billiard trajectories on convex bodies, arXiv:2203.01802v1](https://arxiv.org/html/2203.01802v1), submitted 3 March 2022. Read §7.1 and §7.3, including Proposition 7.1 proof. The abstract history lists v1; the HTML's dynamically rendered date is not a new submission date.

Section 7.1 starts with polygonal K and smooth strictly convex T, constructing candidate three-bounce trajectories from facet triples and fitting a dual triangle. Section 7.3 explicitly switches to two polytopes. It permits boundary hits and explains selecting neighboring facet-normal rays, with a reference to Krupp's dissertation and a smoothing justification. Proposition 7.1 proves that admissible pairs of trajectories sharing the same ordered relative-interior face assignments in both factors have equal length. It then develops finite face-based computation.

**Use:** related work for product computation and branch structure. **Boundary:** this is not automatically the same algorithm, numerical guarantee, or complexity as the project's QP implementation. Any novelty comparison needs the actual implementation contract, not title matching. The cited dissertation §4.3.2 is the next source if the nonsmooth reduction becomes consequential.

### Rudolf: cover optimization is a pre-existing reformulation

[Viterbo's conjecture as a worm problem](https://link.springer.com/article/10.1007/s00605-022-01806-x), Monatshefte für Mathematik 201 (2023), 217–287. Read Theorem 1.1, Corollary 1.2, Theorem 1.4 and introductory definitions; algorithmic sections not audited.

Theorem 1.1 translates the product inequality to a minimum-volume Minkowski worm-cover problem. Corollary 1.2 expresses the optimization through the convex hull of individually translated curves. Theorem 1.4 relates fixed-volume capacity optimization, shortest strong billiards, nontranslatable polygonal curves and cover minimization. The extra equivalence using weak billiards requires strict convexity of the norm body.

**Use:** mathematical motivation for switching representations from polytopes to covers. **Boundary:** the paper predates HKO; equivalences remain useful even though the unrestricted inequality is false. Do not inherit its historical conjecture status. Do not silently substitute weak billiards in nonsmooth code.

### Rudolf: product equality versus Zoll dynamics

[Viterbo's conjecture for Lagrangian products in R4 and symplectomorphisms to the Euclidean ball, arXiv:2203.02294v5](https://arxiv.org/html/2203.02294v5), 21 September 2022. Read abstract, Theorem 1.1 and introduction; §8 located, not fully audited.

Theorem 1.1 proves the inequality for a trapezoid factor and arbitrary convex partner. The paper studies equality cases and symplectomorphisms to balls, and distinguishes full Zoll behavior from a weaker almost-everywhere minimal-period property away from lower-dimensional faces.

**Use:** avoid treating EHZ equality as evidence that every generalized characteristic is minimal. Its quadrilateral question is superseded by BMP; its finer equality/dynamics questions are not thereby erased. Check §8's exact definitions before using this as a theorem contract for project flow results.

### Toda lattice: a different route to structured product examples

Ostrover–Ramos–Sepe, [From Lagrangian products to toric domains via the Toda lattice](https://doi.org/10.1112/S0010437X24007590), Compositio Mathematica 161(2) (2025), 365–384; [arXiv:2309.10912](https://arxiv.org/abs/2309.10912). Read publisher abstract and bibliographic record only.

Certain simplex/Voronoi-cell products are symplectomorphic to balls; the planar instance pairs an equilateral triangle with a regular hexagon. The method uses periodic Toda dynamics to obtain toric descriptions.

**Use:** a structured control and an alternative mathematical outcome beyond finding high ratios. **Next use:** recover the exact coordinate/scaling hypotheses before generating a benchmark. Abstract-level reading does not certify an arbitrary triangle–hexagon orientation.

### Vicente: functional duality differs from polarity

[The strong Viterbo conjecture and various flavours of duality in Lagrangian products, arXiv:2505.07572v2](https://arxiv.org/html/2505.07572v2). Read definitions and Theorems 1.3, 1.5, Corollary 1.5.1; not full proofs. [Author publication page](https://sites.google.com/view/alejandro-vicente/research/) lists acceptance in Israel Journal of Mathematics.

For Luxemburg balls defined by a tuple of Young functions, Theorem 1.3 computes every normalized capacity of the product with the Legendre-functional-dual ball. Theorem 1.5 instead bounds capacities of the polar-dual product between twice the minimum inverse-function product and four. Corollary 1.5.1 supplies an additional derivative condition yielding equality to four.

**Use:** feature/generator ideas involving duality with known answers. **Boundary:** functional dual and polar dual are different partners. These centrally symmetric structured bodies do not subsume the pentagon affine theorem. No need to add smooth-body production merely because this source exists.

### New dynamics connection, with a smoothness boundary

Albers–Chavez Caliz–Tabachnikov, [Symplectic billiards as Minkowski billiards, arXiv:2607.05986v1](https://arxiv.org/html/2607.05986v1). Read Theorems 1–3, §3.3 and hypotheses, not full orbit-count proof.

Theorem 2 describes the invariant symplectic structure of the Minkowski billiard map. Theorem 3 identifies the symplectic billiard map as a square root when the two identified hypersurfaces coincide, under strict convexity and C1 smoothness. Theorem 1 gives at least n two-periodic orbits. Section 3.3 relates integrability and Radon/constant-width phenomena.

**Use:** adjacent conceptual material for dynamics or paired-factor constructions. **Boundary:** it supplies neither a polygon-capacity formula nor a nonsmooth orbit classification. Applying it to polygons would require an additional argument.

### Covering under additional motions is a different problem

Jung–Yoon–Ahn–Tokuyama, [Universal convex covering problems under translation and discrete rotations, arXiv:2211.14807](https://arxiv.org/abs/2211.14807); Advances in Geometry 23(4) (2023), 481–500. Read abstract and primary author-hosted paper introduction/Theorem 2 context. The allowed transformation group includes rotations, unlike the translation-only capacity-cover problem. Useful as a source of geometric proof techniques, not as a directly transferable bound. The [author-hosted affine-dihedral continuation](https://algo.postech.ac.kr/~heekap/Papers/jyat23uccpadga.pdf) was inspected at abstract level only.

## Relation to maintained project results

Canonical source read: `/workspaces/msc-math/.worktrees/pentagon-affine-integration/formal/pentagon-affine-products/README.md` and its source/status description. This pass found no direct published statement matching the independent-linear-pentagon formula or its equality characterization. That is a negative search result, **not evidence sufficient for a first-result claim**. Keep the rotation proof, original CAS provenance, affine extension and local ten-facet theorem distinct as already agreed.

The more productive literature questions are currently: how does our finite solver differ from §7.3 above; which representation gives a usable proof of cover optimality; and which geometric hypotheses make short-orbit candidates sufficient? These can be answered without claiming priority for the entire project.

## Search record and reopening triggers

Queries: `Lagrangian products polygons Viterbo conjecture billiards Q covers 2603.12495`; `Rudolf Minkowski billiards worm problem shortest trajectories symplectic capacity polygons algorithm`; `regular polygons Lagrangian products symplectic capacity pentagon heptagon affine Viterbo`; exact titles for the duality and Toda papers; `"regular" "pentagons" "affine" "capacity" symplectic`; `"Viterbo" "heptagon" capacity`; `"Universal convex covering problems" translations discrete rotations`. Followed BMP references and primary author publication pages. General search results included irrelevant polygon-moduli and architecture hits; they were not evidence.

Reopen when a concrete claim requires priority, a solver is compared for complexity/certification, a smooth billiard theorem is applied to nonsmooth bodies, a new partner class is sampled, or a claimed inequality exceeds the checked scope. Do not repeat discovery searches merely to write background from this map. All literature claims above remain attributed; none authorizes new empirical production.
