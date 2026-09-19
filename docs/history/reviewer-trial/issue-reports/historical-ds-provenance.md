# Repair retained DS provenance and schema before expanding writer caveats

Prepared 2026-09-18 for main; bounded source audit, no dataset mutation or capacity computation.

**Problem.** The 4,096 generic + 10,240 product baseline has recoverable historical results, but writers cannot connect it to the current evaluator contract. Separately, the registered invariant snapshot lacks six later ridge columns and fails the current analyzer. Fix these upstream as distinct tasks: importing a schema cannot establish missing execution history.

## Owners and evidence

All paths below are repository-relative unless absolute.

| Responsibility | Exact owner / observed state |
| --- | --- |
| Immutable source geometry and targets | `experiments/polytope-datasets/{random.jsonl,random-product.jsonl,shared-cache.jsonl}`, registered in `artifacts/registry.json` as `polytope-datasets`, snapshot `f7bc6be841e9d30741d5bf7ec8d4f0c0c74ec22745d3325b792e64cec333ca96`. Source schemas in `rows.rs` retain binary64 dual vertices, rational dual/primal vertices, targets, bucket and sampling fields. Payload contents were not retrieved in this audit. |
| Derived table and joins | `experiments/polytope-invariant-table/{load_caches.rs,prepare.rs,rows.rs,features_face_symplectic.rs}` and tracked `polytope-provenance-table.jsonl`. Loader copies targets and assigns backend labels; it does not attest the executed evaluator. |
| Current evaluator/cache contract | `experiments/sys-landscape/src/datascience_cache.rs`: `ComputedPolytopeCache::compute` only reuses current-method rows; legacy rows invoke `qp_minimizers`, compare old capacity with new bounds, and emit exact capacity, bounds, candidate family and tied words. |
| Downstream consumer | `experiments/sys-datascience/methods/statistical-associations/analyze.py` and shared feature contract. Historical analyzer at `ea8e2657` reproduces the original 39-feature screen; current contract has 45 eligible features. |
| Existing recovery receipt | `/workspaces/msc-math/.worktrees/ds-evidence-closure/docs/ds-evidence-closure/{README.md,replay-receipt.json,replay-historical.sh}`. Registered table SHA256 `607c8731fa03d190d497edc3e8f1b4cca88f7d238260cce527680f568bc33d59`; headline association reproduced, nonnumeric leaves identical, maximum numeric discrepancy below `6.83e-13`. |

`experiments/polytope-datasets/retained-lineage.md` records source byte identities and history: June payloads precede the July 26 certified producer migration (`75daa6f4`). Historical producer source at `ef1b0f6b:experiments/sys-landscape/datascience/produce/random.rs` permits cache reuse. A source revision therefore cannot by itself establish which route produced every target.

## Recoverable versus missing

- **Recovered:** invariant-table bytes and original statistical analysis. The standard host cache contains invariant snapshot `c3042846723dbb89db17f2f39d88004d937e1e790949ff6880188fa89fc1900a`; this audit inspected its manifest. The cheap retained-lineage guard passes on main for all 14,336 provenance rows.
- **Identified, not yet recovered here:** source datasets and shared cache. Their source snapshot is absent from the standard host cache. The registry supplies a retrieval target; this is not evidence that geometries are lost. Actual row-level geometry consistency and the shared cache's witness coverage remain unverified.
- **Not established by retained evidence:** per-run source/build state, clean/dirty status, exact backend execution and cache freshness. Do not reconstruct these as asserted facts from filenames, backend labels or timestamps. A new import can record its own source hashes and revision, not manufacture an old execution receipt.
- **Repairable schema gap:** six missing ridge descriptors can be computed from retrieved geometry using current descriptor code, retaining old targets explicitly as historical. This requires geometry/feature work, not capacity evaluation. Existing aggregate columns alone have not been shown sufficient to derive them. Full feature reconstruction has a known cost concern; first test a small geometry-only slice.

## Cheapest plausible repair

1. **Recover and inventory before rerunning.** Materialize `polytope-datasets` with `scripts/artifacts.py materialize polytope-datasets --no-link`, then verify the manifest, both dataset hashes against `retained-lineage.md`, row counts, and joins by the repository's `poly_id_from_dual_vertices` algorithm. Inventory `shared-cache.jsonl` fields and matched coverage without calling an evaluator. Report missing/duplicate IDs, geometry inconsistencies, available orbit/candidate witnesses and any genuinely current receipts. Preserve immutable originals.
2. **Add an honest import/feature migration.** Produce a new derivative with an import receipt binding source snapshots, row IDs, migration revision and checks. Carry unknown historical evaluator fields as unknown; retain legacy targets alongside any subsequently validated values. Validate binary64/rational geometry correspondence before claiming identical bodies: current capacity is evaluated on exact binary64 geometry. Fill only missing features if the dependency audit supports it; compare overlapping features and joins on a small slice before a full descriptor rebuild. This restores modern analysis without pretending to certify capacities.
3. **Revalidate only what the intended statement needs.** Check first whether recovered caches contain trusted matching current certificates or enough independent completeness evidence to avoid new minimization. Legacy orbit witnesses or stored winning words alone do not prove that no smaller candidate was missed. The inspected cache loader's `current_capacity_payload_is_valid` is a structural check, not a mathematical certificate verifier. No demonstrated no-recalculation certification route was found in this audit.
4. **If that evidence is absent, use a frozen stratified reevaluation pilot on existing geometries.** Include generic facet strata and product buckets, predeclare selection and comparisons, record evaluator revision, exact geometry identity, bounds, family and discrepancy outcomes. Do not regenerate a fresh population to answer legacy-target accuracy. The current cache panics on an incompatible old scalar; a comparison harness must preserve discrepancies as findings rather than aborting the entire audit or silently replacing targets. Measure representative cases before proposing wider compute; no runtime estimate is justified yet.

## Acceptance and next bounded step

The next assignment should stop after source retrieval, integrity/join/field inventory and a code-level import/descriptor dependency check: **zero capacity calls and no whole-table rebuild**. Deliver a concrete migration design and evidence table showing whether any existing records avoid minimization. If retrieval fails, report the exact artifact-access failure rather than replacing this task with new sampling.

Completion should yield a reproducible migrated derivative and/or a named validation panel with a machine-readable comparison receipt; update the owning READMEs and consumers to use those artifacts. A full-baseline current-capacity claim requires coverage of all relevant rows, not a small panel. A panel supports only its bounded comparison; reevaluation of old geometries is not independent population replication. Certified capacity also does not make `sys` a certified ratio while volume remains binary64. Neither an import nor winner-only reevaluation validates historical optimizer trajectories or rankings.

**Recommendation:** undertake the bounded recovery/inventory now, then select the smallest repair supported by actual payloads. This can remove the schema obstacle and resolve whether capacity reruns are necessary before asking the writer to absorb more caveat text. Overall DS scope and fresh research remain separate decisions.
