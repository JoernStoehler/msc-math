# Historical product families: theorem applicability and research use

Checked 2026-09-18. This is a bounded source-to-generator mapping, not a complete literature survey or an audit of every stored geometry. No thesis source, numerical target, or experiment was changed. Parent: `research/literature-closure-plan`, `f54cd3d9`.

**Result:** seven of the ten historical product buckets have a triangle or quadrilateral factor, hence satisfy the Viterbo inequality. They contain 7,168 of the 10,240 product rows. The other 3,072 product rows are not excluded by these checked side-count results. This does not imply that every remaining body is eligible, or that other results do not apply. Existing studies of all ten groups retain scientific value; above-one search and subthreshold enrichment require different interpretations.

## Mathematical contract

Let `P = K × Q` in coordinates `(q1,q2,p1,p2)`, with standard symplectic form, and define `sys(P)=c_EHZ(P)^2/(2 vol_4(P))`. Both planar factors must be compact convex bodies with interior; `vol_4(P)=area(K)area(Q)`.

Balitskiy–Mitrofanov–Polyanskii, [arXiv:2603.12495v1](https://arxiv.org/html/2603.12495v1), Theorem 1.2 establishes this inequality for any convex planar first factor and a quadrilateral or affinely regular hexagonal second factor. Section 4 proves the triangular case through the covering formulation of Theorem 3.3. Section 7.1, Theorem 7.2 supplies the regular-hexagon covering bound. These are preprint results, not described here as refereed publication.

The factor order does not matter for this deduction. The symplectic map `(q,p) -> (p,-q)` sends `K × Q` to `Q × (-K)`. Reflection preserves polygon type and area. Thus a qualifying factor in either position suffices. This is an applicability argument, not a new proof of the cited inequality.

## Actual generator and acceptance

The historical producer can be inspected reproducibly with:

```sh
git show ef1b0f6b:experiments/sys-landscape/datascience/produce/random-product.rs
git show ef1b0f6b:experiments/sys-landscape/src/sys_landscape_cache.rs
git show ef1b0f6b:crates/symplectic/src/geom/polygon.rs
```

The producer calls `random_polygon_2d(k,...)` and `random_polygon_2d(m,...)`, then `SysLandscapePolytopeCache::from_lagrangian_product`. That helper places q inequalities in coordinates 1–2 and p inequalities in coordinates 3–4, with exact zeros elsewhere. There is no ambient rotation between factor construction and evaluation/cache lookup. Dividing by positive heights and converting the resulting binary64 entries into exact rationals does not change this block separation.

`random_polygon_2d` samples independent uniform angles, sorts them, and samples independent positive support heights. Its docstring promises boundedness too early: the raw halfplane draw need not be bounded or irredundant. The actual cache constructor rejects unless the origin is in the interior of the dual hull and all dual points are extreme. Accepted draws therefore are bounded, full-dimensional products with the requested active side counts. No correction to that helper's shared docstring is made here because it is outside this packet's ownership.

The retained distribution has seed 42, heights `[0.8,1.2]`, and 1,024 accepted rows per pair. The source default alone does **not** establish the historical run size: the historical source default is ten, while retained counts establish 1,024. The accepted generator law, retained byte identities, and executed numerical backend must not be conflated. The last lacks a complete historical manifest. See [dataset README](../../experiments/polytope-datasets/README.md), [numerical lineage](../../experiments/polytope-datasets/retained-lineage.md), and the recovered [DS account](../ds-evidence-closure/README.md).

## Bucket mapping

“Excluded” means the mathematical body has `sys≤1` by the indicated argument, not that an old floating-point target is certified. Each bucket has 1,024 retained rows.

| Factor sides | Checked coverage | Disposition for above-one discovery |
|---|---|---|
| 3×3 | triangular factor | Excluded |
| 3×4 | triangular factor; also quadrilateral | Excluded |
| 3×5 | triangular factor | Excluded |
| 3×6 | triangular factor | Excluded |
| 4×4 | quadrilateral factor | Excluded |
| 4×5 | quadrilateral factor | Excluded |
| 4×6 | quadrilateral factor | Excluded |
| 5×5 | no matching checked factor hypothesis | Not excluded by this mapping |
| 5×6 | hexagon hypothesis requires additional geometry | Not excluded by side count; affine-regular hexagon subfamily excluded |
| 6×6 | hexagon hypothesis requires additional geometry | Not excluded by side count; either affine-regular factor suffices |

A random six-sided polygon is not defined to be affinely regular. Central symmetry alone is insufficient for affine regularity. For prospective exact checks, order vertices cyclically and test a common center `c` with `v[i+3]=2c-v[i]`; for `u_i=v_i-c`, also test `u_1=u_0+u_2`. With noncollinear `u_0,u_1`, these identities characterize an affine image of a regular hexagon. This elementary criterion follows by mapping the first two regular-hexagon radius vectors to `u_0,u_1`; the remaining four are determined by their linear relations. Near equality is not an exact theorem certificate. No row-level hexagon tests were executed in this pass.

## Which transformations preserve the deduction?

| Operation on an excluded body | Consequence |
|---|---|
| Translate either factor | Product geometry and the inequality are preserved. |
| Apply separate invertible planar maps `A` and `B` to the factors | Still a coordinate Lagrangian product; triangle/quadrilateral/affine-regular type is preserved. Reapply the theorem. The capacity need not equal its original value. |
| Apply `diag(A,A^{-T})` | Symplectic; preserves capacity and volume as well as product structure. |
| Scale factors independently by positive `a,b` | Preserves ratio: common dilation by `sqrt(ab)` composed with reciprocal symplectic scaling. |
| Apply any exact symplectic linear map, including U(2) | May lose coordinate-product form but preserves the ratio and therefore the exclusion. Product-backend eligibility is a separate question. |
| Apply general SO(4) rotation | Euclidean volume is preserved but symplectic capacity need not be. The image need not satisfy the product hypotheses. Original factor labels alone no longer certify exclusion. |
| Round transformed coordinates into new binary64 geometry | Inspect the body actually evaluated. Structural zero blocks can preserve exact product hypotheses; a nearly symplectic dense matrix does not certify exact invariance. |

The newer [orientation allocation study](../../experiments/sys-datascience/methods/orientation-allocation/README.md) includes general ambient rotations and U(2) controls. Its rotated bodies must not inherit a blanket triangle/quadrilateral exclusion from their seeds. Conversely, a U(2) control cannot become an above-one mathematical candidate just because the implementation dispatches it to a general solver. Some newer packets are retained only in main; consult the [DS current-panel audit](../ds-evidence-closure/current-panels.md) before resolving missing worktree paths.

## Consequences for discovery and interpretation

1. **Separate the scientific objectives.** A selector can improve capacity ratio substantially inside a theorem-excluded family, discover near-equality geometry, or identify a useful predictor. Such outcomes are meaningful even though they cannot yield a counterexample there. Keep these studies and state their target.
2. **Use exclusions as controls.** An above-one numerical report in an excluded family should trigger geometry/provenance/evaluator checks. A below-one report alone does not validate the evaluator. Near-equality families are useful stress cases rather than discarded search space.
3. **Record theorem eligibility before target evaluation.** Future counterexample search should stratify candidate and evaluated counts by verified exclusion, unresolved applicability, and unexcluded-by-checked-results. Preserve generic/product family and transformations separately. Do not call this a complete decidable eligibility oracle.
4. **Measure discovery exposure honestly.** Only 3,072 of the historical product rows avoid these structural exclusions. Including the 4,096 generic rows gives 7,168 rows not excluded by this product-only mapping. These arithmetic counts are not estimates of independent draws, probability mass, or unseen counterexample prevalence. Neither pooled zero-hit counts nor subthreshold selection lift directly estimates counterexample-search performance in the remaining families.
5. **Distinguish preserving type from leaving it.** Factor rotations and shape optimization retaining a triangle/quadrilateral factor remain excluded. General ambient rotations or topology-changing factor edits may leave that class. This distinction changes which adaptive experiments can address above-one discovery.

## Explicit remaining work

- Row-level source-geometry verification against producer labels, including any exact affine-regular hexagon checks, was not performed. The source payloads were not re-downloaded. The mapping is to the inspected generator and documented retained groups.
- No theorem-class inventory across every later proposer/optimizer was attempted. Start with actual candidate geometry plus transformation records; initial bucket names are insufficient.
- No comprehensive literature exclusion search for the remaining hexagon/pentagon subfamilies was performed. The same primary paper treats additional conjectural cases; a conjecture is not an exclusion certificate.
- A prospective study should choose its objective first: counterexample discovery, equality/structure, or predictive enrichment. Then use the appropriate strata and controls. This packet does not recommend removing seven groups from all DS analyses.

Validation: direct inspection of historical generator, its acceptance checks and representation helper; comparison with present geometry constructors; primary theorem and triangular/hexagonal sections read; ten-bucket arithmetic checked. No expensive computation or production restart.
